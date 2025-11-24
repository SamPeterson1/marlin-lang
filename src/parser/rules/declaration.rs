use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, declaration_expr::DeclarationExpr}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::{self, ErrMsg}, rules::{assignment::AssignmentRule, expr::ExprRule, parsed_type::ParsedTypeRule}}, token::{Position, PositionRange, TokenType}};

pub struct DeclarationRule {}

impl fmt::Display for DeclarationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Declaration")
    }
}

impl ParseRule<ASTWrapper<DeclarationExpr>> for DeclarationRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Let).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<DeclarationExpr>> {
        let let_token = parser.try_consume(TokenType::Let)?;
        let declaration_type = parser.apply_rule(ParsedTypeRule {}, "declaration type", Some(ErrMsg::ExpectedType))?;
        let declaration_name = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();

        parser.consume_or_diagnostic(TokenType::Assignment)?;

        let expr = parser.apply_rule(ExprRule {}, "declaration expression", Some(ErrMsg::ExpectedExpression))?;

        parser.consume_or_diagnostic(TokenType::Semicolon);

        let position = PositionRange::concat(&let_token.position, &parser.prev().position);

        parser.declaration_expr_id_counter += 1;
        Some(ASTWrapper::new_declaration(parser.declaration_expr_id_counter, declaration_name, declaration_type, expr, position))
    }    
}
