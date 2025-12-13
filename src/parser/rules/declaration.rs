use std::fmt;

use crate::ast::declaration_expr::DeclarationExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct DeclarationRule {}

impl fmt::Display for DeclarationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Declaration")
    }
}

impl ParseRule<DeclarationExpr> for DeclarationRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Let).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<DeclarationExpr> {
        parser.begin_range();
        parser.try_consume(TokenType::Let)?;

        let declaration_type = parser.apply_rule(ParsedTypeRule {}, "declaration type", Some(ErrMsg::ExpectedType))?;
        let declaration_name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        parser.consume_or_diagnostic(TokenType::Assignment)?;

        let expr = parser.apply_rule(ExprRule {}, "declaration expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        Some(DeclarationExpr::new(declaration_name, declaration_type, expr, parser.end_range()))
    }    
}
