use serde::Serialize;

use crate::ast::{ASTNode, arguments::Arguments};
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub enum AccessType {
    Indirect(Located<String>),
    Direct(Located<String>),
    Array(Box<dyn ASTNode>),
    FunctionCall(Arguments)
}

#[derive(Serialize)]
pub struct MemberAccess {
    pub expr: Box<dyn ASTNode>,
    pub member_accesses: Vec<AccessType>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl MemberAccess {
    pub fn new(expr: Box<dyn ASTNode>, member_accesses: Vec<AccessType>, position: PositionRange) -> Self {
        Self {
            expr,
            member_accesses,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(MemberAccess);
impl_typed!(MemberAccess);
impl_ast_node!(MemberAccess, visit_member_access);
