use serde::Serialize;

use crate::ast::{ASTEnum, ASTNode, block_expr::BlockExpr, declaration_expr::DeclarationExpr, AstId};
use crate::compiler::visit::{Parsed, Phase};
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct LoopExpr<P: Phase = Parsed> {
    pub initial: Option<DeclarationExpr<P>>,
    pub condition: Option<ASTEnum<P>>,
    pub increment: Option<ASTEnum<P>>,
    pub body: BlockExpr<P>,
    pub label: Option<Located<String>>,
    position: PositionRange,
    id: AstId,
}

impl LoopExpr {
    pub fn new_loop(body: BlockExpr, label: Option<Located<String>>, position: PositionRange) -> Self {
        Self {
            initial: None, 
            condition: None, 
            increment: None, 
            body,
            label,
            position,
            id: new_ast_id!(),
        }
    }
    
    pub fn new_while(condition: ASTEnum, body: BlockExpr, label: Option<Located<String>>, position: PositionRange) -> Self {
        Self {
            initial: None, 
            condition: Some(condition), 
            increment: None, 
            body,
            label,
            position,
            id: new_ast_id!(),
        }
    }

    pub fn new_for(initial: DeclarationExpr, condition: ASTEnum, increment: ASTEnum, body: BlockExpr, label: Option<Located<String>>, position: PositionRange) -> Self {
        Self {
            initial: Some(initial), 
            condition: Some(condition), 
            increment: Some(increment), 
            body,
            label,
            position,
            id: new_ast_id!(),
        }
    }
}

impl_ast_node!(LoopExpr, visit_loop);