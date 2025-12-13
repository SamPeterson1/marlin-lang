use serde::Serialize;

use crate::ast::{ASTNode, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct DeclarationExpr {
    pub identifier: Located<String>,
    pub declaration_type: ParsedType,
    pub expr: Box<dyn ASTNode>,
    position: PositionRange,
}


impl DeclarationExpr {
    pub fn new(identifier: Located<String>, declaration_type: ParsedType, expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        Self {
            identifier,
            declaration_type,
            expr,
            position
        }
    }
}

impl_positioned!(DeclarationExpr);
impl_ast_node!(DeclarationExpr, visit_declaration);