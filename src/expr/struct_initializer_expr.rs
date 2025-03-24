use std::{collections::HashMap, fmt, rc::Rc};

use crate::token::PositionRange;

use super::Expr;

#[derive(Clone)]
pub struct StructInitializerExpr {
    pub type_name: Rc<String>,
    pub member_inits: Rc<HashMap<String, Box<dyn Expr>>>,
    pub position: PositionRange
}

impl fmt::Display for StructInitializerExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"StructInitializer\", \"type_name\": \"{}\", \"member_inits\": {{", self.type_name)?;

        for (i, (name, expr)) in self.member_inits.iter().enumerate() {
            write!(f, "\"{}\": {}", name, expr)?;

            if i + 1 < self.member_inits.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}, \"position\": \"{}\"}}", self.position)
    }
}

impl StructInitializerExpr {
    pub fn new(type_name: String, member_inits: HashMap<String, Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(StructInitializerExpr {
            type_name: Rc::new(type_name),
            member_inits: Rc::new(member_inits),
            position
        })
    }
}

crate::impl_expr!(StructInitializerExpr, visit_struct_initializer);