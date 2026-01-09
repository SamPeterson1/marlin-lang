use serde::Serialize;

use crate::ast::{function_item::FunctionItem, ASTNode, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct ImplItem<P: Phase = Parsed> {
    pub identifier: Located<String>,
    pub functions: Vec<FunctionItem<P>>,
    position: PositionRange,
    id: AstId,
}

impl ImplItem {
    pub fn new(identifier: Located<String>, functions: Vec<FunctionItem>, position: PositionRange) -> Self {
        Self {
            identifier,
            functions,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(ImplItem, visit_impl);