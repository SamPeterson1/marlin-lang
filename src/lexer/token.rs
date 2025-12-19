use std::{cmp, fmt};
use serde::Serialize;
use std::mem::discriminant;

pub trait Positioned {
    fn get_position(&self) -> &PositionRange;
}

#[macro_export]
macro_rules! impl_positioned {
    ($struct_name:ty) => {
        impl $crate::lexer::token::Positioned for $struct_name {
            fn get_position(&self) -> &$crate::lexer::token::PositionRange {
                &self.position
            }
        }
    };
}

#[derive(Serialize, Clone)]
pub struct Located<T> {
    pub data: T,
    position: PositionRange,
}

impl<T> Located<T> {
    pub fn new(data: T, position: PositionRange) -> Self {
        Self { data, position }
    }
}

impl<T> Positioned for Located<T> {
    fn get_position(&self) -> &PositionRange {
        &self.position
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: i32,
    char: i32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.char)
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
    start: Position,
    end: Position
}

impl fmt::Display for PositionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
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
    #[allow(dead_code)]
    pub fn zero() -> PositionRange {
        PositionRange {
            start: Position { line: 0, char: 0 },
            end: Position { line: 0, char: 0 },
        }
    }

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
    pub value: TokenType,
    position: PositionRange,
}

impl Token {
    pub fn new(value: TokenType, position: PositionRange) -> Token {
        Token { value, position }
    }
}

impl Positioned for Token {
    fn get_position(&self) -> &PositionRange {
        &self.position
    }
}

impl Token {
    #[allow(dead_code)]
    pub fn unwrap_int_literal(self) -> Located<i64> {
        match self.value {
            TokenType::IntLiteral(value) => Located::new(value, self.position),
            _ => panic!("called unwrap_int_literal on non-integer literal token")
        }
    }

    #[allow(dead_code)]
    pub fn unwrap_double_literal(self) -> Located<f64> {
        match self.value {
            TokenType::DoubleLiteral(value) => Located::new(value, self.position),
            _ => panic!("called unwrap_double_literal on non-double literal token")
        }
    }

    #[allow(dead_code)]
    pub fn unwrap_bool_literal(self) -> Located<bool> {
        match self.value {
            TokenType::BoolLiteral(value) => Located::new(value, self.position),
            _ => panic!("called unwrap_bool_literal on non-boolean literal token")
        }
    }

    #[allow(dead_code)]
    pub fn unwrap_string_literal(self) -> Located<String> {
        match self.value {
            TokenType::StringLiteral(value) => Located::new(value, self.position),
            _ => panic!("called unwrap_string_literal on non-string literal token")
        }
    }

    pub fn unwrap_identifier(self) -> Located<String> {
        match self.value {
            TokenType::Identifier(value) => Located::new(value, self.position),
            _ => panic!("called unwrap_identifier on non-identifier token")
        }
    }
}


#[derive(Debug, Clone)]
pub enum TokenType {
    Semicolon, Colon, Dot, Comma, Assignment, DollarSign, Arrow,
    LeftCurly, RightCurly, LeftSquare, RightSquare, LeftParen, RightParen,
    Plus, Minus, Slash, Star, Ampersand,
    NotEqual, Equal, Greater, GreaterEqual, Less, LessEqual,
    And, Or, Not, Tilda, Bar,

    New, Let, Delete,
    If, Else, For, Fn, Main,
    While, Loop, Break, Result, Return, Impl,

    Int, Double, Bool, Char,
    Struct,

    IntLiteral(i64), DoubleLiteral(f64), BoolLiteral(bool), CharLiteral(char), StringLiteral(String),
    Identifier(String),

    // Wildcard types for comparisons
    #[allow(dead_code)]
    AnyIntLiteral,

    #[allow(dead_code)] 
    AnyDoubleLiteral,

    #[allow(dead_code)]
    AnyBoolLiteral, 

