use serde::Serialize;

use crate::{ast::{ASTNode, AstId, Path, Require}, impl_ast_node, impl_positioned, lexer::token::PositionRange, new_ast_id};

#[derive(Serialize)]
pub struct Scope {
    pub path: Path,
    pub requires: Vec<Require>,
    pub child_scopes: Vec<Scope>,
    pub items: Vec<Box<dyn ASTNode>>,
    position: PositionRange,
    id: AstId,
}

impl Scope {
    pub fn new(path: Path, requires: Vec<Require>, child_scopes: Vec<Scope>, items: Vec<Box<dyn ASTNode>>, position: PositionRange) -> Self {
        Self {
            path,
            requires,
            child_scopes,
            items,
            position,
            id: new_ast_id!(),
        }
    }

    pub fn flatten(mut self) -> Vec<Scope> {
        let mut scopes = Vec::new();

        for mut child_scope in self.child_scopes.drain(..) {
            let mut child_path = self.path.clone();
            child_path.extend(&child_scope.path);
            child_scope.path = child_path;

            scopes.extend(child_scope.flatten());
        }

        scopes.push(self);

        scopes
    }
}

impl_positioned!(Scope);
impl_ast_node!(Scope, visit_scope);