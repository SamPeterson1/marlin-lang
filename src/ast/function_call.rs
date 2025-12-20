use serde::Serialize;

use crate::ast::{ASTNode, arguments::Arguments};
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct FunctionCall {
    pub expr: Box<dyn ASTNode>,
    pub arguments: Arguments,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl FunctionCall {
    pub fn new(expr: Box<dyn ASTNode>, arguments: Arguments, position: PositionRange) -> Self {
        Self {
            expr,
            arguments,
            position,
            resolved_type: None,
        }
    }
}

impl_positioned!(FunctionCall);
impl_typed!(FunctionCall);
impl_ast_node!(FunctionCall, visit_function_call);
