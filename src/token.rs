use std::{cmp, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: i32,
    pub char: i32
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
        write!(f, "{}- {}", self.start, self.end)
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
    pub value: Option<TokenValue>,
    pub position: PositionRange,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Semicolon, Dot, Comma, Colon, Assignment,

    LeftCurly, RightCurly, 
    LeftSquare, RightSquare, 
    LeftParen, RightParen,

    Plus, Minus, Slash, Star, 
    NotEqual, Equal, 
    Greater, GreaterEqual, 
    Less, LessEqual,
    And, Or, Not,

    Arrow,
    Let,

    If, Else, For, Return, Fn, Rand,
    This, While, Loop, Break, Print, Input,

    Int, Double, Bool, String, Func,
    Struct,

    IntLiteral, FloatLiteral, DoubleLiteral, BoolLiteral, StringLiteral,
    Identifier,
    EOF,
}