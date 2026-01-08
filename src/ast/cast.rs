use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, AstId, ParsedType};
use crate::compiler::stages::{Parsed, Phase};
use crate::lexer::token::PositionRange;
use crate::{impl_ast_node, new_ast_id};

#[derive(Serialize)]
pub struct CastExpr<P: Phase = Parsed> {
    pub expr: ASTEnum<P>,
    pub cast_type: ParsedType,
    position: PositionRange,
    id: AstId,
}

impl CastExpr {
    pub fn new(expr: ASTEnum, cast_type: ParsedType, position: PositionRange) -> Self {
        Self {
            expr,
            cast_type,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_ast_node!(CastExpr, visit_cast);