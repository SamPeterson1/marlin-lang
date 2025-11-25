use core::fmt;

use crate::{token::{PositionRange, TokenType}};

pub enum DiagnosticSeverity {
    Error,
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

pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub position: PositionRange,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, [{}:{} - {}:{}]: {}", self.severity, self.position.start.line, self.position.start.char, self.position.end.line, self.position.end.char, self.message)
    }
}

pub enum ErrMsg {
    ErrUnknownSymbol(char),
    ErrUnterminatedString,
    ErrDecimalLiteralAsInt,
    ExpectedStatement,
    ExpectedExpression,
    ExpectedDeclaration,
    ExpectedAssignment,
    ExpectedBlock,
    ExpectedType,
    ExpectedArguments,
    ExpectedParameters,
    ExpectedStructName,
    ExpectedMemberName,
    ExpectedFnName,
    ExpectedToken(TokenType),
    UnexpectedToken,
    ExpectedItem,
    ExpectedArgName,
    ExpectedVar,
    ExpectedTypeNameOrIdentifier,
    CannotReferenceArrayType
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
            Self::ErrUnknownSymbol(x) => &format!("unknown symbol {}", x),
            Self::ErrUnterminatedString => "unterminated string",
            Self::ErrDecimalLiteralAsInt => "decimal literal cannot be used as int",
            Self::ExpectedDeclaration => "expected declaration",
            Self::ExpectedParameters => "expected parameters",
            Self::ExpectedArguments => "expected arguments",
            Self::ExpectedAssignment => "expected assignment",
            Self::ExpectedArgName => "expected arg name",
            Self::ExpectedBlock => "expected block",
            Self::ExpectedStatement => "expected statement",
            Self::ExpectedExpression => "expected expression",
            Self::ExpectedType => "expected type",
            Self::ExpectedStructName => "expected struct name",
            Self::ExpectedMemberName => "expected member name",
            Self::ExpectedFnName => "expected function name",
            Self::ExpectedToken(token) => &format!("expected '{}' token", token),
            Self::UnexpectedToken => "unexpected token",
            Self::ExpectedItem => "expected item",
            Self::ExpectedVar => "expected variable",
            Self::ExpectedTypeNameOrIdentifier => "expected type name or identifier",
            Self::CannotReferenceArrayType => "cannot reference array type",
        };

        write!(f, "{}", msg)
    }
}
