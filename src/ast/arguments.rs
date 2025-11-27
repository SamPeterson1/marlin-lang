use crate::{ast::{ASTNode, ASTWrapper}, token::PositionRange};

pub type Arguments = Vec<Box<dyn ASTNode>>;

impl ASTWrapper<Arguments> {
    pub fn new_arguments(arguments: Arguments, position: PositionRange) -> Self {
        ASTWrapper::new(arguments, position)
    }    
}