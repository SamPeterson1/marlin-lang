use std::{collections::HashMap, fmt, rc::Rc};

use crate::expr::literal_expr::Literal;
use crate::item::{FunctionItem, Item, StructItem};

use crate::types::parsed_type::{ParsedPointerType, ParsedType, ParsedTypeName};
use crate::{error::{Diagnostic, DiagnosticType}, expr::{assignment_expr::AssignmentExpr, binary_expr::BinaryExpr, block_expr::BlockExpr, break_expr::BreakExpr, call_expr::CallExpr, declaration_expr::DeclarationExpr, get_address_expr::GetAddressExpr, get_char_expr::GetCharExpr, if_expr::IfExpr, literal_expr::LiteralExpr, loop_expr::LoopExpr, put_char_expr::PutCharExpr, static_array_expr::StaticArrayExpr, struct_initializer_expr::StructInitializerExpr, unary_expr::UnaryExpr, var_expr::{MemberAccess, VarExpr}, Expr}, logger::{LogSeverity, Logger}, token::{Position, PositionRange, Token, TokenType, TokenValue}};

pub struct ExprParser<'a> {
    ptr: usize,
    tokens: &'a[Token],
    diagnostics: Vec<Diagnostic>,
    var_expr_id_counter: i32,
    declaration_expr_id_counter: i32,
    logger: Logger,
}

