use std::{hash::Hasher, rc::Rc};

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange};

#[derive(Clone, Serialize)]
pub struct VarExpr {
    pub id: i32,
    pub is_reference: bool,
    pub identifier: Rc<String>,
}

impl Eq for VarExpr {}

impl PartialEq for VarExpr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for VarExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ASTWrapper<VarExpr> {
    pub fn new_var(id: i32, identifier: String, is_reference: bool, position: PositionRange) -> Self {
        ASTWrapper {
            data: VarExpr {
                id,
                identifier: Rc::new(identifier),
                is_reference,
            },
            position
        }
    }
}

impl_ast_node!(VarExpr, visit_var);