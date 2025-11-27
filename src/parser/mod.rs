pub mod diagnostic;
mod rules;

use std::collections::VecDeque;

use serde::Serialize;

use crate::ast::{ASTNode, ASTWrapper};

use crate::ast::program::Program;
use crate::logger::{Log, LogLevel, LogSource};
use crate::parser::diagnostic::{Diagnostic, DiagnosticSeverity, ErrMsg};
use crate::parser::rules::program::ProgramRule;
use crate::{logger::{Logger}, token::{Token, TokenType}};

pub struct ExprParser<'a> {
    ptr: usize,
    tokens: &'a[Token],
    diagnostics: Vec<Diagnostic>,
    var_expr_id_counter: i32,
    declaration_expr_id_counter: i32,
    parser_stack: VecDeque<String>,
}

trait TokenCursor {
    fn cur(&self) -> Token;
    fn peek(&self) -> Token;
    fn prev(&self) -> Token;
    fn next(&mut self) -> Token;

    fn try_consume(&mut self, token: TokenType) -> Option<Token> {
        if self.cur().token_type == token {
            Some(self.next())
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.cur().token_type == TokenType::EOF
    }

    fn try_match(&mut self, matches: &[TokenType]) -> Option<Token> {
        for token_match in matches {
            if self.cur().token_type == *token_match {
                return Some(self.cur());
            }
        }

        return None;
    }

    fn try_consume_match(&mut self, matches: &[TokenType]) -> Option<Token> {
        for token_match in matches {
            if self.cur().token_type == *token_match {
                return Some(self.next());
            }
        }

        return None;
    }
}

pub struct ParserCursor<'tok> {
    ptr: usize,
    tokens: &'tok[Token],
}

impl TokenCursor for ExprParser<'_> {
    fn cur(&self) -> Token {
        self.tokens[self.ptr].clone()
    }

    fn peek(&self) -> Token {
        if self.ptr + 1 >= self.tokens.len() {
            return self.tokens[self.ptr].clone();
        }

        self.tokens[self.ptr + 1].clone()
    }

    fn next(&mut self) -> Token {
        let ret = &self.tokens[self.ptr];

        if self.ptr + 1 < self.tokens.len() {
            self.ptr += 1;
        }

        ret.clone()
    }

    fn prev(&self) -> Token {
        self.tokens[self.ptr - 1].clone()
    }
}

impl TokenCursor for ParserCursor<'_> {
    fn cur(&self) -> Token {
        self.tokens[self.ptr].clone()
    }

    fn peek(&self) -> Token {
        if self.ptr + 1 >= self.tokens.len() {
            return self.tokens[self.ptr].clone();
        }

        self.tokens[self.ptr + 1].clone()
    }

    fn next(&mut self) -> Token {
        let ret = self.tokens[self.ptr].clone();

        if self.ptr + 1 < self.tokens.len() {
            self.ptr += 1;
        }

        ret
    }

    fn prev(&self) -> Token {
        self.tokens[self.ptr - 1].clone()
    }
}

pub struct ParseResult {
    pub program: ASTWrapper<Program>,
    pub diagnostics: Vec<Diagnostic>
}

trait ParseRule<T>: std::fmt::Display {
    fn check_match(&self, cursor: ParserCursor) -> bool;
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

impl ExprParser<'_> {
    pub fn new(tokens: &[Token]) -> ExprParser {
        ExprParser {
            ptr: 1, 
            tokens, 
            diagnostics: Vec::new(), 
            var_expr_id_counter: 0, 
            declaration_expr_id_counter: 0, 
            parser_stack: VecDeque::new()            ,
        }
    }

    fn some_or_diagnostic<T>(&mut self, opt: Option<T>, diagnostic: Diagnostic) -> Option<T> {
        if opt.is_none() {
            self.push_diagnostic(diagnostic);
        }

        opt
    }
    
    fn consume_or_diagnostic(&mut self, token: TokenType) -> Option<Token> {
        let cur = &self.cur();
        
        if cur.token_type != token {
            self.push_diagnostic(ErrMsg::ExpectedToken(token).make_diagnostic(cur.position));
            
            None
        } else {
            Some(self.next())
        }
    }

    fn unwrap_or_diagnostic<T>(&mut self, result: Result<T, Diagnostic>) -> Option<T> {
        match result {
            Ok(x) => Some(x),
            Err(diagnostic) => {
                self.push_diagnostic(diagnostic);
                None
            }
        }
    } 

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        let log_severity = match diagnostic.severity {
            DiagnosticSeverity::Error => LogLevel::Error,
            DiagnosticSeverity::Warning => LogLevel::Warning,
        };

        Logger::log(self, log_severity, &format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }

    fn get_cursor(&self) -> ParserCursor {
        ParserCursor { 
            ptr: self.ptr, 
            tokens: self.tokens
        }
    }

    fn apply_rule_boxed<T: ASTNode + Serialize + 'static>(&mut self, rule: impl ParseRule<T>, purpose: &str, err_msg: Option<ErrMsg>) -> Option<Box<dyn ASTNode>> {
        Some(Box::new(self.apply_rule(rule, purpose, err_msg)?))
    }

    fn apply_rule<T: Serialize>(&mut self, rule: impl ParseRule<T>, purpose: &str, err_msg: Option<ErrMsg>) -> Option<T> {
        self.log_debug(&format!("Entering rule {} for {}. Current token {:?}", rule, purpose, self.cur()));


        if rule.check_match(self.get_cursor()) {
            self.log_debug(&format!("Initial match satisfied for {}", purpose));
            
            self.parser_stack.push_front(format!("{}", rule));

            let result = rule.parse(self);

            self.parser_stack.pop_front();

            match &result {
                Some(result) => self.log_debug(&format!("Match succeeded for {}, result {}", purpose, serde_json::to_string(&result).unwrap())),
                None => self.log_error(&format!("Match failed for {}", purpose))
            }


            result
        } else {
            self.log_debug(&format!("Initial match not satisfied for {}", purpose));

            self.parser_stack.pop_front();

            if let Some(err_msg) = err_msg {
                self.push_diagnostic(err_msg.make_diagnostic(self.cur().position));
            }

            None
        } 
    }

    pub fn parse(mut self) -> ParseResult {
        Logger::log_info(&self, "Beginning parser");

        let program = self.apply_rule(ProgramRule {}, "program", None).unwrap();

        ParseResult {
            program,
            diagnostics: self.diagnostics
        }
    }
}