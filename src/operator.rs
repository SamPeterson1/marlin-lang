use std::fmt;

use erased_serde::serialize_trait_object;
use serde::{Serializer, Serialize};

use crate::{error::{Diagnostic, DiagnosticType}, instruction::InstructionBuilder, token::{Position, PositionRange, TokenType}, types::resolved_type::ResolvedType};

pub fn as_binary_operator(token_type: TokenType) -> Box<dyn BinaryOperator> {
    match token_type {
        TokenType::Plus => Box::new(Plus),
        TokenType::Minus => Box::new(Minus),
        TokenType::Star => Box::new(Times),
        TokenType::Slash => Box::new(Divide),
        TokenType::Greater => Box::new(Greater),
        TokenType::GreaterEqual => Box::new(GreaterEqual),
        TokenType::Less => Box::new(Less),
        TokenType::LessEqual => Box::new(LessEqual),
        TokenType::Equal => Box::new(Equal),
        TokenType::NotEqual => Box::new(NotEqual),
        TokenType::And => Box::new(And),
        TokenType::Or => Box::new(Or),
        _ => panic!("Invalid binary operator {:?}", token_type)
    }
}

pub fn as_unary_operator(token_type: TokenType) -> Box<dyn UnaryOperator> {
    match token_type {
        TokenType::Semicolon => Box::new(Semicolon),
        TokenType::Not => Box::new(Not),
        TokenType::Minus => Box::new(Negative),
        _ => panic!("Invalid unary operator {:?}", token_type)
    }
}

pub trait UnaryOperator : fmt::Debug + erased_serde::Serialize {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic>;
    fn compile(&self, dr: u8, sr1: u8, operation_type: ResolvedType) -> Vec<u64>;
}

serialize_trait_object!(UnaryOperator);

#[derive(Debug)]
struct Not;

impl Serialize for Not {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str("!")
    }
}

impl UnaryOperator for Not {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        if value_type == ResolvedType::Boolean {
            Ok(value_type)
        } else {
            Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
            //Err(TypeError::new_unary(&value_type, "!"))
        }
    }

    fn compile(&self, dr: u8, sr1: u8, operation_type: ResolvedType) -> Vec<u64> {
        let mut instructions = Vec::new();
        
        instructions.push(match operation_type {
            ResolvedType::Boolean => InstructionBuilder::notb(dr, sr1),
            _ => panic!("Unsupported type for operator '!'")
        });

        instructions
    }
}

#[derive(Debug)]
struct Negative;

impl Serialize for Negative {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str("-")
    }
}

impl UnaryOperator for Negative {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        if value_type.is_numeric() {
            Ok(value_type)
        } else {
            Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
            //Err(TypeError::new_unary(&value_type, "-"))
        }
    }

    fn compile(&self, dr: u8, sr1: u8, operation_type: ResolvedType) -> Vec<u64> {
        let mut instructions = Vec::new();
        
        instructions.push(match operation_type {
            ResolvedType::Integer => InstructionBuilder::negi(dr, sr1),
            ResolvedType::Double => InstructionBuilder::negd(dr, sr1),
            _ => panic!("Unsupported type for operator '-'")
        });

        instructions
    }

}

#[derive(Debug)]

struct Semicolon;

impl Serialize for Semicolon {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(";")
    }
}

impl UnaryOperator for Semicolon {
    fn interpret_type(&self, _value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        Ok(ResolvedType::Empty)
    }

    fn compile(&self, dr: u8, sr1: u8, operation_type: ResolvedType) -> Vec<u64> {
        Vec::new()
    }
}

pub trait BinaryOperator : erased_serde::Serialize {
    fn interpret_type(&self, left: ResolvedType, right: ResolvedType) -> Result<ResolvedType, Diagnostic>;
    fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64>;
}

serialize_trait_object!(BinaryOperator);

macro_rules! arithmetic_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr, $Compile:item) => {
        #[derive(Debug)]
        struct $Name;

        impl BinaryOperator for $Name {
            fn interpret_type(&self, left: ResolvedType, right: ResolvedType) -> Result<ResolvedType, Diagnostic> {
                if left == right && left.is_numeric(){
                    Ok(left)
                } else {
                    Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
                    //Err(TypeError::new_binary(&left, &right, $OperatorName))
                }
            }

            $Compile
        }

        impl Serialize for $Name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str($OperatorName)
            }
        }
    };
}

macro_rules! comparative_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr, $Compile:item) => {
        #[derive(Debug)]
        struct $Name;

        impl BinaryOperator for $Name {
            fn interpret_type(&self, left: ResolvedType, right: ResolvedType) -> Result<ResolvedType, Diagnostic> {
                if left == right && left.is_numeric(){
                    Ok(ResolvedType::Boolean)
                } else {
                    Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
                    //Err(TypeError::new_binary(&left, &right, $OperatorName))
                }
            }

            $Compile
        }

        impl Serialize for $Name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str($OperatorName)
            }
        }
    };
}

macro_rules! boolean_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr, $Compile:item) => {
        #[derive(Debug)]
        struct $Name;

        impl BinaryOperator for $Name {
            fn interpret_type(&self, left: ResolvedType, right: ResolvedType) -> Result<ResolvedType, Diagnostic> {
                if left == right && left == ResolvedType::Boolean {
                    Ok(left)
                } else {
                    Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
                    //Err(TypeError::new_binary(&left, &right, $OperatorName))
                }
            }

            $Compile
        }

        impl Serialize for $Name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str($OperatorName)
            }
        }
    };
}

arithmetic_binary_operator!(Plus, +, "+", fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::addi(dr, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::addd(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

arithmetic_binary_operator!(Minus, -, "-", fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::subi(dr, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::subd(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '-'")
    });

    instructions
});

arithmetic_binary_operator!(Times, *, "*", fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::multi(dr, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::multd(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '*'")
    });

    instructions
});

arithmetic_binary_operator!(Divide, /, "/",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::divi(dr, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::divd(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '/'")
    });

    instructions
});

comparative_binary_operator!(Greater, >, ">",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, false, false, true, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, false, false, true, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

comparative_binary_operator!(GreaterEqual, >=, ">=",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, false, true, true, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, false, true, true, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

comparative_binary_operator!(Less, <, "<",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, true, false, false, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, true, false, false, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

comparative_binary_operator!(LessEqual, <=, "<=",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, true, true, false, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, true, true, false, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

comparative_binary_operator!(Equal, ==, "==",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, false, true, false, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, false, true, false, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

comparative_binary_operator!(NotEqual, !=, "!=",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Integer => InstructionBuilder::cmpi(dr, true, false, true, sr1, sr2),
        ResolvedType::Double => InstructionBuilder::cmpd(dr, true, false, true, sr1, sr2),
        _ => panic!("Unsupported type for operator '+'")
    });

    instructions
});

boolean_binary_operator!(And, &&, "&&",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Boolean => InstructionBuilder::and(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '&&'")
    });

    instructions
});

boolean_binary_operator!(Or, ||, "||",  fn compile(&self, dr: u8, sr1: u8, sr2: u8, operation_type: ResolvedType) -> Vec<u64> {
    let mut instructions = Vec::new();

    instructions.push(match operation_type {
        ResolvedType::Boolean => InstructionBuilder::or(dr, sr1, sr2),
        _ => panic!("Unsupported type for operator '||'")
    });

    instructions
});
