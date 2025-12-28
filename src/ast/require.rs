use serde::Serialize;

use crate::ast::{ASTNode, AstId, Path};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct Require {
    pub path: Path,
    pub alias: Option<Located<String>>, 
}

impl Require {
    pub fn new(path: Path, alias: Option<Located<String>>) -> Self {
        Self {
            path,
            alias,
        }
    }
}