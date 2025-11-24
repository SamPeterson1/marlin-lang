use crate::{ast::{ASTNode, ASTWrapper, literal_expr::Literal, parsed_type::{ParsedType, ParsedUnitType}}, logger::Log, parser::{ExprParser, ParseRule, TokenCursor, diagnostic::{self, ErrMsg}, rules::{block::BlockRule, call::CallRule, expr::ExprRule, for_loop::ForLoopRule, getc::GetcRule, if_block::IfBlockRule, loop_expr::LoopRule, new_array::NewArrayRule, var::VarRule, while_loop::WhileLoopRule}}, token::{Position, PositionRange, TokenType, TokenValue}};
use std::{arch::x86_64, fmt, rc::Rc};

pub struct PrimaryRule {}

impl fmt::Display for PrimaryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Primary")
    }
}

impl ParseRule<Box<dyn ASTNode>> for PrimaryRule {
    fn check_match(&self, cursor: crate::parser::ParserCursor) -> bool {
        true
    }

    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        if (CallRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(CallRule {}, "primary call", None);
        }

        if (GetcRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(GetcRule {}, "primary getc", None);
        }
        
        if (NewArrayRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(NewArrayRule {}, "primary new array", None);
        }

        if (LoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(LoopRule {}, "primary loop", None);
        }

        if (IfBlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(IfBlockRule {}, "primary if", None);
        }

        if (BlockRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(BlockRule {}, "primary block", None);
        }

        if (ForLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(ForLoopRule {}, "primary for", None);
        }

        if (WhileLoopRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(WhileLoopRule {}, "primary while", None);
        }

        if parser.try_consume(TokenType::LeftParen).is_some() {
            let expr = parser.apply_rule(ExprRule {}, "primary grouped expression", Some(ErrMsg::ExpectedExpression))?;
            parser.consume_or_diagnostic(TokenType::RightParen);

            return Some(expr);
        }

        if (VarRule {}).check_match(parser.get_cursor()) {
            return parser.apply_rule_boxed(VarRule {}, "primary var", None);
        }
        
        let cur = parser.cur();

        let literal = match (cur.token_type, cur.value) {
            (TokenType::IntLiteral, TokenValue::Int(x)) => ASTWrapper::new_literal(Literal::Int(x), ParsedType::Unit(ParsedUnitType::Integer), cur.position),
            (TokenType::DoubleLiteral, TokenValue::Double(x)) => ASTWrapper::new_literal(Literal::Double(x), ParsedType::Unit(ParsedUnitType::Double), cur.position),
            (TokenType::BoolLiteral, TokenValue::Bool(x)) => ASTWrapper::new_literal(Literal::Bool(x), ParsedType::Unit(ParsedUnitType::Boolean), cur.position),
            _ => {
                parser.push_diagnostic(ErrMsg::ExpectedExpression.make_diagnostic(cur.position));
                return None;
            }
        };

        parser.next();

        return Some(Box::new(literal));
    }
}