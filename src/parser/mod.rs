mod rules;

use std::collections::VecDeque;

use serde::Serialize;

use crate::ast::ASTNode;

use crate::ast::Program;
use crate::logger::Log;
use crate::diagnostic::{Diagnostic, ErrMsg};
use crate::parser::rules::program::ProgramRule;
use crate::lexer::token::{PositionRange, Positioned};
use crate::lexer::token::{Token, TokenType};

pub struct ExprParser<'diag> {
    ptr: usize,
    tokens: Vec<Token>,
    diagnostics: &'diag mut Vec<Diagnostic>,
    rule_stack: VecDeque<String>,
    position_stack: VecDeque<PositionRange>,
}

trait TokenCursor {
    fn cur(&self) -> Token;
    fn next(&mut self) -> Token;

    fn try_consume(&mut self, token: TokenType) -> Option<Token> {
        if self.cur().value == token {
            Some(self.next())
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.cur().value == TokenType::EOF
    }

    fn try_match(&self, matches: &[TokenType]) -> Option<Token> {
        for token_match in matches {
            if self.cur().value == *token_match {
                return Some(self.cur());
            }
        }

        return None;
    }

    fn try_consume_match(&mut self, matches: &[TokenType]) -> Option<Token> {
        for token_match in matches {
            if self.cur().value == *token_match {
                return Some(self.next());
            }
        }

        return None;
    }
}

#[derive(Clone, Copy)]
pub struct ParserCursor<'tok> {
    ptr: usize,
    tokens: &'tok[Token],
}

impl TokenCursor for ExprParser<'_> {
    fn cur(&self) -> Token {
        self.tokens[self.ptr].clone()
    }

    fn next(&mut self) -> Token {
        let ret = &self.tokens[self.ptr];

        if self.ptr + 1 < self.tokens.len() {
            self.ptr += 1;
        }

        ret.clone()
    }
}

impl TokenCursor for ParserCursor<'_> {
    fn cur(&self) -> Token {
        self.tokens[self.ptr].clone()
    }

    fn next(&mut self) -> Token {
        let ret = self.tokens[self.ptr].clone();

        if self.ptr + 1 < self.tokens.len() {
            self.ptr += 1;
        }

        ret
    }
}

trait ParseRule<T>: std::fmt::Display {
    fn check_match(&self, cursor: ParserCursor) -> bool;
    fn parse(&self, parser: &mut ExprParser) -> Option<T>;
}

impl Log for ExprParser<'_> {
    fn get_source(&self) -> String {
        let mut stack = self.rule_stack.iter().take(10).rev().map(|x| x.to_string()).collect::<Vec<String>>().join(">");

        if self.rule_stack.len() > 10 {
            stack = format!("{} others...>{}", self.rule_stack.len() - 10, stack);
        }

        format!("ExprParser: {}", stack)
    }
}

impl<'diag> ExprParser<'diag> {
    pub fn new(tokens: Vec<Token>, diagnostics: &'diag mut Vec<Diagnostic>) -> ExprParser<'diag> {
        ExprParser {
            ptr: 0, 
            tokens, 
            diagnostics,
            rule_stack: VecDeque::new(),
            position_stack: VecDeque::new(),
        }
    }

    fn consume_or_diagnostic(&mut self, token: TokenType) -> Option<Token> {
        let cur = &self.cur();
        
        if cur.value != token {
            self.push_diagnostic(ErrMsg::ExpectedToken(token).make_diagnostic(*cur.get_position()));
            
            None
        } else {
            Some(self.next())
        }
    }

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        let log_severity = diagnostic.severity.into();

        self.log(log_severity, &format!("Pushing diagnostic: {}", diagnostic));
        self.diagnostics.push(diagnostic);
    }

    fn get_cursor(&self) -> ParserCursor<'_> {
        ParserCursor { 
            ptr: self.ptr, 
            tokens: &self.tokens
        }
    }

    fn begin_range(&mut self) {
        self.position_stack.push_front(*self.cur().get_position());
    }

    fn current_range(&self) -> PositionRange {
        let prev_position = *self.tokens[if self.ptr == 0 { 0 } else { self.ptr - 1 }].get_position();

        if let Some(start) = self.position_stack.front() {
            PositionRange::concat(&start, &prev_position)
        } else {
            prev_position
        }
    }

    fn end_range(&mut self) -> PositionRange {
        let prev_position = *self.tokens[if self.ptr == 0 { 0 } else { self.ptr - 1 }].get_position();

        if let Some(start) = self.position_stack.pop_front() {
            PositionRange::concat(&start, &prev_position)
        } else {
            prev_position
        }
    }

    fn apply_rule_boxed<T: ASTNode + Serialize + 'static>(&mut self, rule: impl ParseRule<T>, purpose: &str, err_msg: Option<ErrMsg>) -> Option<Box<dyn ASTNode>> {
        Some(Box::new(self.apply_rule(rule, purpose, err_msg)?))
    }

    fn apply_rule<T: Serialize>(&mut self, rule: impl ParseRule<T>, purpose: &str, err_msg: Option<ErrMsg>) -> Option<T> {
        self.position_stack.push_front(*self.cur().get_position());
        self.log_debug(&format!("Entering rule {} for {}. Current token {:?}", rule, purpose, self.cur()));

        if rule.check_match(self.get_cursor()) {
            self.log_debug(&format!("Initial match satisfied for {}", purpose));
            
            self.rule_stack.push_front(format!("{}", rule));
            let result = rule.parse(self);
            self.rule_stack.pop_front();

            match &result {
                Some(result) => self.log_debug(&format!("Match succeeded for {}, result {}", purpose, serde_json::to_string(&result).unwrap())),
                _ => self.log_error(&format!("Match failed for {}", purpose))
            }


            result
        } else {
            self.log_debug(&format!("Initial match not satisfied for {}", purpose));

            self.rule_stack.pop_front();

            if let Some(err_msg) = err_msg {
                self.push_diagnostic(err_msg.make_diagnostic(*self.cur().get_position()));
            }

            None
        } 
    }

    pub fn parse(mut self) -> Program {
        self.log_info("Beginning parser");

        let program: Program = self.apply_rule(ProgramRule {}, "program", None).unwrap();

        self.log_info("Parser finished");

        program
    }
}