use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

#[derive(Serialize)]
pub struct LoopExpr {
    pub initial: Option<Box<dyn ASTNode>>,
    pub condition: Option<Box<dyn ASTNode>>,
    pub increment: Option<Box<dyn ASTNode>>,
    pub body: Box<dyn ASTNode>,
}

impl ASTWrapper<LoopExpr> {
    pub fn new_loop(body: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: LoopExpr {
                initial: None, 
                condition: None, 
                increment: None, 
                body,
            },
            position
        }
    }
    
    pub fn new_while(condition: Box<dyn ASTNode>, body: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: LoopExpr {
                initial: None, 
                condition: Some(condition), 
                increment: None, 
                body,
            },
            position
        }
    }

    pub fn new_for(initial: Box<dyn ASTNode>, condition: Box<dyn ASTNode>, increment: Box<dyn ASTNode>, body: Box<dyn ASTNode>, position: PositionRange) -> Self {
        ASTWrapper {
            data: LoopExpr {
                initial: Some(initial), 
                condition: Some(condition), 
                increment: Some(increment), 
                body,
            },
            position
        }
    }
}

crate::impl_ast_node!(LoopExpr, visit_loop);