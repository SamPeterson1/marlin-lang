use std::{collections::HashMap, rc::Rc};

use item::{FunctionItem, Item, StructItem};

use crate::{environment::{Literal, ParsedFunctionType, ParsedPointerType, ParsedType}, error::{Diagnostic, DiagnosticType}, token::{Position, PositionRange, Token, TokenType, TokenValue}};
pub use self::expr::*;

pub mod expr;
pub mod item;

pub struct ExprParser<'a> {
    ptr: usize,
    tokens: &'a[Token],
    diagnostics: Vec<Diagnostic>,
    var_expr_id_counter: i32,
    declaration_expr_id_counter: i32,
}

pub struct ParseResult {
    pub items: Vec<Box<dyn Item>>,
    pub diagnostics: Vec<Diagnostic>
}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &[Token]) -> ExprParser {
        ExprParser {ptr: 0, tokens, diagnostics: Vec::new(), var_expr_id_counter: 0, declaration_expr_id_counter: 0}
    }

    fn err_expected_identifier(&self) -> Diagnostic {
        let prev = self.prev().unwrap();
        let msg = String::from("expected identifier");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)
    }

    fn err_expected_equals(&self) -> Diagnostic {
        let prev = self.prev().unwrap();
        let msg = String::from("expected equals sign");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)
    }

    fn err_expected_semicolon(&self) -> Diagnostic {
        let prev = self.prev().unwrap();
        let msg = String::from("expected semicolon");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)
    }

    fn err_expected_closing_parenthesis(&self) -> Diagnostic {
        let prev = self.prev().unwrap();
        let msg = String::from("expected closing parenthesis");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)
    }

    fn err_expected_parenthesis_after_for(&self) -> Diagnostic {
        let prev = self.cur().unwrap();
        let msg = String::from("expected parenthesis after for keyword");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)
    }
    
    fn err_unexpected_token(&self) -> Diagnostic {
        let prev = self.prev().unwrap();
        let msg = String::from("unexpected token");

        Diagnostic::new(1, DiagnosticType::Error, prev.position, msg)}

    pub fn parse(mut self) -> ParseResult {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            if let Some(expr) = self.item() {
                items.push(expr);
            }

            println!("{:?} {:?}", self.cur(), self.peek());
        }

        ParseResult {
            items,
            diagnostics: self.diagnostics
        }
    }

    fn try_match(&mut self, matches: &[TokenType]) -> bool {
        if self.ptr == self.tokens.len() {
            return false;
        }

        for token_match in matches {
            if self.tokens[self.ptr].token_type == *token_match {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) -> Token {
        let ret = &self.tokens[self.ptr];

        if self.ptr < self.tokens.len() {
            self.ptr += 1;
        }

        ret.clone()
    }

    fn is_at_end(&self) -> bool {
        self.ptr == self.tokens.len() || self.tokens[self.ptr].token_type == TokenType::EOF
    }

    fn peek(&self) -> Option<Token> {
        if self.ptr + 1 < self.tokens.len() {
            Some(self.tokens[self.ptr + 1].clone())
        } else {
            None
        }
    }

    fn cur(&self) -> Option<Token> {
        if self.ptr < self.tokens.len() {
            Some(self.tokens[self.ptr].clone())
        } else {
            None
        }
    }

    fn prev(&self) -> Option<Token> {
        if self.ptr >= 1 {
            Some(self.tokens[self.ptr - 1].clone())
        } else {
            None
        }
    }

    fn consume(&mut self, token: TokenType, diagnostic: Option<Diagnostic>) -> bool {
        let cur = self.cur();
        
        if cur.is_none() || cur.unwrap().token_type != token {
            if let Some(diagnostic) = diagnostic {
                self.diagnostics.push(diagnostic);
            }
            
            false
        } else {
            self.advance();
            true
        }
    }

    fn try_assignment(&mut self) -> Option<VarExpr> {
        let ptr = self.ptr;
        let var_expr = self.var();

        if var_expr.is_none() || !self.consume(TokenType::Assignment, None) {
            self.ptr = ptr;
            return None;
        }

        var_expr
    }

    #[allow(dead_code)]
    fn is_type(&mut self) -> bool {
        if self.cur().is_none() {
            return false;
        }

        match self.cur().unwrap().token_type {
            TokenType::Bool | TokenType::Int  | TokenType::Double | TokenType::String | TokenType::Identifier => true,
            _ => false
        }
    }

    //any primitive type
    //(type, type, ...) -> type
    fn try_type(&mut self) -> Option<ParsedType> {
        println!("Trying type");
        let cur = self.cur()?;

        match (cur.token_type, cur.value) {
            (TokenType::Int, None) => {self.advance(); Some(ParsedType::Integer)},
            (TokenType::Double, None) => {self.advance(); Some(ParsedType::Double)},
            (TokenType::Bool, None) => {self.advance(); Some(ParsedType::Boolean)},
            (TokenType::String, None) => {self.advance(); Some(ParsedType::String)},
            (TokenType::Identifier, Some(TokenValue::String(type_name))) => {self.advance(); Some(ParsedType::TypeName(type_name))},
            (TokenType::Star, None) => {
                self.advance();
                println!("Parsing pointer type");
                let pointee = self.try_type()?;
                Some(ParsedType::Pointer(ParsedPointerType {pointee: Rc::new(pointee)}))
            },
            _ => None
        }    
    }

    fn item(&mut self) -> Option<Box<dyn Item>> {
        let cur = self.cur()?;

        match cur.token_type {
            TokenType::Struct => self.struct_item(),
            TokenType::Fn => self.function_item(),
            _ => None
        }
    }

    fn struct_item(&mut self) -> Option<Box<dyn Item>> {
        self.advance();

        let mut cur = self.cur()?;
        let mut struct_name = None;

        if let (TokenType::Identifier, Some(TokenValue::String(name))) = (cur.token_type, cur.value) {
            struct_name = Some(name);
            self.advance();
        }

        if !self.consume(TokenType::LeftCurly, None) {
            return None;
        }

        let mut members = HashMap::new();

        cur = self.cur()?;

        while cur.token_type != TokenType::RightCurly {
            let member_type = self.try_type()?;
            cur = self.cur()?;

            let mut member_name = None;

            if let (TokenType::Identifier, Some(TokenValue::String(name))) = (cur.token_type, cur.value) {
                member_name = Some(name);
                self.advance();
            }

            members.insert(member_name?, member_type);

            self.consume(TokenType::Semicolon, None);

            cur = self.cur()?;
        }

        if !self.consume(TokenType::RightCurly, None) {
            return None
        }

        Some(StructItem::new(struct_name?, members, PositionRange::new(Position::new(0, 0))))
    }

    //fn type identifier(type identifier, type identifier, ...) block
    fn function_item(&mut self) -> Option<Box<dyn Item>> {
        self.advance();

        let return_type = if self.peek()?.token_type == TokenType::LeftParen {
            println!("No return type");
            ParsedType::Empty
        } else {
            self.try_type()?
        };

        let cur = self.cur()?;

        println!("Parsing functio name, cur token = {:?}", cur);

        let mut function_name = None;

        if let (TokenType::Identifier, Some(TokenValue::String(name))) = (cur.token_type, cur.value) {
            function_name = Some(name);
            self.advance();
        }

        self.consume(TokenType::LeftParen, None);

        let mut args = Vec::new();

        while !self.try_match(&[TokenType::RightParen]) {
            let arg_type = self.try_type()?;
            println!("Parsing arg type {:?}", arg_type);
            let cur = self.cur()?;

            let mut arg_name = None;

            if let (TokenType::Identifier, Some(TokenValue::String(name))) = (cur.token_type, cur.value) {
                arg_name = Some(name);
                self.advance();
            }

            args.push((arg_name?, arg_type));

            self.consume(TokenType::Comma, None);
            println!("Parsing arg");
        }

        println!("Parsing block");
        let block = self.block()?;
        println!("Parsed block, {:?}", function_name);
        Some(FunctionItem::new(function_name?, args, block, return_type, PositionRange::new(Position::new(0, 0))))
    }

    //expr;
    fn statement(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.expr()?;

        if self.try_match(&[TokenType::Semicolon]) {
            expr = UnaryExpr::new(expr, self.prev().as_ref().unwrap());
        } else {
            //TODO: Throw error if semicolon is needed
        }

        Some(expr)
    }

    //if_block | block
    fn expr(&mut self) -> Option<Box<dyn Expr>> {
        match self.cur()?.token_type {
            TokenType::Semicolon => {self.advance(); Some(EmptyExpr::new(self.cur()?.position))}
            TokenType::If => self.if_block(),
            TokenType::For => self.for_loop(),
            TokenType::While => self.while_loop(),
            TokenType::Loop => self.loop_expr(),
            TokenType::Break => self.break_expr(),
            TokenType::Rand => self.rand(),
            TokenType::Input => self.input(),
            TokenType::Print => self.print(),
            TokenType::Alloc => self.array_allocation(),
            TokenType::Putc => self.putc(),
            TokenType::Getc => self.getc(),
            _ => self.block(),
        }
    }

    fn putc(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        Some(PutCharExpr::new(self.expr()?,PositionRange::new(Position::new(0, 0))))
    }

    fn getc(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        Some(GetCharExpr::new(PositionRange::new(Position::new(0, 0))))
    }

    fn array_allocation(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();
        let array_type = self.try_type().unwrap();

        self.consume(TokenType::LeftSquare, None);
        
        let mut array_size = None;
        let cur = self.cur()?;
        if let (TokenType::IntLiteral, Some(TokenValue::Int(size))) = (cur.token_type, cur.value) {
            array_size = Some(size as usize);
            self.advance();
        }

        self.consume(TokenType::RightSquare, None);

        Some(StaticArrayExpr::new(array_size?, array_type))
    }

    //input expr
    fn input(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        let return_type = self.try_type()?;
        let prompt = self.expr()?;

        Some(InputExpr::new(prompt, return_type))
    }

    //rand(expr, expr)
    fn rand(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        self.consume(TokenType::LeftParen, None);
        let min = self.expr()?;
        self.consume(TokenType::Comma, None);
        let max = self.expr()?;
        self.consume(TokenType::RightParen, None);


        Some(RandExpr::new(min, max, &PositionRange::new(Position::new(0, 0))))
    }

    //print expr
    fn print(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        Some(PrintExpr::new(self.expr()?, &PositionRange::new(Position::new(0, 0))))
    }

    //break expr
    fn break_expr(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        let expr = self.expr()?;

        self.consume(TokenType::Semicolon, Some(self.err_expected_semicolon()));
        Some(BreakExpr::new(expr))
    }

    //while expr block
    fn while_loop(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        let condition = self.expr()?;
        let body = self.block()?;

        Some(LoopExpr::new_while(condition, body))
    }

    //loop block
    fn loop_expr(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        let body = self.block()?;

        Some(LoopExpr::new(body, PositionRange::new(Position::new(0, 0))))
    }

    //for (expr; expr; expr) block
    fn for_loop(&mut self) -> Option<Box<dyn Expr>> {
        println!("PARSING FOR");
        self.advance();

        let has_parenthesis = self.consume(TokenType::LeftParen, Some(self.err_expected_parenthesis_after_for()));

        let initial = self.expr();
        self.consume(TokenType::Semicolon, Some(self.err_expected_semicolon()));

        let condition = self.expr();

        self.consume(TokenType::Semicolon, Some(self.err_expected_semicolon()));

        let increment = self.expr();

        let err = if has_parenthesis {
            Some(self.err_expected_closing_parenthesis())
        } else {
            None
        };

        self.consume(TokenType::RightParen, err);

        println!("Parsing body: {:?}", self.cur());

        let body = self.block();

        println!("Initial {:?}", initial);
        println!("Condition {:?}", condition);
        println!("Increment {:?}", increment);
        println!("Body {:?}", body);
        
        let result = Some(LoopExpr::new_for(initial?, condition?, increment?, body?, PositionRange::new(Position::new(0, 0))));
        
        result
    }

    //if assignment block else if_block
    fn if_block(&mut self) -> Option<Box<dyn Expr>> {
        println!("Parsing if");
        self.advance();
        
        let condition = self.assignment()?;

        println!("If Condition: {:?}", condition);
        println!("If Next token: {:?}", self.cur());

        let success = self.block()?;

        println!("If Success: {:?}",success);

        let fail = if self.try_match(&[TokenType::Else]) {
            Some(match self.cur()?.token_type {
                TokenType::If => self.if_block()?,
                _ => self.block()?
            })
        } else {
            None
        };

        println!("Whee");

        Some(IfExpr::new(condition, success, fail))
    }

    //{ statement* } | declaration
    fn block(&mut self) -> Option<Box<dyn Expr>> {
        if self.try_match(&[TokenType::LeftCurly]) {
            let mut exprs: Vec<Box<dyn Expr>> = Vec::new();

            while self.cur()?.token_type != TokenType::RightCurly {
                exprs.push(self.statement()?);
                println!("Parsed block statement {:?}", exprs.get(exprs.len()-1));
                println!("Next token: {:?}", self.cur());
            }

            self.advance();
            Some(BlockExpr::new(exprs))
        } else {
            self.declaration()
        }
    }

    //let identifier: type = expr | assignment
    fn declaration(&mut self) -> Option<Box<dyn Expr>> {
        if self.try_match(&[TokenType::Let]) {
            println!("Parsing declaration");
            let declaration_type = self.try_type();

            let cur = self.cur()?;

            let identifier = match (cur.token_type, cur.value) {
                (TokenType::Identifier, Some(TokenValue::String(identifier))) => {
                    self.advance();
                    Some(identifier)
                },
                _ => {
                    self.diagnostics.push(self.err_expected_identifier());
                    None
                }
            };
            
            self.consume(TokenType::Assignment, Some(self.err_expected_equals()));

            let expr = self.expr()?;
            self.declaration_expr_id_counter += 1;
            Some(DeclarationExpr::new(self.declaration_expr_id_counter, identifier?, declaration_type?, expr))
        } else {
            self.assignment()
        }
    }

    //(IDENTIFIER =)* expr
    fn assignment(&mut self) -> Option<Box<dyn Expr>> {  
        println!("Trying assignment {:?}", self.cur());      
        match self.try_assignment() {
            Some(asignee) => {
                Some(AssignmentExpr::new(asignee, self.expr()?))
            },
            None => self.struct_initializer()
        }
    }

    //(*...)identifier.field.field ...[term][term]...
    fn var(&mut self) -> Option<VarExpr> {
        let identifier;
        let mut n_derefs = 0;

        while self.consume(TokenType::Star, None) {
            n_derefs += 1;
        }

        let mut cur = self.cur()?;

        if let (TokenType::Identifier, Some(TokenValue::String(value))) = (cur.token_type, cur.value) {
            identifier = Some(value);
            self.advance();
        } else {
            return None;
        }

        let mut member_accesses = Vec::new();

        while self.try_match(&[TokenType::Dot, TokenType::Arrow]) {
            let access_token = self.prev()?;

            cur = self.cur()?;

            if let (TokenType::Identifier, Some(TokenValue::String(value))) = (cur.token_type, cur.value) {
                if access_token.token_type == TokenType::Dot {
                    member_accesses.push(MemberAccess::Direct(value));
                } else {
                    member_accesses.push(MemberAccess::Indirect(value));
                }
            } else {
                //TODO: throw error
                return None;
            }

            self.advance();
        }

        let mut array_accesses = Vec::new();

        while self.consume(TokenType::LeftSquare, None) {
            array_accesses.push(self.term()?);
            self.consume(TokenType::RightSquare, None);
        }

        self.var_expr_id_counter += 1;

        Some(VarExpr::new_unboxed(self.var_expr_id_counter, n_derefs, identifier?, member_accesses, array_accesses))
    }

    fn struct_initializer(&mut self) -> Option<Box<dyn Expr>> {
        let cur = self.cur()?;

        if let (TokenType::Identifier, Some(TokenValue::String(type_name))) = (cur.token_type, cur.value) {
            if let Some(TokenType::LeftCurly) = self.peek().map(|token| token.token_type) {
                self.advance();
                self.advance();

                let mut member_inits = HashMap::new();

                while !self.consume(TokenType::RightCurly, None) {
                    let cur = self.cur()?;
                    let mut member_name = None;
                    
                    if let (TokenType::Identifier, Some(TokenValue::String(value))) = (cur.token_type, cur.value) {
                        member_name = Some(value);
                        self.advance();
                    }

                    self.consume(TokenType::Colon, None);

                    let expr = self.expr()?;

                    self.consume(TokenType::Comma, None);

                    member_inits.insert(member_name?, expr);
                }

                return Some(StructInitializerExpr::new(type_name, member_inits, PositionRange::new(Position::new(0, 0))))
            }
        }

        self.boolean_term()
    }

    //boolean_factor (and boolean_factor)*
    fn boolean_term(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.boolean_factor()?;

        while self.consume(TokenType::And, None) {
            let operator = self.prev()?;
            let right = self.boolean_factor()?;

            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }

    //equality (or equality)*
    fn boolean_factor(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.equality()?;

        while self.consume(TokenType::Or, None) {
            let operator = self.prev()?;
            let right = self.equality()?;

            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }

    //comparison (( "!=" | "==") comparison)*
    fn equality(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.comparison()?;
        let matches = [TokenType::Equal, TokenType::NotEqual];

        while self.try_match(&matches) {
            let operator = self.prev()?;
            let right: Box<dyn Expr> = self.comparison()?;
            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }

    //term ((">" | ">=" | "<" | "<=") term)*
    fn comparison(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.term()?;

        let matches = [TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        
        while self.try_match(&matches) {
            let operator = self.prev()?;
            let right = self.term()?;

            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }


    //factor (("-" | "+") factor)*
    fn term(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.factor()?;
        let matches = [TokenType::Minus, TokenType::Plus];

        while self.try_match(&matches) {
            let operator = self.prev()?;
            let right = self.factor()?;

            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }

    //unary (("/" | "*")) unary)*
    fn factor(&mut self) -> Option<Box<dyn Expr>> {
        let mut expr = self.unary()?;
        let matches = [TokenType::Slash, TokenType::Star];

        while self.try_match(&matches) {
            let operator = self.prev()?;
            let right = self.unary()?;

            expr = BinaryExpr::new(expr, right, &operator);
        }

        Some(expr)
    }

    //(("!" | "-") unary) | call
    fn unary(&mut self) -> Option<Box<dyn Expr>> {
        let matches = [TokenType::Not, TokenType::Minus];

        if self.try_match(&matches) {
            let operator = self.prev()?;
            Some(UnaryExpr::new(self.unary()?, &operator))
        } else {
            self.call()
        }
    }

    //identifier(args)* | primary
    fn call(&mut self) -> Option<Box<dyn Expr>> {
        if let Some(var_expr) = self.var() {
            if self.consume(TokenType::LeftParen, None) {
                let mut args: Vec<Box<dyn Expr>> = Vec::new();

                while !self.try_match(&[TokenType::RightParen]) {
                    args.push(self.boolean_term()?);
                    self.consume(TokenType::Comma, None);
                }

                Some(CallExpr::new(var_expr, args))
            } else {
                Some(Box::new(var_expr))
            }
        } else {
            self.primary()
        }
    }

    //&IDENTIFIER | LITERAL | "(" expr ")"
    fn primary(&mut self) -> Option<Box<dyn Expr>> {
        let cur = self.cur()?;

        match (cur.token_type, cur.value) {
            (TokenType::IntLiteral, Some(TokenValue::Int(value))) => {
                self.advance();
                Some(LiteralExpr::new(Literal::Int(value), ParsedType::Integer, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::DoubleLiteral, Some(TokenValue::Double(value))) => {
                self.advance();
                Some(LiteralExpr::new(Literal::Double(value), ParsedType::Double, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::BoolLiteral, Some(TokenValue::Bool(value))) => {
                self.advance();
                Some(LiteralExpr::new(Literal::Bool(value), ParsedType::Boolean, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::StringLiteral, Some(TokenValue::String(value))) => {
                self.advance();
                Some(LiteralExpr::new(Literal::String(value), ParsedType::Pointer(ParsedPointerType {pointee: Rc::new(ParsedType::Integer)}), PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::Identifier, Some(TokenValue::String(_))) => {
                Some(Box::new(self.var()?))
            },
            (TokenType::Ampersand, None) => {
                self.advance();
                Some(GetAddressExpr::new(self.var()?, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::LeftParen, None) => {
                self.advance();

                let expr = self.expr();

                self.consume(TokenType::RightParen, Some(self.err_expected_closing_parenthesis()));

                expr
            },
            _ => {
                self.advance();
                self.diagnostics.push(self.err_unexpected_token());
                None
            } 
        }
    }
}