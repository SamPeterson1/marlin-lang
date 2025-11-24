use std::hash::Hasher;

use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, parsed_type::ParsedType}, token::PositionRange};

#[derive(Serialize)]
pub struct DeclarationExpr {
    pub id: i32,
    pub identifier: String,
    pub declaration_type: ASTWrapper<ParsedType>,
    pub expr: Box<dyn ASTNode>,
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

impl ASTWrapper<DeclarationExpr> {
    pub fn new_declaration(id: i32, identifier: String, declaration_type: ASTWrapper<ParsedType>, expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: DeclarationExpr {
                id,
                identifier,
                declaration_type,
                expr,
            },
            position
        }
        
    }
}

crate::impl_ast_node!(DeclarationExpr, visit_declaration);