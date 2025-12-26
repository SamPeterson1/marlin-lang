use std::fmt;

use crate::ast::ConstructorItem;
use crate::diagnostic::ErrMsg;
use crate::parser::{ExprParser, ParseRule, ParserCursor, TokenCursor};
use crate::parser::rules::{block::BlockRule, parameters::ParametersRule};
use crate::lexer::token::TokenType;

pub struct ConstructorRule {}

impl fmt::Display for ConstructorRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constructor")
    }
}

impl ParseRule<ConstructorItem> for ConstructorRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::DollarSign).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ConstructorItem> {
        parser.begin_range();
        parser.try_consume(TokenType::DollarSign)?;

        let parameters = parser.apply_rule(ParametersRule {}, "constructor parameters", Some(ErrMsg::ExpectedParameters))?;
        let body = parser.apply_rule(BlockRule {}, "constructor body", Some(ErrMsg::ExpectedBlock))?;        
        
        Some(ConstructorItem::new(parameters, body, parser.end_range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType, PositionRange};
    use crate::parser::ExprParser;

}