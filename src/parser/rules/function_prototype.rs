use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper, function_prototype::FunctionPrototype}, logger::Log, parser::{ExprParser, ParseRule, ParserCursor, TokenCursor, diagnostic::ErrMsg, rules::{parameters::ParametersRule, parsed_type::ParsedTypeRule, unary::UnaryRule}}, token::{PositionRange, TokenType}};

pub struct FunctionPrototypeRule {}

impl fmt::Display for FunctionPrototypeRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FunctionPrototype")
    }
}

impl ParseRule<ASTWrapper<FunctionPrototype>> for FunctionPrototypeRule {
    fn check_match(&self, mut cursor: ParserCursor) -> bool {
        cursor.try_consume(TokenType::Fn).is_some()
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<FunctionPrototype>> {
        let fn_token = parser.try_consume(TokenType::Fn)?;

        let name = parser.consume_or_diagnostic(TokenType::Identifier)?.get_string().to_string();

        let parameters = parser.apply_rule(ParametersRule {}, "function parameters", Some(ErrMsg::ExpectedParameters))?;

        parser.consume_or_diagnostic(TokenType::Arrow);

        let ret_type = parser.apply_rule(ParsedTypeRule {}, "return type", None)?;

        let position = PositionRange::concat(&fn_token.position, &ret_type.position);

        Some(ASTWrapper::new_function_prototype(name, parameters, ret_type, position))
    }
}