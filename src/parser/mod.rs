mod diagnostic;
mod rules;

use std::collections::VecDeque;
use std::rc::Rc;

use serde::Serialize;

use crate::ast::ASTNode;

use crate::logger::{Log, LogLevel, LogSource};
use crate::parser::rules::item::ItemRule;
use crate::types::parsed_type::{ParsedPointerType, ParsedType, ParsedTypeName};
use crate::{error::{Diagnostic, DiagnosticType}, logger::{Logger}, token::{Token, TokenType, TokenValue}};

pub struct ExprParser<'a> {
    ptr: usize,
    ptr_stack: VecDeque<usize>,
    tokens: &'a[Token],
    diagnostics: Vec<Diagnostic>,
    var_expr_id_counter: i32,
    declaration_expr_id_counter: i32,
    parser_stack: VecDeque<String>,
}

pub struct ParseResult {
    pub items: Vec<Box<dyn ASTNode>>,
    pub diagnostics: Vec<Diagnostic>
}

trait ParseRule<T>: std::fmt::Display {
    fn parse(&self, parser: &mut ExprParser) -> Option<T>;
}

impl LogSource for ExprParser<'_> {
    fn get_source(&self) -> String {
        let mut stack = self.parser_stack.iter().take(10).rev().map(|x| x.to_string()).collect::<Vec<String>>().join(">");

        if self.parser_stack.len() > 10 {
            stack = format!("{} others...>{}", self.parser_stack.len() - 10, stack);
        }

        format!("ExprParser: {}", stack)
    }
}

impl Log for ExprParser<'_> {}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &[Token]) -> ExprParser {
        ExprParser {
            ptr: 1, 
            ptr_stack: VecDeque::new(),
            tokens, 
            diagnostics: Vec::new(), 
            var_expr_id_counter: 0, 
            declaration_expr_id_counter: 0, 
            parser_stack: VecDeque::new()            ,
        }
    }

    //Pop stack without resetting self.ptr
    pub fn commit_ptr(&mut self) {
        self.ptr_stack.pop_front();
    }

    pub fn push_ptr(&mut self) {
        self.ptr_stack.push_front(self.ptr);
    }

    pub fn pop_ptr(&mut self) {
        self.ptr = self.ptr_stack.pop_front().unwrap();
    }

    fn apply_rule_boxed<T: ASTNode + 'static>(&mut self, rule: impl ParseRule<T>) -> Option<Box<dyn ASTNode>> {
        Some(Box::new(self.apply_rule(rule)?))
    }

    fn apply_rule<T>(&mut self, rule: impl ParseRule<T>) -> Option<T> {
        self.parser_stack.push_front(format!("{}", rule));
        self.log_debug(&format!("Entering rule {}", rule));

        let result = rule.parse(self);

        self.log_debug(&format!("Exiting rule {}", rule));
        self.parser_stack.pop_front();

        result
    }

    fn log_parse_result(&self, expr: &Option<impl Serialize>, name: &str) {
        if let Some(expr) = expr {
            Logger::log_debug(self, &format!("Parsed {}: {}", name, serde_json::to_string(expr).unwrap()));
        } else {
            Logger::log_error(self, &format!("Failed to parse {}", name));
        }
    }

    pub fn parse(mut self) -> ParseResult {
        Logger::log_info(&self, "Beginning parser");

        let mut items = Vec::new();
        
        while !self.is_at_end() {
            if let Some(expr) = self.apply_rule(ItemRule {}) {
                Logger::log_info(&self, &format!("Parsed item successfully"));
                Logger::log_debug(&self, &format!("Parsed item: {}", serde_json::to_string(&expr).unwrap()));
                items.push(expr);
            } else {
                Logger::log_error(&self, &format!("Failed to parse item. Current token: {:?}", self.cur()));
            }
        }

        Logger::log_info(&self, &format!("Parsed {} items with {} diagnostics", items.len(), self.diagnostics.len()));

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
            DiagnosticType::Error => LogLevel::Error,
            DiagnosticType::Warning => LogLevel::Warning,
        };

        Logger::log(self, log_severity, &format!("Pushing diagnostic: {}", diagnostic));
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
}