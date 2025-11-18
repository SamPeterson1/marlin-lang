use std::{collections::HashMap, rc::Rc};

use serde::Serialize;

use crate::{expr::{ASTNode, ASTWrapper}, impl_ast_node, token::PositionRange};

#[derive(Clone, Serialize)]
pub struct StructInitializerExpr {
    pub type_name: Rc<String>,
    pub member_inits: Rc<HashMap<String, Box<dyn ASTNode>>>,
}

impl ASTWrapper<StructInitializerExpr> {
    pub fn new_struct_initializer(type_name: String, member_inits: HashMap<String, Box<dyn ASTNode>>, position: PositionRange) -> Self {
        ASTWrapper {
            data: StructInitializerExpr {
                type_name: Rc::new(type_name),
                member_inits: Rc::new(member_inits),
            },
            position
        }
    }
}

impl_ast_node!(StructInitializerExpr, visit_struct_initializer);