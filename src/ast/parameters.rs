use serde::Serialize;

use crate::ast::parsed_type::ParsedType;
use crate::{impl_positioned};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct Parameters {
    pub parameters: Vec<(ParsedType, Located<String>)>,
    position: PositionRange,
}

impl Parameters {
    pub fn new(parameters: Vec<(ParsedType, Located<String>)>, position: PositionRange) -> Self {
        Self {
            parameters,
            position,
        }
    }
}

impl_positioned!(Parameters);