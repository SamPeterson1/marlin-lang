use crate::{ast::{ASTNode, ASTWrapper, member_access::AccessType}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{arguments::ArgumentsRule, expr::ExprRule, primary::PrimaryRule}}, token::{PositionRange, TokenType}};
use std::fmt;

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
        let expr = parser.apply_rule(PrimaryRule {}, "member access expression", Some(ErrMsg::ExpectedExpression))?;

        let mut member_accesses = Vec::new();

        parser.log_debug(&format!("Parsing member access for expression at token {:?}", parser.cur()));

        while let Some(token) = parser.try_match(&[TokenType::Dot, TokenType::Arrow, TokenType::LeftSquare, TokenType::LeftParen]) {
            
            if token.token_type == TokenType::Dot {
                parser.next();
                let identifier = parser.try_consume(TokenType::Identifier)?;

                member_accesses.push(AccessType::Direct(identifier.get_string().to_string()));
            } else if token.token_type == TokenType::Arrow {
                parser.next();

                let identifier = parser.try_consume(TokenType::Identifier)?;

                member_accesses.push(AccessType::Indirect(identifier.get_string().to_string()));
            } else if token.token_type == TokenType::LeftSquare {
                parser.next();

                let index_expr = parser.apply_rule(ExprRule {}, "array index expression", Some(ErrMsg::ExpectedExpression))?;

                parser.consume_or_diagnostic(TokenType::RightSquare);

                member_accesses.push(AccessType::Array(index_expr));
            } else if token.token_type == TokenType::LeftParen {
                let args = parser.apply_rule(ArgumentsRule {}, "function call arguments", Some(ErrMsg::ExpectedArguments))?;

                member_accesses.push(AccessType::FunctionCall(args));
            }
        }

        let position = PositionRange::concat(expr.get_position(), &parser.prev().position);
        
        Some(Box::new(ASTWrapper::new_member_access(expr, member_accesses, position)))
    }
}