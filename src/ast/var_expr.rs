use serde::Serialize;

use crate::ast::AstId;
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange, Positioned};

#[derive(Serialize)]
pub struct VarExpr {
    pub identifier: Located<String>,
    id: AstId,
}

impl VarExpr {
    pub fn new(identifier: Located<String>) -> Self {
        Self {
            identifier,
            id: new_ast_id!(),
        }
    }
}

impl Positioned for VarExpr {
    fn get_position(&self) -> &PositionRange {
        self.identifier.get_position()
    }
}

impl_ast_node!(VarExpr, visit_var);