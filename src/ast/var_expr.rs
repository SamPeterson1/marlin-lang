use serde::Serialize;

use crate::{impl_ast_node};
use crate::lexer::token::{Located, PositionRange, Positioned};

#[derive(Serialize)]
pub struct VarExpr {
    pub identifier: Located<String>,
}

impl VarExpr {
    pub fn new(identifier: Located<String>) -> Self {
        Self {
            identifier,
        }
    }
}

impl Positioned for VarExpr {
    fn get_position(&self) -> &PositionRange {
        self.identifier.get_position()
    }
}

impl_ast_node!(VarExpr, visit_var);