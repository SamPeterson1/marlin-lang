use crate::{expr::{Expr, struct_initializer_expr::StructInitializerExpr, unary_expr::UnaryExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::expr::ExprRule}, token::{Position, PositionRange, TokenType}};
use std::{collections::HashMap, fmt};

pub struct StructInitializerRule {}

impl fmt::Display for StructInitializerRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StructInitializer")
    }
}

//struct_initializer: IDENTIFIER LEFT_CURLY [member_intializer] (COMMA, [member_intializer])* RIGHT_CURLY
//member_intializer: IDENTIFIER COLON [inline_expression]
impl ParseRule<StructInitializerExpr> for StructInitializerRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<StructInitializerExpr> {
        parser.log_debug(&format!("Entering struct initializer parser. Current token {:?}", parser.cur()));

        let type_name_token = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_struct_name(PositionRange::new(Position::new(0, 0))));
        let type_name = type_name_token.as_ref().map(|x| x.get_string().to_string());

        parser.log_parse_result(&type_name, "struct type name");

        parser.consume_or_diagnostic(TokenType::LeftCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::LeftCurly));

        let mut member_inits = HashMap::new();

        loop {
            let member_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_member_name(PositionRange::new(Position::new(0, 0))))
                .map(|x| x.get_string().to_string());
            parser.log_parse_result(&member_name, "member name");

            parser.consume_or_diagnostic(TokenType::Colon, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Colon));

            let expr = parser.apply_rule(ExprRule {});
            parser.log_parse_result(&expr, "member expression");

            member_inits.insert(member_name?, expr?);

            if parser.try_consume(TokenType::Comma).is_none() {
                parser.log_debug(&format!("Done parsing struct member initializers"));
                parser.consume_or_diagnostic(TokenType::RightCurly, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightCurly));
                break;
            }
        }

        let position = PositionRange::concat(&type_name_token?.position, &parser.prev().position);

        Some(StructInitializerExpr::new(type_name?, member_inits, position))
    }
}