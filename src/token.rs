use std::{cmp, fmt};

use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: i32,
    pub char: i32
}

pub trait Positioned {
    fn get_position(&self) -> &PositionRange;
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.char)
    }
}

impl Position {
    pub fn new(line: i32, char: i32) -> Position {
        Position {line, char}
    }

    pub fn next_char(&mut self) {
        self.char += 1;
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.char = 1;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position
}

impl fmt::Display for PositionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Serialize for PositionRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("[{} - {}]", self.start, self.end))
    }
}

impl PositionRange {
    pub fn concat(a: &PositionRange, b: &PositionRange) -> PositionRange {
        let start = Position {
            line: cmp::min(a.start.line, b.start.line),
            char: cmp::min(a.start.char, b.start.char)
        };

        let end = Position {
            line: cmp::max(a.end.line, b.end.line),
            char: cmp::max(a.end.char, b.end.char)
        };

        PositionRange {start, end}
    }

    pub fn new(position: Position) -> PositionRange {
        PositionRange {
            start: position,
            end: position,
        }
    }

    pub fn with_end(mut self, position: Position) -> PositionRange {
        self.end = position;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub position: PositionRange,
}

impl Token {
    pub fn get_value_for_type(&self, token_type: TokenType) -> &TokenValue {
        if self.token_type == token_type {
            &self.value
        } else {
            &TokenValue::None
        }
    }

    pub fn get_int(&self) -> i64 {
        match self.value {
            TokenValue::Int(i) => i,
            _ => panic!("Token is not an integer")
        }
    }

    pub fn get_double(&self) -> f64 {
        match self.value {
            TokenValue::Double(d) => d,
            _ => panic!("Token is not a double")
        }
    }

    pub fn get_bool(&self) -> bool {
        match self.value {
            TokenValue::Bool(b) => b,
            _ => panic!("Token is not a boolean")
        }
    }

    pub fn get_string(&self) -> &String {
        match &self.value {
            TokenValue::String(s) => s,
            _ => panic!("Token is not a string")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    None,
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Semicolon, Dot, Comma, Colon, Assignment, DollarSign, AtSign,

    LeftCurly, RightCurly, 
    LeftSquare, RightSquare, 
    LeftParen, RightParen,

    Plus, Minus, Slash, Star, Ampersand,
    NotEqual, Equal, 
    Greater, GreaterEqual, 
    Less, LessEqual,
    And, Or, Not,

    Putc, Getc,
    Alloc,
    Arrow,
    Let,

    If, Else, For, Return, Fn, Rand, Main,
    This, While, Loop, Break, Print, Input,

    Int, Double, Bool, String,
    Struct,

    IntLiteral, FloatLiteral, DoubleLiteral, BoolLiteral, StringLiteral,
    Identifier,
    EOF,
    SOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Semicolon => write!(f, "';'"),
            TokenType::Dot => write!(f, "'.'"),
            TokenType::Comma => write!(f, "','"),
            TokenType::Colon => write!(f, "':'"),
            TokenType::Assignment => write!(f, "'='"),
            TokenType::DollarSign => write!(f, "$"),
            TokenType::AtSign => write!(f, "@"),

            TokenType::LeftCurly => write!(f, "'{{'"),
            TokenType::RightCurly => write!(f, "'}}'"),
            TokenType::LeftSquare => write!(f, "'['"),
            TokenType::RightSquare => write!(f, "']'"),
            TokenType::LeftParen => write!(f, "'('"),
            TokenType::RightParen => write!(f, "')'"),

            TokenType::Plus => write!(f, "'+'"),
            TokenType::Minus => write!(f, "'-'"),
            TokenType::Slash => write!(f, "'/'"),
            TokenType::Star => write!(f, "'*'"),
            TokenType::Ampersand => write!(f, "'&'"),
            TokenType::NotEqual => write!(f, "'!='"),
            TokenType::Equal => write!(f, "'=='"),
            TokenType::Greater => write!(f, "'>'"),
            TokenType::GreaterEqual => write!(f, "'>='"),
            TokenType::Less => write!(f, "'<'"),
            TokenType::LessEqual => write!(f, "'<='"),
            TokenType::And => write!(f, "'and'"),
            TokenType::Or => write!(f, "'or'"),
            TokenType::Not => write!(f, "'!'"),

            TokenType::Putc => write!(f, "'putc'"),
            TokenType::Getc => write!(f, "'getc'"),
            TokenType::Alloc => write!(f, "'alloc'"),
            TokenType::Arrow => write!(f, "'->'"),
            TokenType::Let => write!(f, "'let'"),

            TokenType::If => write!(f, "'if'"),
            TokenType::Else => write!(f, "'else'"),
            TokenType::For => write!(f, "'for'"),
            TokenType::Return => write!(f, "'return'"),
            TokenType::Fn => write!(f, "'fn'"),
            TokenType::Rand => write!(f, "'rand'"),
            TokenType::Main => write!(f, "'main'"),
            TokenType::This => write!(f, "'this'"),
            TokenType::While => write!(f, "'while'"),
            TokenType::Loop => write!(f, "'loop'"),
            TokenType::Break => write!(f, "'break'"),
            TokenType::Print => write!(f, "'print'"),
            TokenType::Input => write!(f, "'input'"),

            TokenType::Int => write!(f, "'int'"),
            TokenType::Double => write!(f, "'double'"),
            TokenType::Bool => write!(f, "'bool'"),
            TokenType::String => write!(f, "'string'"),
            TokenType::Struct => write!(f, "'struct'"),

            TokenType::IntLiteral => write!(f, "int literal"),
            TokenType::FloatLiteral => write!(f, "float literal"),
            TokenType::DoubleLiteral => write!(f, "double literal"),
            TokenType::BoolLiteral => write!(f, "boolean literal"),
            TokenType::StringLiteral => write!(f, "string literal"),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::SOF => write!(f, "SOF"),
        }
    }
}