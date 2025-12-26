use serde::Serialize;

use crate::ast::{ASTNode, parsed_type::ParsedType, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct DeclarationExpr {
    pub identifier: Located<String>,
    pub declaration_type: ParsedType,
    pub expr: Option<Box<dyn ASTNode>>,
    position: PositionRange,
    id: AstId,
}

impl DeclarationExpr {
    pub fn new(identifier: Located<String>, declaration_type: ParsedType, expr: Option<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            identifier,
            declaration_type,
            expr,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_positioned!(DeclarationExpr);
impl_ast_node!(DeclarationExpr, visit_declaration);