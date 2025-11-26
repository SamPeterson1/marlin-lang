use std::{hash::Hasher, rc::Rc};

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub enum MemberAccess {
    Indirect(String),
    Direct(String)
}

#[derive(Serialize)]
pub struct RVarExpr {
    pub is_reference: bool,
    pub member_access: MemberAccess,
}


impl ASTWrapper<RVarExpr> {
    pub fn new_var(is_reference: bool, member_access: MemberAccess, position: PositionRange) -> Self {
        ASTWrapper {
            data: RVarExpr {
                is_reference,
                member_access
            },
            position
        }
    }
}

impl_ast_node!(RVarExpr, visit_rvar);