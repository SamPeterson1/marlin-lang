use std::sync::Mutex;

use serde::Serialize;

use crate::ast::{ASTNode, parsed_type::ParsedType};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::{Located, PositionRange};

static DECLARATION_ID_COUNTER: Mutex<u64> = Mutex::new(0);

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct DeclarationId(u64);

#[derive(Serialize)]
pub struct DeclarationExpr {
    pub identifier: Located<String>,
    pub declaration_type: ParsedType,
    pub expr: Box<dyn ASTNode>,
    pub id: DeclarationId,
    position: PositionRange,
}

impl DeclarationExpr {
    pub fn new(identifier: Located<String>, declaration_type: ParsedType, expr: Box<dyn ASTNode>, position: PositionRange) -> Self {
        let mut id = DECLARATION_ID_COUNTER.lock().unwrap();
        let current_id = *id;
        *id = *id + 1;

        Self {
            identifier,
            declaration_type,
            expr,
            id: DeclarationId(current_id),
            position
        }
    }
}

impl_positioned!(DeclarationExpr);
impl_ast_node!(DeclarationExpr, visit_declaration);