use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, parsed_type::ParsedType, AstId};
use crate::compiler::stages::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct DeclarationExpr<P: Phase = Parsed> {
    pub identifier: Located<String>,
    pub declaration_type: ParsedType,
    pub expr: Option<ASTEnum<P>>,
    position: PositionRange,
    id: AstId,
}

impl DeclarationExpr {
    pub fn new(identifier: Located<String>, declaration_type: ParsedType, expr: Option<ASTEnum>, position: PositionRange) -> Self {
        Self {
            identifier,
            declaration_type,
            expr,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(DeclarationExpr, visit_declaration);