pub struct ParseResult {
    pub items: Vec<Box<dyn Item>>,
    pub diagnostics: Vec<Diagnostic>
}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &[Token]) -> ExprParser {
        ExprParser {ptr: 1, tokens, diagnostics: Vec::new(), var_expr_id_counter: 0, declaration_expr_id_counter: 0, logger: Logger::new("ExprParser")}
    }
    
    fn err_expected_struct_name(&self) -> Diagnostic {
        let msg = String::from("expected struct name");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_member_name(&self) -> Diagnostic {
        let msg = String::from("expected member name");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_member_type(&self) -> Diagnostic {
        let msg = String::from("expected member type");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_fn_name(&self) -> Diagnostic {
        let msg = String::from("expected function name");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_unexpected_token(&self) -> Diagnostic {
        let msg = String::from("unexpected token");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_item(&self) -> Diagnostic {
        let msg = String::from("expected item");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_token(&self, token_type: TokenType) -> Diagnostic {
        let msg = format!("expected {}", token_type);

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_arg_type(&self) -> Diagnostic {
        let msg = String::from("expected argument type");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_arg_name(&self) -> Diagnostic {
        let msg = String::from("expected argument name");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_return_type(&self) -> Diagnostic {
        let msg = String::from("expected return type");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_declaration_type(&self) -> Diagnostic {
        let msg = String::from("expected declaration type");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_declaration_name(&self) -> Diagnostic {
        let msg = String::from("expected declaration name");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn err_expected_var(&self) -> Diagnostic {
        let msg = String::from("expected variable");

        Diagnostic::new(1, DiagnosticType::Error, self.cur().position, msg)
    }

    fn log_parse_result(&self, expr: &Option<impl fmt::Display>, name: &str) {
        if let Some(expr) = expr {
            self.logger.log_trace_info(&format!("Parsed {}: {}", name, expr));
        } else {
            self.logger.log_trace_error(&format!("Failed to parse {}", name));
        }
    }

    pub fn parse(mut self) -> ParseResult {
        self.logger.log_brief_info("Beginning parser");

        let mut items = Vec::new();
        
        while !self.is_at_end() {
            if let Some(expr) = self.item() {
                self.logger.log_brief_info(&format!("Parsed item successfully"));
                self.logger.log_detailed_info(&format!("Parsed item: {}", expr));
                items.push(expr);
            } else {
                self.logger.log_brief_error(&format!("Failed to parse item. Current token: {:?}", self.cur()));
            }
        }

        self.logger.log_brief_info(&format!("Parsed {} items with {} diagnostics", items.len(), self.diagnostics.len()));

        ParseResult {
            items,
            diagnostics: self.diagnostics
        }
    }

    fn try_match(&mut self, matches: &[TokenType]) -> Option<Token> {
        for token_match in matches {
            if self.cur().token_type == *token_match {
                return Some(self.advance());
            }
        }

        return None;
    }

    fn advance(&mut self) -> Token {
        let ret = &self.tokens[self.ptr];

        if self.tokens[self.ptr].token_type != TokenType::EOF {
            self.ptr += 1;
        }

        ret.clone()
    }

    fn is_at_end(&self) -> bool {
        self.ptr == self.tokens.len() || self.tokens[self.ptr].token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        if self.ptr + 1 >= self.tokens.len() {
            return self.cur();
        }

        self.tokens[self.ptr + 1].clone()
    }

    fn cur(&self) -> Token {
        self.tokens[self.ptr].clone()
    }

    fn prev(&self) -> Token {
        self.tokens[self.ptr - 1].clone()
    }

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        let log_severity = match diagnostic.diagnostic_type {
            DiagnosticType::Error => LogSeverity::Error,
            DiagnosticType::Warning => LogSeverity::Warning,
        };

        self.logger.log_brief(log_severity, &format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }

    fn some_or_diagnostic<T>(&mut self, opt: Option<T>, diagnostic: Diagnostic) -> Option<T> {
        if opt.is_none() {
            self.push_diagnostic(diagnostic);
        }

        opt
    }

    fn try_consume(&mut self, token: TokenType) -> Option<Token> {
        if self.cur().token_type == token {
            Some(self.advance())
        } else {
            None
        }
    }

    fn consume_or_diagnostic(&mut self, token: TokenType, diagnostic: Diagnostic) -> Option<Token> {
        if self.cur().token_type != token {
            self.push_diagnostic(diagnostic);
            
            None
        } else {
            Some(self.advance())
        }
    }

    fn try_assignment(&mut self) -> Option<VarExpr> {
        self.logger.log_trace_info("Trying to parse assignment");
        let ptr = self.ptr;
        let var_expr = self.var();

        if let Some(var_expr) = &var_expr {
            self.logger.log_trace_info(&format!("Parsed var expr: {}", var_expr));
        } else {
            self.logger.log_trace_info("Did not parse var expr");
        }

        if var_expr.is_none() || self.try_consume(TokenType::Assignment).is_none() {
            self.logger.log_trace_info("No assignment found");
            self.ptr = ptr;
            return None;
        }

        self.logger.log_trace_info("Found assignment");
        var_expr
    }

    //any primitive type
    //(type, type, ...) -> type
    fn try_type(&mut self) -> Option<ParsedType> {
        let cur = self.cur();

        match (cur.token_type, cur.value) {
            (TokenType::Int, TokenValue::None) => {self.advance(); Some(ParsedType::Integer)},
            (TokenType::Double, TokenValue::None) => {self.advance(); Some(ParsedType::Double)},
            (TokenType::Bool, TokenValue::None) => {self.advance(); Some(ParsedType::Boolean)},
            (TokenType::Identifier, TokenValue::String(type_name)) => {
                self.advance(); 
                Some(ParsedType::TypeName(ParsedTypeName {
                    name: type_name.to_string().into(),
                    position: cur.position
                }))
            },
            (TokenType::Star, TokenValue::None) => {
                self.advance();
                let pointee = self.try_type()?;
                Some(ParsedType::Pointer(ParsedPointerType {pointee: Rc::new(pointee)}))
            },
            _ => None
        }    
    }

    fn item(&mut self) -> Option<Box<dyn Item>> {
        self.logger.log_trace_info(&format!("Entering item parser. Current token {:?}", self.cur()));
        let cur = self.cur();

        match cur.token_type {
            TokenType::Struct => self.struct_item(),
            TokenType::Fn => self.function_item(),
            _ => {
                self.push_diagnostic(self.err_expected_item());
                self.advance();
                None
            }
        }
    }

    //struct: STRUCT IDENTIFIER LEFT_CURLY ([type] IDENTIFIER COMMA)* RIGHT_CURLY
    fn struct_item(&mut self) -> Option<Box<dyn Item>> {
        self.logger.log_trace_info(&format!("Entering struct parser. Current token {:?}", self.cur()));
        let struct_token = self.advance();

        let struct_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_struct_name())
            .map(|x| x.get_string().to_string());

        self.log_parse_result(&struct_name, "struct name");

        self.consume_or_diagnostic(TokenType::LeftCurly, self.err_expected_token(TokenType::LeftCurly));

        let mut members = HashMap::new();

        loop {
            let opt_type = self.try_type();
            let member_type = self.some_or_diagnostic(opt_type, self.err_expected_member_name());
            let member_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_member_name())
                .map(|x| x.get_string().to_string());

            self.log_parse_result(&member_type, "member type");
            self.log_parse_result(&member_name, "member name");

            if let (Some(member_type), Some(member_name)) = (member_type, member_name) {
                members.insert(member_name, member_type);
            }

            if self.try_consume(TokenType::Comma).is_none() {
                self.logger.log_trace_info(&format!("Done parsing struct members"));
                self.consume_or_diagnostic(TokenType::RightCurly, self.err_expected_token(TokenType::RightCurly));
                break;
            }
        }

        let position = PositionRange::concat(&struct_token.position, &self.prev().position);

        Some(StructItem::new(struct_name?, members, position))
    }

    //function: FN IDENTIFIER LEFT_PAREN [type identifier]* RIGHT_PAREN (-> [type])? [block]
    fn function_item(&mut self) -> Option<Box<dyn Item>> {
        self.logger.log_trace_info(&format!("Entering function parser. Current token {:?}", self.cur()));
        let fn_token = self.advance();

        let function_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_fn_name())
            .map(|x| x.get_string().to_string());

        self.log_parse_result(&function_name, "function name");

        self.consume_or_diagnostic(TokenType::LeftParen, self.err_expected_token(TokenType::LeftParen));

        let mut args = Vec::new();

        loop {
            if self.try_consume(TokenType::RightParen).is_some() {
                self.logger.log_trace_info(&format!("Done parsing function arguments"));
                break;
            }

            let opt_type = self.try_type();
            let arg_type = self.some_or_diagnostic(opt_type, self.err_expected_arg_type());

            let arg_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_arg_name())
                .map(|x| x.get_string().to_string());

            self.log_parse_result(&arg_type, "argument type");
            self.log_parse_result(&arg_name, "argument name");

            if let (Some(arg_type), Some(arg_name)) = (arg_type, arg_name) {
                args.push((arg_name, arg_type));
            }

            if self.try_consume(TokenType::Comma).is_none() {
                self.logger.log_trace_info(&format!("Done parsing function arguments"));
                self.consume_or_diagnostic(TokenType::RightParen, self.err_expected_token(TokenType::RightParen));
                break;
            }
        }

        let return_type = if self.try_consume(TokenType::Arrow).is_some() {
            let opt_type = self.try_type();
            self.some_or_diagnostic(opt_type, self.err_expected_return_type())
        } else {
            Some(ParsedType::Empty)
        };

        self.log_parse_result(&return_type, "return type");

        let block = self.block();

        self.log_parse_result(&block, "function block");

        let position = PositionRange::concat(&fn_token.position, &self.prev().position);

        Some(FunctionItem::new(function_name?, args, block?, return_type?, position))
    }

    //statement: [expression] SEMICOLON
    fn statement(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering statement parser. Current token {:?}", self.cur()));

        let expr = self.expr();

        self.log_parse_result(&expr, "expression");

        if let Some(semicolon_token) = self.try_consume(TokenType::Semicolon) {
            self.logger.log_trace_info(&format!("Parsed semicolon token"));
            Some(UnaryExpr::new(expr?, semicolon_token))
        } else {
            self.logger.log_trace_info(&format!("No semicolon found"));
            expr
        }
    }


    
    //expression: [loop] | [if] | [alloc] | [getc] | [block] | [for] | [while] | [break] | [putc] | [struct_initializer] | [declaration]
    fn expr(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering expression parser. Current token {:?}", self.cur()));

        match self.cur().token_type {
            TokenType::If => self.if_block(),
            TokenType::For => self.for_loop(),
            TokenType::While => self.while_loop(),
            TokenType::Loop => self.loop_expr(),
            TokenType::Break => self.break_expr(),
            TokenType::Alloc => self.array_allocation(),
            TokenType::Putc => self.putc(),
            TokenType::Getc => self.getc(),
            TokenType::LeftCurly => self.block(),
            TokenType::Identifier => {
                if self.peek().token_type == TokenType::LeftCurly {
                    self.struct_initializer()
                } else {
                    self.declaration()
                }
            },
            _ => self.declaration()
        }
    }

    //inline_expression: [condition]
    fn inline_expr(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering inline expression parser. Current token {:?}", self.cur()));

        self.condition()
    }

    fn putc(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering putc parser. Current token {:?}", self.cur()));
        self.advance();

        let expr = self.expr();

        self.log_parse_result(&expr, "putc expression");

        Some(PutCharExpr::new(expr?,PositionRange::new(Position::new(0, 0))))
    }

    fn getc(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering getc parser. Current token {:?}", self.cur()));
        self.advance();

        Some(GetCharExpr::new(PositionRange::new(Position::new(0, 0))))
    }

    fn array_allocation(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering array allocation parser. Current token {:?}", self.cur()));
        let alloc_token = self.advance();
        let array_type = self.try_type();

        self.log_parse_result(&array_type, "array type");

        self.consume_or_diagnostic(TokenType::LeftSquare, self.err_expected_token(TokenType::LeftSquare));
        
        let array_size = self.consume_or_diagnostic(TokenType::IntLiteral, self.err_expected_token(TokenType::IntLiteral))
            .map(|x| x.get_int() as usize);

        self.log_parse_result(&array_size, "array size");

        self.consume_or_diagnostic(TokenType::RightSquare, self.err_expected_token(TokenType::RightSquare));

        let position = PositionRange::concat(&alloc_token.position, &self.prev().position);

        Some(StaticArrayExpr::new(array_size?, array_type?, position))
    }

    //break: BREAK [expression]? SEMICOLON
    fn break_expr(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering break parser. Current token {:?}", self.cur()));
        let break_token = self.advance();

        let expr = self.expr();

        self.log_parse_result(&expr, "break expression");
        self.consume_or_diagnostic(TokenType::Semicolon, self.err_expected_token(TokenType::Semicolon));

        let position = PositionRange::concat(&break_token.position, &self.prev().position);

        Some(BreakExpr::new(expr?, position))
    }

    //while: WHILE [inline_expression] [block]
    fn while_loop(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering while parser. Current token {:?}", self.cur()));
        let while_token = self.advance();

        let condition = self.inline_expr();
        let body = self.block();

        self.log_parse_result(&condition, "while condition");
        self.log_parse_result(&body, "while body");

        let position = PositionRange::concat(&while_token.position, &self.prev().position);

        Some(LoopExpr::new_while(condition?, body?, position))
    }

    //loop: LOOP [block]
    fn loop_expr(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering loop parser. Current token {:?}", self.cur()));
        let loop_token = self.advance();

        let body = self.block();

        self.log_parse_result(&body, "loop body");

        let position = PositionRange::concat(&loop_token.position, &self.prev().position);

        Some(LoopExpr::new(body?, position))
    }

    //for: FOR LEFT_PAREN [declaration] [inline_expression] SEMICOLON [assignment] RIGHT_PAREN [block]
    fn for_loop(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering for parser. Current token {:?}", self.cur()));
        let for_token = self.advance();

        self.consume_or_diagnostic(TokenType::LeftParen, self.err_expected_token(TokenType::LeftParen));

        let initial = self.declaration();
        self.log_parse_result(&initial, "for initial");

        let condition = self.inline_expr();
        self.log_parse_result(&condition, "for condition");

        self.consume_or_diagnostic(TokenType::Semicolon, self.err_expected_token(TokenType::Semicolon));

        let increment = self.assignment();
        self.log_parse_result(&increment, "for increment");

        self.consume_or_diagnostic(TokenType::RightParen, self.err_expected_token(TokenType::RightParen));


        let body = self.block();    
        self.log_parse_result(&body, "for body");
        
        let position= PositionRange::concat(&for_token.position, &self.prev().position);

        let result = Some(LoopExpr::new_for(initial?, condition?, increment?, body?, position));
        
        result
    }

    //if: IF [inline_expression] [block] [elif]* [else]?
    fn if_block(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering if parser. Current token {:?}", self.cur()));
        let if_token = self.advance();  
        
        let condition = self.inline_expr();
        self.log_parse_result(&condition, "if condition");

        let success = self.block();
        self.log_parse_result(&success, "if success");

        let fail = if self.try_consume(TokenType::Else).is_some(){
            let fail = match self.cur().token_type {
                TokenType::If => self.if_block(),
                _ => self.block()
            };

            self.log_parse_result(&fail, "if fail");
            fail
        } else {
            self.logger.log_trace_info(&format!("No else block found"));
            None
        };

        let position = PositionRange::concat(&if_token.position, &self.prev().position);

        Some(IfExpr::new(condition?, success?, fail, position))
    }

    //block: LEFT_CURLY [statement]* RIGHT_CURLY
    fn block(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering block parser. Current token {:?}", self.cur()));
        let start_token = self.cur();
        self.consume_or_diagnostic(TokenType::LeftCurly, self.err_expected_token(TokenType::LeftCurly));
        let mut exprs: Vec<Box<dyn Expr>> = Vec::new();

        while self.try_match(&[TokenType::EOF, TokenType::RightCurly]).is_none() {
            let statement = self.statement();
            self.log_parse_result(&statement, "block statement");

            if let Some(statement) = statement {
                exprs.push(statement);
            }
            //println!("Parsed block statement");
            //println!("Next token: {:?}", self.cur());
        }

        let position = PositionRange::concat(&start_token.position, &self.prev().position);

        Some(BlockExpr::new(exprs, position))
    }

    //declaration: LET [type] IDENTIFIER ASSIGNMENT [expression]? SEMICOLON | [assignment]
    fn declaration(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering declaration parser. Current token {:?}", self.cur()));
        if let Some(let_token) = self.try_consume(TokenType::Let) {
            let opt_type = self.try_type();
            let declaration_type = self.some_or_diagnostic(opt_type, self.err_expected_declaration_type());
            self.log_parse_result(&declaration_type, "declaration type");

            let declaration_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_declaration_name())
                .map(|x| x.get_string().to_string());
            self.log_parse_result(&declaration_name, "declaration name");

            self.consume_or_diagnostic(TokenType::Assignment, self.err_expected_token(TokenType::Assignment));

            let expr = self.expr();
            self.log_parse_result(&expr, "declaration expression");

            self.consume_or_diagnostic(TokenType::Semicolon, self.err_expected_token(TokenType::Semicolon));

            let position = PositionRange::concat(&let_token.position, &self.prev().position);

            self.declaration_expr_id_counter += 1;
            Some(DeclarationExpr::new(self.declaration_expr_id_counter, declaration_name?, declaration_type?, expr?, position))
        } else {
            self.assignment()
        }
    }

    //assignment: [var] ASSIGNMENT [expression] SEMICOLON | [inline_expression]
    fn assignment(&mut self) -> Option<Box<dyn Expr>> {  
        self.logger.log_trace_info(&format!("Entering assignment parser. Current token {:?}", self.cur()));
        match self.try_assignment() {
            Some(asignee) => {
                let expr = self.expr();
                self.log_parse_result(&expr, "assignment expression");

                self.consume_or_diagnostic(TokenType::Semicolon, self.err_expected_token(TokenType::Semicolon));

                Some(AssignmentExpr::new(asignee, expr?))
            },
            None => {
                self.inline_expr()
            }
        }
    }

    //var: STAR* IDENTIFIER (DOT IDENTIFIER)* (LEFT_BRACKET inline_expression RIGHT_BRACKET)*
    fn var(&mut self) -> Option<VarExpr> {
        self.logger.log_trace_info(&format!("Entering var parser. Current token {:?}", self.cur()));
        
        let first_position = self.cur().position.clone();
        let mut n_derefs = 0;

        while self.try_consume(TokenType::Star).is_some() {
            n_derefs += 1;
        }

        self.logger.log_trace_info(&format!("Parsed {} derefs", n_derefs));

        let identifier = self.try_consume(TokenType::Identifier)
            .map(|x| x.get_string().to_string())?;

        let mut member_accesses = Vec::new();

        while let Some(access_token) = self.try_match(&[TokenType::Dot, TokenType::Arrow]) {
            if let Some(identifier) = self.try_consume(TokenType::Identifier) {
                if access_token.token_type == TokenType::Dot {
                    self.logger.log_trace_info("Parsed direct member access");
                    member_accesses.push(MemberAccess::Direct(identifier.get_string().to_string()));
                } else {
                    self.logger.log_trace_info("Parsed indirect member access");
                    member_accesses.push(MemberAccess::Indirect(identifier.get_string().to_string()));
                }
            } else {
                return None;
            }
        }

        let mut array_accesses = Vec::new();

        while self.try_consume(TokenType::LeftSquare).is_some() {
            let expr = self.inline_expr();

            if let Some(inline_expr) = expr {
                array_accesses.push(inline_expr);   
            }

            self.try_consume(TokenType::RightSquare)?;
        }

        self.var_expr_id_counter += 1;

        let position = PositionRange::concat(&first_position, &self.prev().position);

        Some(VarExpr::new_unboxed(self.var_expr_id_counter, n_derefs, identifier, member_accesses, array_accesses, position))
    }

    //struct_initializer: IDENTIFIER LEFT_CURLY [member_intializer] (COMMA, [member_intializer])* RIGHT_CURLY
    //member_intializer: IDENTIFIER COLON [inline_expression]
    fn struct_initializer(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering struct initializer parser. Current token {:?}", self.cur()));

        let type_name_token = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_struct_name());
        let type_name = type_name_token.as_ref().map(|x| x.get_string().to_string());

        self.log_parse_result(&type_name, "struct type name");

        self.consume_or_diagnostic(TokenType::LeftCurly, self.err_expected_token(TokenType::LeftCurly));

        let mut member_inits = HashMap::new();

        loop {
            let member_name = self.consume_or_diagnostic(TokenType::Identifier, self.err_expected_member_name())
                .map(|x| x.get_string().to_string());
            self.log_parse_result(&member_name, "member name");

            self.consume_or_diagnostic(TokenType::Colon, self.err_expected_token(TokenType::Colon));

            let expr = self.expr();
            self.log_parse_result(&expr, "member expression");

            member_inits.insert(member_name?, expr?);

            if self.try_consume(TokenType::Comma).is_none() {
                self.logger.log_trace_info(&format!("Done parsing struct member initializers"));
                self.consume_or_diagnostic(TokenType::RightCurly, self.err_expected_token(TokenType::RightCurly));
                break;
            }
        }

        let position = PositionRange::concat(&type_name_token?.position, &self.prev().position);

        Some(StructInitializerExpr::new(type_name?, member_inits, position))
    }

    //condition: [boolean_factor] (OR [boolean_factor])*
    fn condition(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering condition parser. Current token {:?}", self.cur()));

        let mut boolean_factor = self.boolean_factor();
        self.log_parse_result(&boolean_factor, "boolean factor");
        let mut expr = boolean_factor?;

        while let Some(operator) = self.try_consume(TokenType::Or) {
            boolean_factor = self.boolean_factor();
            self.log_parse_result(&boolean_factor, "boolean factor");

            expr = BinaryExpr::new(expr, boolean_factor?, operator.token_type);
        }

        Some(expr)
    }

    //equality (or equality)*
    fn boolean_factor(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering boolean factor parser. Current token {:?}", self.cur()));

        let mut equality = self.equality();
        self.log_parse_result(&equality, "equality expression");
        let mut expr = equality?;

        while let Some(operator) = self.try_consume(TokenType::And) {
            equality = self.equality();
            self.log_parse_result(&equality, "equality expression");

            expr = BinaryExpr::new(expr, equality?, operator.token_type);
        }

        Some(expr)
    }

    //comparison (( "!=" | "==") comparison)*
    fn equality(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering equality parser. Current token {:?}", self.cur()));

        let mut comparison = self.comparison();
        self.log_parse_result(&comparison, "comparison expression");

        let mut expr = comparison?;

        while let Some(operator) = self.try_match(&[TokenType::Equal, TokenType::NotEqual]) {
            comparison = self.comparison();
            self.log_parse_result(&comparison, "comparison expression");

            expr = BinaryExpr::new(expr, comparison?, operator.token_type);
        }

        Some(expr)
    }

    //term ((">" | ">=" | "<" | "<=") term)*
    fn comparison(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering comparison parser. Current token {:?}", self.cur()));

        let mut term = self.term();
        self.log_parse_result(&term, "term expression");
        let mut expr = term?;

        let matches = [TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];
        
        while let Some(operator) = self.try_match(&matches) {
            term = self.term();
            self.log_parse_result(&term, "term expression");
            expr = BinaryExpr::new(expr, term?, operator.token_type);
        }

        Some(expr)
    }


    //factor (("-" | "+") factor)*
    fn term(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering term parser. Current token {:?}", self.cur()));

        let mut factor = self.factor();
        self.log_parse_result(&factor, "factor expression");
        let mut expr = factor?;

        while let Some(operator) = self.try_match(&[TokenType::Minus, TokenType::Plus]) {
            factor = self.factor();
            self.log_parse_result(&factor, "factor expression");
            expr = BinaryExpr::new(expr, factor?, operator.token_type);
        }

        Some(expr)
    }

    //unary (("/" | "*")) unary)*
    fn factor(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering factor parser. Current token {:?}", self.cur()));

        let mut unary = self.unary();
        self.log_parse_result(&unary, "unary expression");
        let mut expr = unary?;

        while let Some(operator) = self.try_match(&[TokenType::Slash, TokenType::Star]) {
            unary = self.unary();
            self.log_parse_result(&unary, "unary expression");
            expr = BinaryExpr::new(expr, unary?, operator.token_type);
        }

        Some(expr)
    }

    //(("!" | "-") unary) | call
    fn unary(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering unary parser. Current token {:?}", self.cur()));

        if let Some(operator) = self.try_match(&[TokenType::Not, TokenType::Minus]) {
            let unary = self.unary();
            self.log_parse_result(&unary, "unary expression");
            Some(UnaryExpr::new(unary?, operator))
        } else {
            self.call()
        }
    }

    //call: IDENTIFIER LEFT_PAREN (([inline_expression], COMMA)* [inline_expression]?) | [primary]
    fn call(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering call parser. Current token {:?}", self.cur()));

        if self.cur().token_type == TokenType::Identifier {
            if self.peek().token_type == TokenType::LeftParen {
                let function_name = self.advance().get_string().to_string();
                self.logger.log_trace_info(&format!("Parsed function name: {}", function_name));

                self.advance();

                let mut args: Vec<Box<dyn Expr>> = Vec::new();

                loop {
                    let arg = self.inline_expr();
                    self.log_parse_result(&arg, "call arg");

                    if let Some(arg) = arg {
                        args.push(arg);
                    }

                    if self.try_consume(TokenType::Comma).is_none() {
                        self.logger.log_trace_info(&format!("Done parsing call args"));
                        self.consume_or_diagnostic(TokenType::RightParen, self.err_expected_token(TokenType::RightParen));
                        break;
                    }
                }

                Some(CallExpr::new(function_name, args))
            } else {
                self.primary()
            }
        } else {
            self.primary()
        }
    }

    //primary: &?[var] | [literal] | LEFT_PAREN inline_expression RIGHT_PAREN
    fn primary(&mut self) -> Option<Box<dyn Expr>> {
        self.logger.log_trace_info(&format!("Entering primary parser. Current token {:?}", self.cur()));
        let cur = self.cur();

        match (cur.token_type, cur.value) {
            (TokenType::IntLiteral, TokenValue::Int(value)) => {
                self.logger.log_trace_info(&format!("Parsed int literal: {}", value));
                self.advance();
                Some(LiteralExpr::new(Literal::Int(value), ParsedType::Integer, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::DoubleLiteral, TokenValue::Double(value)) => {
                self.logger.log_trace_info(&format!("Parsed double literal: {}", value));
                self.advance();
                Some(LiteralExpr::new(Literal::Double(value), ParsedType::Double, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::BoolLiteral, TokenValue::Bool(value)) => {
                self.logger.log_trace_info(&format!("Parsed bool literal: {}", value));
                self.advance();
                Some(LiteralExpr::new(Literal::Bool(value), ParsedType::Boolean, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::StringLiteral, TokenValue::String(value)) => {
                self.logger.log_trace_info(&format!("Parsed string literal: {}", value));
                self.advance();
                Some(LiteralExpr::new(Literal::String(value), ParsedType::Pointer(ParsedPointerType {pointee: Rc::new(ParsedType::Integer)}), PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::Ampersand, TokenValue::None) => {
                self.advance();
                let var_opt = self.var();
                let var_expr = self.some_or_diagnostic(var_opt, self.err_expected_var());
                self.log_parse_result(&var_expr, "get address var expression");
                Some(GetAddressExpr::new(var_expr?, PositionRange::new(Position::new(0, 0))))
            },
            (TokenType::LeftParen, TokenValue::None) => {
                self.advance();

                let expr = self.expr();
                self.log_parse_result(&expr, "parenthesized expression");

                self.consume_or_diagnostic(TokenType::RightParen, self.err_expected_token(TokenType::RightParen));

                expr
            },
            _ => {
                let var_opt = self.var();
                let var_expr= self.some_or_diagnostic(var_opt, self.err_unexpected_token());

                if var_expr.is_none() {
                    let cur = self.advance();
                    self.logger.log_brief_error(&format!("Reached bottom of parser stack. Skipping token {:?} and giving up", cur));
                }

                self.log_parse_result(&var_expr, "var expression");
                Some(Box::new(var_expr?))
            } 
        }
    }
}