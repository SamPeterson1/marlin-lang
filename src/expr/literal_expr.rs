use std::fmt;

use crate::{token::PositionRange, types::parsed_type::ParsedType};

use super::Expr;

pub enum Literal {
    Int (i64),
    Double (f64),
    Bool (bool),
    String (String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{{\"type\": \"Integer\", \"value\": {}}}", i),
            Literal::Double(d) => write!(f, "{{\"type\": \"Double\", \"value\": {}}}", d),
            Literal::Bool(b) => write!(f, "{{\"type\": \"Boolean\", \"value\": {}}}", b),
            Literal::String(s) => write!(f, "{{\"type\": \"String\", \"value\": \"{}\"}}", s),
        }
    }
}

pub struct LiteralExpr {
    pub value: Literal,
    pub parsed_type: ParsedType,
    pub position: PositionRange,
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Literal\", \"value\": {}, \"parsed_type\": \"{}\", \"position\": \"{}\"}}", self.value, self.parsed_type, self.position)
    }
}

impl LiteralExpr {
    pub fn new(value: Literal, parsed_type: ParsedType, position: PositionRange) -> LiteralExpr {
        LiteralExpr {
            value,
            parsed_type,
            position
        }
    }
}

crate::impl_expr!(LiteralExpr, visit_literal);