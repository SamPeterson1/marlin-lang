use serde::Serialize;

use crate::ast::{ASTNode, DeclarationExpr};
use crate::ast::{block_expr::BlockExpr, parsed_type::ParsedType, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct FunctionItem<P: Phase = Parsed> {
    pub name: Located<String>,
    pub parameters: Vec<DeclarationExpr<P>>,
    pub return_type: ParsedType,
    pub body: Option<BlockExpr<P>>,
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

impl_ast_node!(FunctionItem, visit_function);