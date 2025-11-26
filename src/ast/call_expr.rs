use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, arguments::Arguments}, token::{PositionRange, Token}};

#[derive(Serialize)]
pub struct CallExpr {
    pub function: String,
    pub args: ASTWrapper<Arguments>,
}

impl ASTWrapper<CallExpr> {
    pub fn new_call(function: &Token, args: ASTWrapper<Arguments>, applied_to: Box<dyn ASTNode>) -> Self {
        let position = PositionRange::concat(&function.position, applied_to.get_position());

        ASTWrapper {
            data: CallExpr {
                function: function.get_string().to_string(),
                args,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(CallExpr, visit_call);