use std::{hash::Hasher, rc::Rc};

use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange};

#[derive(Debug, Clone, Serialize)]
pub enum MemberAccess {
    Direct(String),
    Indirect(String)
}

#[derive(Clone, Serialize)]
pub struct VarExpr {
    pub id: i32,
    pub identifier: Rc<String>,
    pub member_accesses: Rc<Vec<MemberAccess>>,
    pub array_accesses: Rc<Vec<Box<dyn ASTNode>>>,
    pub n_derefs: i32,
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
    pub fn new_var(id: i32, n_derefs: i32, identifier: String, member_accesses: Vec<MemberAccess>, array_accesses: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: VarExpr {
                id,
                n_derefs,
                identifier: Rc::new(identifier),
                member_accesses: Rc::new(member_accesses),
                array_accesses: Rc::new(array_accesses),
            },
            position
        }
    }
}

impl_ast_node!(VarExpr, visit_var);