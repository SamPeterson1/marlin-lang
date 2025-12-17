use core::fmt;

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
        };

        write!(f, "{}", msg)
    }
}
