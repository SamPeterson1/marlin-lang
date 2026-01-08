use std::fmt;

use crate::ast::DeclarationExpr;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{expr::ExprRule, parsed_type::ParsedTypeRule};
use crate::lexer::token::TokenType;

pub struct DeclarationRule {
    pub use_let: bool,
}

impl fmt::Display for DeclarationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Declaration")
    }
}

impl ParseRule<DeclarationExpr> for DeclarationRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        !self.use_let || cursor.try_consume(TokenType::Let).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<DeclarationExpr> {
        parser.begin_range();
        
        if self.use_let {
            parser.try_consume(TokenType::Let)?;
        }

        let declaration_type = parser.apply_rule(ParsedTypeRule {}, "declaration type", Some(ErrMsg::ExpectedType))?;
        let declaration_name = parser.consume_or_diagnostic(TokenType::AnyIdentifier)?.unwrap_identifier();

        let expr = if parser.try_consume(TokenType::Assignment).is_some() {
            let expr = parser.apply_rule(ExprRule {}, "declaration expression", Some(ErrMsg::ExpectedExpression))?;

            Some(expr)
        } else {
            None
        };

        if self.use_let {
            parser.consume_or_diagnostic(TokenType::Semicolon);
        }

        Some(DeclarationExpr::new(declaration_name, declaration_type, expr, parser.end_range()))
    }    
}