use serde::Serialize;

use crate::ast::DeclarationExpr;
use crate::ast::{block_expr::BlockExpr, parsed_type::ParsedType, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct FunctionItem {
    pub name: Located<String>,
    pub parameters: Vec<DeclarationExpr>,
    pub return_type: ParsedType,
    pub body: Option<BlockExpr>,
    position: PositionRange,
    id: AstId,
}

impl FunctionItem {
    pub fn new(name: Located<String>, parameters: Vec<DeclarationExpr>, return_type: ParsedType, body: Option<BlockExpr>, position: PositionRange) -> Self {
        Self {
            name,
            parameters,
            return_type,
            body,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(FunctionItem);
impl_ast_node!(FunctionItem, visit_function);