use serde::Serialize;

use crate::ast::{ASTNode, ParsedType, AstId};
use crate::lexer::token::PositionRange;
use crate::{impl_ast_node, impl_positioned, new_ast_id};

#[derive(Serialize)]
pub struct CastExpr {
    pub expr: Box<dyn ASTNode>,
    pub cast_type: ParsedType,
    position: PositionRange,
    id: AstId,
}

impl CastExpr {
    pub fn new(expr: Box<dyn ASTNode>, cast_type: ParsedType, position: PositionRange) -> Self {
        Self {
            expr,
            cast_type,
            position,
            id: new_ast_id!(),
        }
    }    
}

impl_positioned!(CastExpr);
impl_ast_node!(CastExpr, visit_cast);