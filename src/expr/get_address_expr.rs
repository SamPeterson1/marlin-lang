use std::fmt;

use crate::token::PositionRange;

use super::{var_expr::VarExpr, Expr};

pub struct GetAddressExpr {
    pub var_expr: VarExpr,
    pub position: PositionRange
}

impl fmt::Display for GetAddressExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"GetAddress\", \"var_expr\": {}, \"position\": \"{}\"}}", self.var_expr, self.position)
    }
}

impl GetAddressExpr {
    pub fn new(var_expr: VarExpr, position: PositionRange) -> GetAddressExpr {
        GetAddressExpr {
            var_expr,
            position
        }
    }
}

crate::impl_expr!(GetAddressExpr, visit_get_address);