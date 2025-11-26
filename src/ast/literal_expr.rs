use serde::Serialize;

use crate::{ast::ASTWrapper, token::PositionRange};

#[derive(Serialize)]
pub enum Literal {    
    Int (i64),
    Double (f64),
    Bool (bool),
}

#[derive(Serialize)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl ASTWrapper<LiteralExpr> {
    pub fn new_int_literal(value: i64, position: PositionRange) -> Self {
        ASTWrapper {
            data: LiteralExpr {
                value: Literal::Int(value),
            },
            position
        }
    }

    pub fn new_double_literal(value: f64, position: PositionRange) -> Self {
        ASTWrapper {
            data: LiteralExpr {
                value: Literal::Double(value),
            },
            position
        }
    }

    pub fn new_bool_literal(value: bool, position: PositionRange) -> Self {
        ASTWrapper {
            data: LiteralExpr {
                value: Literal::Bool(value),
            },
            position
        }
    }    
}

crate::impl_ast_node!(LiteralExpr, visit_literal);