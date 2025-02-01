use std::{collections::HashMap, rc::Rc};

use crate::{environment::{EnvRef, Function, FunctionType, ParsedFunctionType, ParsedType, TypedValue, Value}, error::{Diagnostic, DiagnosticType}, token::{Position, PositionRange, Token, TokenType, TokenValue}};
pub use self::expr::*;

pub mod expr;

pub struct ExprParser<'a> {
    ptr: usize,
    tokens: &'a[Token],
    diagnostics: Vec<Diagnostic>,
    var_expr_id_counter: i32,
}

pub struct ParseResult {
    pub exprs: Vec<Box<dyn Expr>>,
    pub diagnostics: Vec<Diagnostic>
}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &[Token]) -> ExprParser {
        ExprParser {ptr: 0, tokens, diagnostics: Vec::new(), var_expr_id_counter: 0}
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
        let mut exprs = Vec::new();
        
        while !self.is_at_end() {
            if let Some(expr) = self.statement() {
                exprs.push(expr);
            }
        }

        ParseResult {
            exprs,
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
        self.ptr == self.tokens.len()
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

    fn try_assignment(&mut self) -> Option<Rc<VarExpr>> {
        let cur = self.cur()?;

        if let (TokenType::Identifier, Some(TokenValue::String(identifier))) = (cur.token_type, cur.value) {
            if let TokenType::Assignment = self.peek()?.token_type {
                self.ptr += 2;
                
                self.var_expr_id_counter += 1;
                let asignee = VarExpr::new_unboxed(self.var_expr_id_counter, identifier);

                return Some(Rc::new(asignee))
            }
        }

        None
    }

    #[allow(dead_code)]
    fn is_type(&mut self) -> bool {
        if self.cur().is_none() {
            return false;
        }

        match self.cur().unwrap().token_type {
            TokenType::Bool | TokenType::Int | TokenType::Float | TokenType::Double | TokenType::String | TokenType::Func | TokenType::Identifier => true,
            _ => false
        }
    }

    //any primitive type
    //(type, type, ...) -> type
    fn try_type(&mut self) -> Option<ParsedType> {
        let cur = self.cur()?;

        match (cur.token_type, cur.value) {
            (TokenType::Int, None) => {self.advance(); Some(ParsedType::Integer)},
            (TokenType::Float, None) => {self.advance(); Some(ParsedType::Float)},
            (TokenType::Double, None) => {self.advance(); Some(ParsedType::Double)},
            (TokenType::Bool, None) => {self.advance(); Some(ParsedType::Boolean)},
            (TokenType::String, None) => {self.advance(); Some(ParsedType::String)},
            (TokenType::Identifier, Some(TokenValue::String(type_name))) => {self.advance(); Some(ParsedType::TypeName(type_name))},
            (TokenType::Func, None) => {
                self.advance();

                if self.try_match(&[TokenType::LeftParen]) {
                    let mut arg_types = Vec::new();

                    while let Some(arg_type) = self.try_type() {
                        println!("pushed arg {:?}, cur: {:?}", &arg_type, self.cur()?.token_type);

                        arg_types.push(arg_type);
                        if !self.try_match(&[TokenType::Comma]) {
                            break;
                        }
                    }                 

                    self.consume(TokenType::RightParen, None);
                    
                    let ret_type = if self.try_match(&[TokenType::Arrow]) {
                        Some(self.try_type()?)
                    } else {
                        None
                    }?;

                    Some(ParsedType::Function(ParsedFunctionType {arg_types: Rc::new(arg_types), ret_type: Rc::new(ret_type)}))
                } else {
                    None
                }
            },
            _ => None
        }    
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
            TokenType::Fn => self.function(),
            TokenType::Struct => self.struct_declaration(),
            _ => self.block(),
        }
    }

    fn struct_declaration(&mut self) -> Option<Box<dyn Expr>> {
        println!("Struct decalaration");
        self.advance();

        let mut cur = self.cur()?;
        let mut struct_name = None;

        if let (TokenType::Identifier, Some(TokenValue::String(name))) = (cur.token_type, cur.value) {
            println!("Parsed struct name {}", name);
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

            println!("Parsed memeber {:?} {:?}", member_type, member_name);

            members.insert(member_name?, member_type);

            self.consume(TokenType::Semicolon, None);

            cur = self.cur()?;
        }

        if !self.consume(TokenType::RightCurly, None) {
            return None
        }

        Some(StructExpr::new(struct_name?, members, PositionRange::new(Position::new(0, 0))))
    }

    //fn (args) -> type expr
    fn function(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();

        //TODO: add identifier thing

        let mut args = Vec::new();
        let mut arg_types = Vec::new();

        self.consume(TokenType::LeftParen, None);

        let peek = self.peek().unwrap();
        
        if let (TokenType::Identifier, Some(TokenValue::String(identifier))) = (peek.token_type, peek.value) {
            arg_types.push(self.try_type()?);
            args.push(identifier);
            self.advance();
        }

        while self.try_match(&[TokenType::Comma]) {
            let peek = self.peek().unwrap();

            if let (TokenType::Identifier, Some(TokenValue::String(identifier))) = (peek.token_type, peek.value) {
                arg_types.push(self.try_type()?);
                args.push(identifier);
                self.advance();
            } else {
                self.advance();
                self.diagnostics.push(self.err_expected_identifier());
            }
        }

        self.consume(TokenType::RightParen, None);

        self.consume(TokenType::Arrow, None);

        let ret_type = self.try_type()?;
        
        let body = self.expr()?;

        let value = Value::Function(Function {args: Rc::new(args), body: body.into(), env: EnvRef::new_none()});
        let parsed_type = ParsedType::Function(ParsedFunctionType {arg_types: Rc::new(arg_types), ret_type: Rc::new(ret_type)});

        //TODO: fix position
        Some(LiteralExpr::new(value, parsed_type, PositionRange::new(Position::new(0, 0))))
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
        println!("done parsing break; cur: {:?}", self.cur()?.token_type);
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

        let body = self.block();

        Some(LoopExpr::new_for(initial?, condition?, increment?, body?, PositionRange::new(Position::new(0, 0))))
    }

    //if assignment block else if_block
    fn if_block(&mut self) -> Option<Box<dyn Expr>> {
        self.advance();
        
        let condition = self.assignment()?;
        let success = self.block()?;

        let fail = if self.try_match(&[TokenType::Else]) {
            Some(match self.cur()?.token_type {
                TokenType::If => self.if_block()?,
                _ => self.block()?
            })
        } else {
            None
        };

        Some(IfExpr::new(condition, success, fail))
    }

    //{ statement* } | declaration
    fn block(&mut self) -> Option<Box<dyn Expr>> {
        if self.try_match(&[TokenType::LeftCurly]) {
            println!("parsing block");
            let mut exprs: Vec<Box<dyn Expr>> = Vec::new();

            while self.cur()?.token_type != TokenType::RightCurly {
                println!("cur: {:?}", self.cur()?.token_type);
                exprs.push(self.statement()?);
            }

            self.advance();
            println!("done parsing block");
            Some(BlockExpr::new(exprs))
        } else {
            self.declaration()
        }
    }

    //let identifier: type = expr | assignment
    fn declaration(&mut self) -> Option<Box<dyn Expr>> {
        if self.try_match(&[TokenType::Let]) {
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

            Some(DeclarationExpr::new(identifier?, declaration_type?, self.expr()?))
        } else {
            self.assignment()
        }
    }

    //(IDENTIFIER =)* expr
    fn assignment(&mut self) -> Option<Box<dyn Expr>> {        
        match self.try_assignment() {
            Some(asignee) => {
                Some(AssignmentExpr::new(asignee, self.expr()?))
            },
            None => self.equality()
        }
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
        let identifier_token = self.cur()?;

        if let (TokenType::Identifier, Some(TokenValue::String(identifier))) = (identifier_token.token_type, identifier_token.value) {
            if self.peek().unwrap().token_type == TokenType::LeftParen {
                println!("parsing fn {:?}", self.cur());
                self.advance(); self.advance();
                let mut args = Vec::new();

                while !self.try_match(&[TokenType::RightParen]) {
                    args.push(self.equality()?);
                    self.consume(TokenType::Comma, None);
                }
                println!("parsed ars {:?}", self.cur());

                self.var_expr_id_counter += 1;
                Some(CallExpr::new(VarExpr::new(self.var_expr_id_counter, identifier, PositionRange::new(Position::new(0, 0))), args))
            } else {
                self.primary()
            }
        } else {
            self.primary()
        }
    }

    //IDENTIFIER | LITERAL | "(" expr ")"
    fn primary(&mut self) -> Option<Box<dyn Expr>> {
        println!("Parsing primary {:?}", self.cur());
        let cur = self.cur()?;

        match (cur.token_type, cur.value) {
            (TokenType::IntLiteral, Some(TokenValue::Int(value))) => {
                self.advance();
                Some(LiteralExpr::new(Value::Int(value), ParsedType::Integer, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::FloatLiteral, Some(TokenValue::Float(value))) => {
                self.advance();
                Some(LiteralExpr::new(Value::Float(value), ParsedType::Float, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::DoubleLiteral, Some(TokenValue::Double(value))) => {
                self.advance();
                Some(LiteralExpr::new(Value::Double(value), ParsedType::Double, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::BoolLiteral, Some(TokenValue::Bool(value))) => {
                self.advance();
                Some(LiteralExpr::new(Value::Bool(value), ParsedType::Boolean, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::StringLiteral, Some(TokenValue::String(value))) => {
                self.advance();
                Some(LiteralExpr::new(Value::String(value), ParsedType::String, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::Identifier, Some(TokenValue::String(identifier))) => {
                let token = self.cur()?;
                self.advance();

                self.var_expr_id_counter += 1;
                Some(VarExpr::new(self.var_expr_id_counter, identifier, token.position))
            },
            (TokenType::LeftParen, None) => {
                self.advance();

                let expr = self.expr();

                self.consume(TokenType::RightParen, Some(self.err_expected_closing_parenthesis()));

                expr
            }
            _ => {
                self.advance();
                self.diagnostics.push(self.err_unexpected_token());
                None
            } 
        }
    }
}