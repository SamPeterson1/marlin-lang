use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, arguments::Arguments}, token::{Position, PositionRange, Token}};

#[derive(Serialize)]
pub struct CallExpr {
    pub function: String,
    pub args: Option<ASTWrapper<Arguments>>,
    pub applied_to: Box<dyn ASTNode>
}

impl ASTWrapper<CallExpr> {
    pub fn new_call(function: &Token, args: Option<ASTWrapper<Arguments>>, applied_to: Box<dyn ASTNode>) -> Self {
        let position = PositionRange::concat(&function.position, applied_to.get_position());

        ASTWrapper {
            data: CallExpr {
                function: function.get_string().to_string(),
                args,
                applied_to
            },
            position
        }
        
    }
}

crate::impl_ast_node!(CallExpr, visit_call);