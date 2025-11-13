use crate::{item::StructItem, logger::Log, parser::{ExprParser, ParseRule, diagnostic}, token::{Position, PositionRange, TokenType}};
use std::fmt;
use std::collections::HashMap;

pub struct StructRule {}

impl fmt::Display for StructRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Struct")
    }
}

impl ParseRule<StructItem> for StructRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<StructItem> {
        let struct_token = parser.advance();

        let struct_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_struct_name(PositionRange::new(Position::new(0, 0))))
            .map(|x| x.get_string().to_string());

        parser.log_parse_result(&struct_name, "struct name");

        parser.consume_or_diagnostic(TokenType::LeftCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftCurly));

        let mut members = HashMap::new();

        loop {
            let opt_type = parser.try_type();
            let member_type = parser.some_or_diagnostic(opt_type, diagnostic::err_expected_member_name(PositionRange::new(Position::new(0, 0)), ));
            let member_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_member_name(PositionRange::new(Position::new(0, 0))))
                .map(|x| x.get_string().to_string());

            parser.log_parse_result(&member_type, "member type");
            parser.log_parse_result(&member_name, "member name");

            if let (Some(member_type), Some(member_name)) = (member_type, member_name) {
                members.insert(member_name, member_type);
            }

            if parser.try_consume(TokenType::Comma).is_none() {
                parser.log_debug(&format!("Done parsing struct members"));
                parser.consume_or_diagnostic(TokenType::RightCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightCurly));
                break;
            }
        }

        let position = PositionRange::concat(&struct_token.position, &parser.prev().position);

        Some(StructItem::new(struct_name?, members, position))
    }
}