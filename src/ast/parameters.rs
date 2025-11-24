use crate::{ast::{ASTWrapper, parsed_type::ParsedType}, token::PositionRange};

pub type Parameters = Vec<(ASTWrapper<ParsedType>, String)>;

impl ASTWrapper<Parameters> {
    pub fn new_parameters(parameters: Parameters, position: PositionRange) -> Self {
        ASTWrapper {
            data: parameters,
            position
        }
    }    
}