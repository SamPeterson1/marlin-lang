use serde::Serialize;

use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub enum Literal {    
    Int (i64),
    Double (f64),
    Bool (bool),
    Char (char),
    String (String),
}

#[derive(Serialize)]
pub struct LiteralExpr {
    pub value: Literal,
    position: PositionRange,
}

impl LiteralExpr {
    pub fn new(value: Literal, position: PositionRange) -> Self {
        Self {
            value,
            position,
        }
    }
}

impl_positioned!(LiteralExpr);
impl_ast_node!(LiteralExpr, visit_literal);