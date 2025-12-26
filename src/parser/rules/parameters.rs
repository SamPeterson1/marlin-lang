use std::fmt;

use crate::ast::DeclarationExpr;
use crate::diagnostic::ErrMsg;
use crate::logger::Log;
use crate::parser::rules::declaration::DeclarationRule;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::parsed_type::ParsedTypeRule;
use crate::lexer::token::TokenType;

pub struct ParametersRule {}

impl fmt::Display for ParametersRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parameters")
    }
}

impl ParseRule<Vec<DeclarationExpr>> for ParametersRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::LeftParen).is_some()
    }
    
    fn parse(&self, parser: &mut ExprParser) -> Option<Vec<DeclarationExpr>> {
        parser.begin_range();
        parser.try_consume(TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        
        if parser.cur().value != TokenType::RightParen {
            let declaration_expr = parser.apply_rule(DeclarationRule { use_let: false }, "first parameter declaration", None)?;
            parameters.push(declaration_expr);
        }

        parser.log_debug(&format!("Current token after first parameter parse: {:?}", parser.cur()));

        while let Some(_) = parser.try_consume(TokenType::Comma) {
            let declaration_expr = parser.apply_rule(DeclarationRule { use_let: false }, "parameter declaration", None)?;
            parameters.push(declaration_expr);
        }

        parser.consume_or_diagnostic(TokenType::RightParen);

        Some(parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};

}