use std::fmt;

use crate::ast::{ASTNode, member_access::{AccessType, MemberAccess}};
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{arguments::ArgumentsRule, expr::ExprRule, primary::PrimaryRule};
use crate::lexer::token::TokenType;

pub struct MemberAccessRule {}

impl fmt::Display for MemberAccessRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MemberAccess")
    }
}

impl ParseRule<Box<dyn ASTNode>> for MemberAccessRule {
    fn check_match(&self, _cursor: ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.begin_range();
        let expr = parser.apply_rule(PrimaryRule {}, "member access expression", Some(ErrMsg::ExpectedExpression))?;

        let mut member_accesses = Vec::new();

        parser.log_debug(&format!("Parsing member access for expression at token {:?}", parser.cur()));

        while let Some(token) = parser.try_match(&[TokenType::Dot, TokenType::Arrow, TokenType::LeftSquare, TokenType::LeftParen]) {
            
            if token.value == TokenType::Dot {
                parser.next();
                let identifier = parser.try_consume(TokenType::AnyIdentifier)?;

                member_accesses.push(AccessType::Direct(identifier.unwrap_identifier()));
            } else if token.value == TokenType::Arrow {
                parser.next();

                let identifier = parser.try_consume(TokenType::AnyIdentifier)?;

                member_accesses.push(AccessType::Indirect(identifier.unwrap_identifier()));
            } else if token.value == TokenType::LeftSquare {
                parser.next();

                let index_expr = parser.apply_rule(ExprRule {}, "array index expression", Some(ErrMsg::ExpectedExpression))?;

                parser.consume_or_diagnostic(TokenType::RightSquare);

                member_accesses.push(AccessType::Array(index_expr));
            } else if token.value == TokenType::LeftParen {
                let args = parser.apply_rule(ArgumentsRule {}, "function call arguments", Some(ErrMsg::ExpectedArguments))?;

                member_accesses.push(AccessType::FunctionCall(args));
            }
        }
        
        Some(Box::new(MemberAccess::new(expr, member_accesses, parser.end_range())))
    }
}