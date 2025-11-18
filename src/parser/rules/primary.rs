use crate::{expr::{ASTNode, ASTWrapper, get_address_expr::GetAddressExpr, literal_expr::{Literal, LiteralExpr}, loop_expr::LoopExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::{expr::ExprRule, var::VarRule}}, token::{Position, PositionRange, TokenType, TokenValue}, types::parsed_type::{ParsedPointerType, ParsedType}};
use std::{fmt, rc::Rc};

pub struct PrimaryRule {}

impl fmt::Display for PrimaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Primary")
    }
}

//primary: &?[var] | [literal] | LEFT_PAREN inline_expression RIGHT_PAREN
impl ParseRule<Box<dyn ASTNode>> for PrimaryRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering primary parser. Current token {:?}", parser.cur()));
        let cur = parser.cur();

        match (cur.token_type, cur.value) {
            (TokenType::IntLiteral, TokenValue::Int(value)) => {
                parser.log_debug(&format!("Parsed int literal: {}", value));
                parser.advance();
                Some(Box::new(ASTWrapper::new_literal(Literal::Int(value), ParsedType::Integer, PositionRange::new(Position::new(0, 0)))))
            },
            (TokenType::DoubleLiteral, TokenValue::Double(value)) => {
                parser.log_debug(&format!("Parsed double literal: {}", value));
                parser.advance();
                Some(Box::new(ASTWrapper::new_literal(Literal::Double(value), ParsedType::Double, PositionRange::new(Position::new(0, 0)))))
            },
            (TokenType::BoolLiteral, TokenValue::Bool(value)) => {
                parser.log_debug(&format!("Parsed bool literal: {}", value));
                parser.advance();
                Some(Box::new(ASTWrapper::new_literal(Literal::Bool(value), ParsedType::Boolean, PositionRange::new(Position::new(0, 0)))))
            },
            (TokenType::StringLiteral, TokenValue::String(value)) => {
                parser.log_debug(&format!("Parsed string literal: {}", value));
                parser.advance();
                Some(Box::new(ASTWrapper::new_literal(Literal::String(value), ParsedType::Pointer(ParsedPointerType {pointee: Rc::new(ParsedType::Integer)}), PositionRange::new(Position::new(0, 0)))))
            },
            (TokenType::Ampersand, TokenValue::None) => {
                parser.advance();
                let var_opt = parser.apply_rule(VarRule {});
                let var_expr = parser.some_or_diagnostic(var_opt, diagnostic::err_expected_var(PositionRange::new(Position::new(0, 0))));
                parser.log_parse_result(&var_expr, "get address var expression");
                Some(Box::new(ASTWrapper::new_get_address(var_expr?, PositionRange::new(Position::new(0, 0)))))
            },
            (TokenType::LeftParen, TokenValue::None) => {
                parser.advance();

                let expr = parser.apply_rule(ExprRule {});
                parser.log_parse_result(&expr, "parenthesized expression");

                parser.consume_or_diagnostic(TokenType::RightParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightParen));

                expr
            },
            _ => {
                let var_opt = parser.apply_rule(VarRule {});
                let var_expr= parser.some_or_diagnostic(var_opt, diagnostic::err_unexpected_token(PositionRange::new(Position::new(0, 0))));

                if var_expr.is_none() {
                    let cur = parser.advance();
                    parser.log_error(&format!("Reached bottom of parser stack. Skipping token {:?} and giving up", cur));
                }

                parser.log_parse_result(&var_expr, "var expression");
                Some(Box::new(var_expr?))
            } 
        }
    }
}