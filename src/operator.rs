use std::rc::Rc;

use crate::{environment::{ResolvedType, Value}, error::{Diagnostic, DiagnosticType}, token::{Position, PositionRange, Token, TokenType}};

pub fn as_binary_operator(token: &Token) -> Box<dyn BinaryOperator> {
    match token.token_type {
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
        _ => panic!("Invalid binary operator {:?}", token.token_type)
    }
}

pub fn as_unary_operator(token: &Token) -> Box<dyn UnaryOperator> {
    match token.token_type {
        TokenType::Semicolon => Box::new(Semicolon),
        TokenType::Not => Box::new(Not),
        TokenType::Minus => Box::new(Negative),
        _ => panic!("Invalid unary operator {:?}", token.token_type)
    }
}

pub trait UnaryOperator : std::fmt::Debug {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic>;
    fn interpret(&self, value: Rc<Value>) -> Rc<Value>;
}

#[derive(Debug)]
struct Not;

impl UnaryOperator for Not {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        if value_type == ResolvedType::Boolean {
            Ok(value_type)
        } else {
            Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
            //Err(TypeError::new_unary(&value_type, "!"))
        }
    }

    fn interpret(&self, value: Rc<Value>) -> Rc<Value> {
        Rc::new(Value::Bool(!value.as_bool()))
    }
}

#[derive(Debug)]
struct Negative;

impl UnaryOperator for Negative {
    fn interpret_type(&self, value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        if value_type.is_numeric() {
            Ok(value_type)
        } else {
            Err(Diagnostic::new(0, DiagnosticType::Error, PositionRange::new(Position::new(0, 0)), "placeholder".to_string()))
            //Err(TypeError::new_unary(&value_type, "-"))
        }
    }

    fn interpret(&self, value: Rc<Value>) -> Rc<Value> {
        let result = match *value {
            Value::Int(x) => Value::Int(-x),
            Value::Float(x) => Value::Float(-x),
            Value::Double(x) => Value::Double(-x),
            _ => unreachable!()
        };

        Rc::new(result)
    }
}

#[derive(Debug)]

struct Semicolon;

impl UnaryOperator for Semicolon {
    fn interpret_type(&self, _value_type: ResolvedType) -> Result<ResolvedType, Diagnostic> {
        Ok(ResolvedType::Empty)
    }

    fn interpret(&self, _value: Rc<Value>) -> Rc<Value> {
        Rc::new(Value::Empty)
    }
}

pub trait BinaryOperator : std::fmt::Debug {
    fn interpret_type(&self, left: ResolvedType, right: ResolvedType) -> Result<ResolvedType, Diagnostic>;
    fn interpret(&self, left: Rc<Value>, right: Rc<Value>) -> Rc<Value>;
}

macro_rules! arithmetic_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr) => {
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

            fn interpret(&self, left: Rc<Value>, right: Rc<Value>) -> Rc<Value> {
                let result = match *left {
                    Value::Int(x) => Value::Int(x $Operator right.as_int()),
                    Value::Float(x) => Value::Float(x $Operator right.as_float()),
                    Value::Double(x) => Value::Double(x $Operator right.as_double()),
                    _ => unreachable!()
                };

                Rc::new(result)
            }
        }
    };
}

macro_rules! comparative_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr) => {
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

            fn interpret(&self, left: Rc<Value>, right: Rc<Value>) -> Rc<Value> {
                let result = match *left {
                    Value::Int(x) => Value::Bool(x $Operator right.as_int()),
                    Value::Float(x) => Value::Bool(x $Operator right.as_float()),
                    Value::Double(x) => Value::Bool(x $Operator right.as_double()),
                    _ => unreachable!()
                };

                Rc::new(result)
            }
        }
    };
}

macro_rules! boolean_binary_operator {
    ($Name:ident, $Operator:tt, $OperatorName:expr) => {
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

            fn interpret(&self, left: Rc<Value>, right: Rc<Value>) -> Rc<Value> {
                let result = match *left {
                    Value::Bool(x) => Value::Bool(x $Operator right.as_bool()),
                    _ => unreachable!()
                };

                Rc::new(result)
            }
        }
    };
}

arithmetic_binary_operator!(Plus, +, "+");
arithmetic_binary_operator!(Minus, -, "-");
arithmetic_binary_operator!(Times, *, "*");
arithmetic_binary_operator!(Divide, /, "/");
comparative_binary_operator!(Greater, >, ">");
comparative_binary_operator!(GreaterEqual, >=, ">=");
comparative_binary_operator!(Less, <, "<");
comparative_binary_operator!(LessEqual, <=, "<=");
comparative_binary_operator!(Equal, ==, "==");
comparative_binary_operator!(NotEqual, !=, "!=");
boolean_binary_operator!(And, &&, "&&");
boolean_binary_operator!(Or, ||, "||");
