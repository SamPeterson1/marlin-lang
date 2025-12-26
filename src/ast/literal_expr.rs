use serde::Serialize;

use crate::ast::AstId;
use crate::{impl_ast_node, impl_positioned, new_ast_id};
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
    id: AstId,
}

impl LiteralExpr {
    pub fn new(value: Literal, position: PositionRange) -> Self {
        Self {
            value,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(LiteralExpr);
impl_ast_node!(LiteralExpr, visit_literal);