use serde::Serialize;

use crate::ast::{ASTNode, ParsedType};
use crate::lexer::token::PositionRange;
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};

#[derive(Serialize)]
pub struct CastExpr {
    pub expr: Box<dyn ASTNode>,
    pub cast_type: ParsedType,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
}

impl CastExpr {
    pub fn new(expr: Box<dyn ASTNode>, cast_type: ParsedType, position: PositionRange) -> Self {
        Self {
            expr,
            cast_type,
            position,
            resolved_type: None,
        }
    }    
}

impl_positioned!(CastExpr);
impl_typed!(CastExpr);
impl_ast_node!(CastExpr, visit_cast);