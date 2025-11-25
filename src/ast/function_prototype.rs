use serde::Serialize;

use crate::{ast::{ASTWrapper, parameters::Parameters, parsed_type::ParsedType}, token::{PositionRange, Token}};

#[derive(Serialize)]
pub struct FunctionPrototype {
    name: String,
    parameters: ASTWrapper<Parameters>,
    return_type: ASTWrapper<ParsedType>
}

impl ASTWrapper<FunctionPrototype> {
    pub fn new_function_prototype(name: String, parameters: ASTWrapper<Parameters>, return_type: ASTWrapper<ParsedType>, position: PositionRange) -> Self {        
        ASTWrapper {
            data: FunctionPrototype { 
                name, 
                parameters,
                return_type
            },
            position
        }
    }    
}