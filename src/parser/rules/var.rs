use std::fmt;

use crate::{expr::var_expr::{MemberAccess, VarExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::inline_expr::InlineExprRule}, token::{PositionRange, TokenType}};

pub struct VarRule {}

impl fmt::Display for VarRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var")
    }
}

//var: STAR* IDENTIFIER (DOT IDENTIFIER)* (LEFT_BRACKET inline_expression RIGHT_BRACKET)*
impl ParseRule<VarExpr> for VarRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<VarExpr> {
        parser.log_debug(&format!("Entering var parser. Current token {:?}", parser.cur()));
    
        let first_position = parser.cur().position.clone();
        let mut n_derefs = 0;

        while parser.try_consume(TokenType::Star).is_some() {
            n_derefs += 1;
        }

        parser.log_debug(&format!("Parsed {} derefs", n_derefs));

        let identifier = parser.try_consume(TokenType::Identifier)
            .map(|x| x.get_string().to_string())?;

        let mut member_accesses = Vec::new();

        while let Some(access_token) = parser.try_match(&[TokenType::Dot, TokenType::Arrow]) {
            if let Some(identifier) = parser.try_consume(TokenType::Identifier) {
                if access_token.token_type == TokenType::Dot {
                    parser.log_debug("Parsed direct member access");
                    member_accesses.push(MemberAccess::Direct(identifier.get_string().to_string()));
                } else {
                    parser.log_debug("Parsed indirect member access");
                    member_accesses.push(MemberAccess::Indirect(identifier.get_string().to_string()));
                }
            } else {
                return None;
            }
        }

        let mut array_accesses = Vec::new();

        while parser.try_consume(TokenType::LeftSquare).is_some() {
            let expr = parser.apply_rule(InlineExprRule {});

            if let Some(inline_expr) = expr {
                array_accesses.push(inline_expr);   
            }

            parser.try_consume(TokenType::RightSquare)?;
        }

        parser.var_expr_id_counter += 1;

        let position = PositionRange::concat(&first_position, &parser.prev().position);

        Some(VarExpr::new_unboxed(parser.var_expr_id_counter, n_derefs, identifier, member_accesses, array_accesses, position))
    }
}
