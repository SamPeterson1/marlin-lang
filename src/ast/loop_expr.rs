use serde::Serialize;

use crate::ast::{ASTNode, block_expr::BlockExpr, declaration_expr::DeclarationExpr, AstId};
use crate::{impl_ast_node, impl_positioned, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct LoopExpr {
    pub initial: Option<DeclarationExpr>,
    pub condition: Option<Box<dyn ASTNode>>,
    pub increment: Option<Box<dyn ASTNode>>,
    pub body: BlockExpr,
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
    
    pub fn new_while(condition: Box<dyn ASTNode>, body: BlockExpr, label: Option<Located<String>>, position: PositionRange) -> Self {
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

    pub fn new_for(initial: DeclarationExpr, condition: Box<dyn ASTNode>, increment: Box<dyn ASTNode>, body: BlockExpr, label: Option<Located<String>>, position: PositionRange) -> Self {
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

impl_positioned!(LoopExpr);
impl_ast_node!(LoopExpr, visit_loop);