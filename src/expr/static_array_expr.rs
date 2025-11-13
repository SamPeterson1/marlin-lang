use std::fmt;

use crate::{token::PositionRange, types::parsed_type::ParsedType};

use super::Expr;

pub struct StaticArrayExpr {
    pub len: usize,
    pub declaration_type: ParsedType,
    pub position: PositionRange
}

impl fmt::Display for StaticArrayExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"StaticArray\", \"len\": {}, \"declaration_type\": {}, \"position\": \"{}\"}}", self.len, self.declaration_type, self.position)
    }
}

impl StaticArrayExpr {
    pub fn new(len: usize, declaration_type: ParsedType, position: PositionRange) -> StaticArrayExpr {
        StaticArrayExpr {
            len, declaration_type,
            position
        }
    }
}

crate::impl_expr!(StaticArrayExpr, visit_static_array);