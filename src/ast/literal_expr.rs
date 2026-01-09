use serde::Serialize;

use crate::ast::{ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
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
pub struct LiteralExpr<P: Phase = Parsed> {
    pub value: Literal,
    position: PositionRange,
    id: AstId,

    #[serde(skip)]
    _phase: std::marker::PhantomData<P>,
}

impl LiteralExpr {
    pub fn new(value: Literal, position: PositionRange) -> Self {
        Self {
            value,
            position,
            id: new_ast_id!(),
            _phase: std::marker::PhantomData,
        }
    }
}

impl_ast_node!(LiteralExpr, visit_literal);