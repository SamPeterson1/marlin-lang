use crate::token::PositionRange;
use std::fmt;

/*
pub struct TypeError {
    msg: String
}

impl TypeError {
    pub fn new_unary(value_type: &Type, operator_name: &str) -> TypeError {
        let msg = format!("Invalid type {:?} for operator {}", value_type, operator_name);

        TypeError {msg}
    }

    pub fn new_binary(left_type: &Type, right_type: &Type, operator_name: &str) -> TypeError {
        let msg = format!("Invalid types {:?}, {:?} for operator {}", left_type, right_type, operator_name);

        TypeError {msg}
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
*/

pub enum DiagnosticType {
    Error,
    Warning
}

impl fmt::Display for DiagnosticType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiagnosticType::Error => write!(f, "ERROR"),
            DiagnosticType::Warning => write!(f, "WARNING")
        }
    }
}

pub const LEX_ERR_UNKNOWN_SYMBOL: i32 = 1;
pub const LEX_ERR_UNTERMINATED_STRING: i32 = 2;
pub const LEX_ERR_DECIMAL_LITERAL_AS_INT: i32 = 3;

pub struct Diagnostic {
    pub err_code: i32,
    pub diagnostic_type: DiagnosticType,
    pub position: PositionRange,
    pub msg: String
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: [{}] {}", self.diagnostic_type, self.position, self.msg)
    }
}

impl Diagnostic {
    pub fn new(err_code: i32, diagnostic_type: DiagnosticType, position: PositionRange, msg: String) -> Diagnostic {
        Diagnostic {
            err_code,
            diagnostic_type,
            position,
            msg
        }
    }
}

