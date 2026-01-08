use core::fmt;

use crate::ast::{BinaryOperator, UnaryOperator};
use crate::logger::LogLevel;
use crate::lexer::token::{PositionRange, TokenType};

#[derive(Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    #[allow(dead_code)]
    Warning
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            DiagnosticSeverity::Error => "ERROR",
            DiagnosticSeverity::Warning => "WARNING"
        };

        write!(f, "{}", str)
    }
}

impl Into<LogLevel> for DiagnosticSeverity {
    fn into(self) -> LogLevel {
        match self {
            DiagnosticSeverity::Error => LogLevel::Error,
            DiagnosticSeverity::Warning => LogLevel::Warning
        }
    }
}

pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub position: PositionRange,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, [{}]: {}", self.severity, self.position, self.message)
    }
}

#[allow(dead_code)]
pub enum ErrMsg {
    UnknownSymbol(char),
    UnterminatedString,
    DecimalLiteralAsInt,
    UnterminatedChar,
    ExpectedStatement,
    ExpectedExpression,
    ExpectedDeclaration,
    ExpectedAssignment,
    ExpectedBlock,
    ExpectedType,
    ExpectedArguments,
    ExpectedParameters,
    ExpectedToken(TokenType),
    UnknownEscapeSequence(char),
    UnknownTypeName(String),
    UnknownVariable(String),
    DuplicateVariable(String),
    IncompatibleBinaryTypes(String, String, BinaryOperator),
    IncompatibleUnaryType(String, UnaryOperator),
    FieldNotFound(String),
    IncompatibleMemberAccessType(String),
    ArrayIndexNotInteger(String),
    MismatchedIfBranches(String, String),
    IncompatibleAssignment(String, String),
    FunctionArgumentCountMismatch(usize, usize),
    FunctionArgumentTypeMismatch(usize, String, String),
    CallOnNonFunctionType(String),
    ConstructorNotFound(String)
}

impl ErrMsg {
    pub fn make_diagnostic(self, position: PositionRange) -> Diagnostic {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            message: format!("{}", &self),
            position
        }
    }
}

impl fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::UnknownSymbol(x) => &format!("unknown symbol {}", x),
            Self::UnterminatedString => "unterminated string",
            Self::DecimalLiteralAsInt => "decimal literal cannot be used as int",
            Self::UnterminatedChar => "unterminated char literal",
            Self::ExpectedDeclaration => "expected declaration",
            Self::ExpectedParameters => "expected parameters",
            Self::ExpectedArguments => "expected arguments",
            Self::ExpectedAssignment => "expected assignment",
            Self::ExpectedBlock => "expected block",
            Self::ExpectedStatement => "expected statement",
            Self::ExpectedExpression => "expected expression",
            Self::ExpectedType => "expected type",
            Self::ExpectedToken(token) => &format!("expected '{}' token", token),
            Self::UnknownEscapeSequence(x) => &format!("unknown escape sequence: \\{}", x),
            Self::UnknownTypeName(name) => &format!("unknown type name: '{}'", name),
            Self::UnknownVariable(name) => &format!("unknown variable: '{}'", name),
            Self::DuplicateVariable(name) => &format!("duplicate variable declaration: '{}'", name),
            Self::IncompatibleBinaryTypes(left, right, operator) => {
                &format!("incompatible types for operator '{}': left is '{}', right is '{}'", operator, left, right)
            },
            Self::IncompatibleUnaryType(ty, operator) => {
                &format!("incompatible type for operator '{}': expression is of type '{}'", operator, ty)
            },
            Self::FieldNotFound(field_name) => {
                &format!("field '{}' not found in struct", field_name)
            },
            Self::IncompatibleMemberAccessType(ty) => {
                &format!("cannot access member of type '{}'", ty)
            },
            Self::ArrayIndexNotInteger(ty) => {
                &format!("array index must be of integer type, found '{}'", ty)
            },
            Self::MismatchedIfBranches(then_type, else_type) => {
                &format!("mismatched types in if branches: 'then' is '{}', 'else' is '{}'", then_type, else_type)
            },
            Self::IncompatibleAssignment(var_type, expr_type) => {
                &format!("cannot assign expression of type '{}' to variable of type '{}'", expr_type, var_type)
            },
            Self::FunctionArgumentCountMismatch(expected, found) => {
                &format!("function expected {} arguments, but {} were provided", expected, found)
            },
            Self::FunctionArgumentTypeMismatch(index, expected, found) => {
                &format!("function argument {} expected type '{}', but found type '{}'", index, expected, found)
            },
            Self::CallOnNonFunctionType(ty) => {
                &format!("cannot call expression of non-function type '{}'", ty)
            },
            Self::ConstructorNotFound(ty) => {
                &format!("constructor not found for type '{}'", ty)
            }
        };

        write!(f, "{}", msg)
    }
}
