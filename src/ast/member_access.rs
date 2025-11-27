use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, arguments::Arguments}, impl_ast_node, token::PositionRange};

#[derive(Serialize)]
pub enum AccessType {
    Indirect(String),
    Direct(String),
    Array(Box<dyn ASTNode>),
    FunctionCall(ASTWrapper<Arguments>)
}

#[derive(Serialize)]
pub struct MemberAccess {
    pub expr: Box<dyn ASTNode>,
    pub member_accesses: Vec<AccessType>,
}

impl ASTWrapper<MemberAccess> {
    pub fn new_member_access(expr: Box<dyn ASTNode>, member_accesses: Vec<AccessType>, position: PositionRange) -> Self {
        ASTWrapper::new(MemberAccess {
            expr,
            member_accesses,
        }, position)
    }
}

impl_ast_node!(MemberAccess, visit_member_access);
