use std::{hash::Hasher, rc::Rc};

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, member_access::MemberAccess}, impl_ast_node, token::PositionRange};



#[derive(Serialize)]
pub struct LVarExpr {
    pub n_dereferences: u32,
    pub member_access: ASTWrapper<MemberAccess>
}

impl ASTWrapper<LVarExpr> {
    pub fn new_lvar(n_dereferences: u32, member_access: ASTWrapper<MemberAccess>, position: PositionRange) -> Self {
        ASTWrapper {
            data: LVarExpr {
                n_dereferences,
                member_access
            },
            position
        }
    }
}

impl_ast_node!(LVarExpr, visit_lvar);