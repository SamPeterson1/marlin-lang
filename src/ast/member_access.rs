use std::rc::Rc;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub enum AccessType {
    Indirect(String),
    Direct(String)
}

#[derive(Serialize)]
pub struct MemberAccess {
    pub id: i32,
    pub identifier: Rc<String>,
    pub member_accesses: Vec<AccessType>,
}

impl ASTWrapper<MemberAccess> {
    pub fn new_member_access(id: i32, identifier: String, member_accesses: Vec<AccessType>, position: PositionRange) -> Self {
        ASTWrapper {
            data: MemberAccess {
                id,
                identifier: Rc::new(identifier),
                member_accesses: member_accesses
            },
            position
        }
    }
}
