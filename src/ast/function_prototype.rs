use serde::Serialize;

use crate::{ast::{ASTWrapper, parameters::Parameters, parsed_type::ParsedType}, token::{PositionRange, Token}};

#[derive(Serialize)]
pub struct FunctionPrototype {
    name: String,
    parameters: ASTWrapper<Parameters>,
    return_type: ASTWrapper<ParsedType>
}

impl ASTWrapper<FunctionPrototype> {
    pub fn new_function_prototype(name: Token, parameters: ASTWrapper<Parameters>, return_type: ASTWrapper<ParsedType>) -> Self {
        let position = PositionRange::concat(&name.position, &return_type.position);
        
        ASTWrapper {
            data: FunctionPrototype { 
                name: name.get_string().to_string(), 
                parameters,
                return_type
            },
            position
        }
    }    
}