use serde::Serialize;

use crate::ast::{block_expr::BlockExpr, parameters::Parameters, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct FunctionItem {
    pub name: Located<String>,
    pub parameters: Parameters,
    pub return_type: ParsedType,
    pub body: BlockExpr,
    position: PositionRange,
}

impl FunctionItem {
    pub fn new(name: Located<String>, parameters: Parameters, return_type: ParsedType, body: BlockExpr, position: PositionRange) -> Self {
        Self {
            name,
            parameters,
            return_type,
            body,
            position,
        }
    }
}

impl_positioned!(FunctionItem);
impl_ast_node!(FunctionItem, visit_function);