use std::fmt;

use crate::{ast::{ASTWrapper, function_item::FunctionItem}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::block::BlockRule}, token::{Position, PositionRange, TokenType}, types::parsed_type::ParsedType};

pub struct FunctionRule {}

impl fmt::Display for FunctionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function")
    }
}

impl ParseRule<ASTWrapper<FunctionItem>> for FunctionRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<FunctionItem>> {
        let fn_token = parser.advance();

        let function_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_fn_name(PositionRange::new(Position::new(0, 0))))
            .map(|x| x.get_string().to_string());

        parser.log_parse_result(&function_name, "function name");

        parser.consume_or_diagnostic(TokenType::LeftParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftParen));

        let mut args = Vec::new();

        loop {
            if parser.try_consume(TokenType::RightParen).is_some() {
                parser.log_debug(&format!("Done parsing function arguments"));
                break;
            }

            let opt_type = parser.try_type();
            let arg_type = parser.some_or_diagnostic(opt_type, diagnostic::err_expected_arg_type(PositionRange::new(Position::new(0, 0))));

            let arg_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_arg_name(PositionRange::new(Position::new(0, 0))))
                .map(|x| x.get_string().to_string());

            parser.log_parse_result(&arg_type, "argument type");
            parser.log_parse_result(&arg_name, "argument name");

            if let (Some(arg_type), Some(arg_name)) = (arg_type, arg_name) {
                args.push((arg_name, arg_type));
            }

            if parser.try_consume(TokenType::Comma).is_none() {
                parser.log_debug(&format!("Done parsing function arguments"));
                parser.consume_or_diagnostic(TokenType::RightParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightParen));
                break;
            }
        }

        let return_type = if parser.try_consume(TokenType::Arrow).is_some() {
            let opt_type = parser.try_type();
            parser.some_or_diagnostic(opt_type, diagnostic::err_expected_return_type(PositionRange::new(Position::new(0, 0))))
        } else {
            Some(ParsedType::Empty)
        };

        parser.log_parse_result(&return_type, "return type");

        let block = parser.apply_rule(BlockRule {});

        parser.log_parse_result(&block, "function block");

        let position = PositionRange::concat(&fn_token.position, &parser.prev().position);

        Some(ASTWrapper::new_function(function_name?, args, Box::new(block?), return_type?, position))
    }
}