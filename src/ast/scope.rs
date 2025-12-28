use serde::Serialize;

use crate::{ast::{AstId, Program}, impl_ast_node, impl_positioned, lexer::token::{Located, PositionRange}, new_ast_id};

#[derive(Serialize)]
pub struct Scope {
    pub path: Vec<Located<String>>,
    pub body: Program,
    position: PositionRange,
    id: AstId,
}

impl Scope {
    pub fn new(path: Vec<Located<String>>, body: Program, position: PositionRange) -> Self {
        Self {
            path,
            body,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(Scope);
impl_ast_node!(Scope, visit_scope);