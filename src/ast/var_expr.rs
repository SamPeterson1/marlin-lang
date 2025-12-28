use serde::Serialize;

use crate::ast::AstId;
use crate::ast::path::Path;
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{PositionRange, Positioned};

#[derive(Serialize)]
pub struct VarExpr {
    pub path: Path,
    id: AstId,
}

impl VarExpr {
    pub fn new(path: Path) -> Self {
        Self {
            path,
            id: new_ast_id!(),
        }
    }
}

impl Positioned for VarExpr {
    fn get_position(&self) -> &PositionRange {
        self.path.get_position()
    }
}

impl_ast_node!(VarExpr, visit_var);