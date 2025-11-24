use serde::Serialize;

use crate::{ast::{ASTWrapper, parsed_type::ParsedType}, token::PositionRange};

#[derive(Serialize)]
pub enum Literal {    
    Int (i64),
    Double (f64),
    Bool (bool),
    String (String),
}

#[derive(Serialize)]
pub struct LiteralExpr {
    pub value: Literal,
    pub parsed_type: ParsedType,
}

impl ASTWrapper<LiteralExpr> {
    pub fn new_literal(value: Literal, parsed_type: ParsedType, position: PositionRange) -> Self {
        ASTWrapper {
            data: LiteralExpr {
                value,
                parsed_type,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(LiteralExpr, visit_literal);