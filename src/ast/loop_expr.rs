use serde::Serialize;

use crate::ast::{ASTNode, block_expr::BlockExpr, declaration_expr::DeclarationExpr};
use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_positioned, impl_typed};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct LoopExpr {
    pub initial: Option<DeclarationExpr>,
    pub condition: Option<Box<dyn ASTNode>>,
    pub increment: Option<Box<dyn ASTNode>>,
    pub body: BlockExpr,
    pub label: Option<Located<String>>,
    position: PositionRange,
    resolved_type: Option<ResolvedType>,
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
            resolved_type: None,
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
            resolved_type: None,
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
            resolved_type: None,
        }
    }
}

impl_positioned!(LoopExpr);
impl_typed!(LoopExpr);
impl_ast_node!(LoopExpr, visit_loop);