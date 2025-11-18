use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, token::{Position, PositionRange}};

#[derive(Serialize)]
pub struct CallExpr {
    pub function: String,
    pub args: Vec<Box<dyn ASTNode>>,
}

impl ASTWrapper<CallExpr> {
    pub fn new_call(function: String, args: Vec<Box<dyn ASTNode>>) -> Self {
        let position = PositionRange::new(Position::new(0, 0));

        ASTWrapper {
            data: CallExpr {
                function,
                args,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(CallExpr, visit_call);