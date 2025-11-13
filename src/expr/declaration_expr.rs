use std::{fmt, hash::Hasher};

use crate::{token::PositionRange, types::parsed_type::ParsedType};

use super::Expr;

pub struct DeclarationExpr {
    pub id: i32,
    pub identifier: String,
    pub declaration_type: ParsedType,
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for DeclarationExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Declaration\", \"identifier\": \"{}\", \"declaration_type\": {}, \"expr\": {}, \"position\": \"{}\"}}", self.identifier, self.declaration_type, self.expr, self.position)
    }
}

impl Eq for DeclarationExpr {}

impl PartialEq for DeclarationExpr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for DeclarationExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl DeclarationExpr {
    pub fn new(id: i32, identifier: String, declaration_type: ParsedType, expr: Box<dyn Expr>, position: PositionRange) -> DeclarationExpr {
        DeclarationExpr {
            id,
            identifier,
            declaration_type,
            expr,
            position,
        }
    }
}

crate::impl_expr!(DeclarationExpr, visit_declaration);