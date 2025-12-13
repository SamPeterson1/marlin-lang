use serde::Serialize;

use crate::ast::{ASTNode, assignment_expr::AssignmentExpr, block_expr::BlockExpr, declaration_expr::DeclarationExpr};
use crate::{impl_ast_node, impl_positioned};
use crate::lexer::token::PositionRange;

#[derive(Serialize)]
pub struct LoopExpr {
    pub initial: Option<DeclarationExpr>,
    pub condition: Option<Box<dyn ASTNode>>,
    pub increment: Option<AssignmentExpr>,
    pub body: BlockExpr,
    position: PositionRange,
}

impl LoopExpr {
    pub fn new_loop(body: BlockExpr, position: PositionRange) -> Self {
        Self {
            initial: None, 
            condition: None, 
            increment: None, 
            body,
            position,
        }
    }
    
    pub fn new_while(condition: Box<dyn ASTNode>, body: BlockExpr, position: PositionRange) -> Self {
        Self {
            initial: None, 
            condition: Some(condition), 
            increment: None, 
            body,
            position,
        }
    }

    pub fn new_for(initial: DeclarationExpr, condition: Box<dyn ASTNode>, increment: AssignmentExpr,  body: BlockExpr, position: PositionRange) -> Self {
        Self {
            initial: Some(initial), 
            condition: Some(condition), 
            increment: Some(increment), 
            body,
            position,
        }
    }
}

impl_positioned!(LoopExpr);
impl_ast_node!(LoopExpr, visit_loop);