    #[allow(dead_code)]
    AnyStringLiteral, 

    #[allow(dead_code)]
    AnyCharLiteral, 

    AnyIdentifier,
    EOF
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        
        if discriminant(self) == discriminant(other) {
            match (self, other) {
                (TokenType::IntLiteral(a), TokenType::IntLiteral(b)) => a == b,
                (TokenType::DoubleLiteral(a), TokenType::DoubleLiteral(b)) => a == b,
                (TokenType::BoolLiteral(a), TokenType::BoolLiteral(b)) => a == b,
                (TokenType::CharLiteral(a), TokenType::CharLiteral(b)) => a == b,
                (TokenType::StringLiteral(a), TokenType::StringLiteral(b)) => a == b,
                (TokenType::Identifier(a), TokenType::Identifier(b)) => a == b,
                _ => true,
            }
        } else {
            match (self, other) {
                (TokenType::IntLiteral(_), TokenType::AnyIntLiteral) => true,
                (TokenType::AnyIntLiteral, TokenType::IntLiteral(_)) => true,
                (TokenType::DoubleLiteral(_), TokenType::AnyDoubleLiteral) => true,
                (TokenType::AnyDoubleLiteral, TokenType::DoubleLiteral(_)) => true,
                (TokenType::BoolLiteral(_), TokenType::AnyBoolLiteral) => true,
                (TokenType::AnyBoolLiteral, TokenType::BoolLiteral(_)) => true,
                (TokenType::CharLiteral(_), TokenType::AnyCharLiteral) => true,
                (TokenType::AnyCharLiteral, TokenType::CharLiteral(_)) => true,
                (TokenType::StringLiteral(_), TokenType::AnyStringLiteral) => true,
                (TokenType::AnyStringLiteral, TokenType::StringLiteral(_)) => true,
                (TokenType::Identifier(_), TokenType::AnyIdentifier) => true,
                (TokenType::AnyIdentifier, TokenType::Identifier(_)) => true,
                _ => false,
            }
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::Tilda => "~",
            TokenType::Bar => "|",
            TokenType::Semicolon => ";",
            TokenType::Colon => ":",
            TokenType::Dot => ".",
            TokenType::Comma => ",",
            TokenType::Assignment => "=",
            TokenType::DollarSign => "$",
            TokenType::Arrow => "->",
            TokenType::LeftCurly => "{",
            TokenType::RightCurly => "}",
            TokenType::LeftSquare => "[",
            TokenType::RightSquare => "]",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Ampersand => "&",
            TokenType::NotEqual => "!=",
            TokenType::Equal => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::And => "and",
            TokenType::Or => "or",
            TokenType::Not => "!",
            TokenType::New => "new",
            TokenType::Let => "let",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::For => "for",
            TokenType::Fn => "fn",
            TokenType::Main => "main",
            TokenType::Delete => "delete",
            TokenType::While => "while",
            TokenType::Loop => "loop",
            TokenType::Break => "break",
            TokenType::Result => "result",
            TokenType::Return => "return",
            TokenType::Impl => "impl",
            TokenType::Int => "int",
            TokenType::Double => "double",
            TokenType::Bool => "bool",
            TokenType::Char => "char",
            TokenType::Struct => "struct",
            TokenType::IntLiteral(_) | TokenType::AnyIntLiteral => "integer literal",
            TokenType::DoubleLiteral(_) | TokenType::AnyDoubleLiteral => "double literal",
            TokenType::BoolLiteral(_) | TokenType::AnyBoolLiteral => "boolean literal",
            TokenType::CharLiteral(_) | TokenType::AnyCharLiteral => "char literal",
            TokenType::StringLiteral(_) | TokenType::AnyStringLiteral => "string literal",
            TokenType::Identifier(_) | TokenType::AnyIdentifier => "identifier",
            TokenType::EOF => "end of file",
        };
        write!(f, "{}", name)
    }
}

