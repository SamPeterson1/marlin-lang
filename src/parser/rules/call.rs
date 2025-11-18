use std::fmt;

use crate::{ast::{ASTNode, ASTWrapper}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::{inline_expr::InlineExprRule, primary::PrimaryRule}}, token::{Position, PositionRange, TokenType}};

pub struct CallRule {}

impl fmt::Display for CallRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Call")
    }
}

//call: IDENTIFIER LEFT_PAREN (([inline_expression], COMMA)* [inline_expression]?) | [primary]
impl ParseRule<Box<dyn ASTNode>> for CallRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        parser.log_debug(&format!("Entering call parser. Current token {:?}", parser.cur()));

        if parser.cur().token_type == TokenType::Identifier {
            if parser.peek().token_type == TokenType::LeftParen {
                let function_name = parser.advance().get_string().to_string();
                parser.log_debug(&format!("Parsed function name: {}", function_name));

                parser.advance();

                let mut args: Vec<Box<dyn ASTNode>> = Vec::new();

                loop {
                    let arg = parser.apply_rule(InlineExprRule {});
                    parser.log_parse_result(&arg, "call arg");

                    if let Some(arg) = arg {
                        args.push(arg);
                    }

                    if parser.try_consume(TokenType::Comma).is_none() {
                        parser.log_debug(&format!("Done parsing call args"));
                        parser.consume_or_diagnostic(TokenType::RightParen, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::RightParen));
                        break;
                    }
                }

                Some(Box::new(ASTWrapper::new_call(function_name, args)))
            } else {
                parser.apply_rule(PrimaryRule {})
            }
        } else {
            parser.apply_rule(PrimaryRule {})
        }
    }
}
