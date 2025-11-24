use serde::Serialize;

use crate::{ast::{ASTNode, ASTWrapper, assignment_expr::AssignmentExpr, block_expr::BlockExpr, declaration_expr::DeclarationExpr}, token::PositionRange};

#[derive(Serialize)]
pub struct LoopExpr {
    pub initial: Option<ASTWrapper<DeclarationExpr>>,
    pub condition: Option<Box<dyn ASTNode>>,
    pub increment: Option<ASTWrapper<AssignmentExpr>>,
    pub body: ASTWrapper<BlockExpr>,
}

impl ASTWrapper<LoopExpr> {
    pub fn new_loop(body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
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
    
    pub fn new_while(condition: Box<dyn ASTNode>, body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
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

    pub fn new_for(initial: ASTWrapper<DeclarationExpr>, condition: Box<dyn ASTNode>, increment: ASTWrapper<AssignmentExpr>, body: ASTWrapper<BlockExpr>, position: PositionRange) -> Self {
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