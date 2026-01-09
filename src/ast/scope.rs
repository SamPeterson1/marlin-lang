use serde::Serialize;

use crate::{ast::{ASTEnum, ASTNode, AstId, Path}, compiler::visit::{Parsed, Phase}, impl_ast_node, lexer::token::{Located, PositionRange}, new_ast_id};

#[derive(Serialize)]
pub struct Require {
    pub path: Located<Path>,
    pub alias: Option<Located<String>>, 
}

impl Require {
    pub fn new(path: Located<Path>, alias: Option<Located<String>>) -> Self {
        Self {
            path,
            alias,
        }
    }
}

#[derive(Serialize)]
pub struct Scope<P: Phase = Parsed> {
    pub path: Located<Path>,
    pub requires: Vec<Require>,
    pub child_scopes: Vec<Scope<P>>,
    pub items: Vec<ASTEnum<P>>,
    position: PositionRange,
    id: AstId,
}

impl Scope {
    pub fn new(path: Located<Path>, requires: Vec<Require>, child_scopes: Vec<Scope>, items: Vec<ASTEnum>, position: PositionRange) -> Self {
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

impl_ast_node!(Scope, visit_scope);