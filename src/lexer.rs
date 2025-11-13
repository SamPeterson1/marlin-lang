use std::{iter::Peekable, str::Chars};

use crate::error::{self, Diagnostic, DiagnosticType};
use crate::logger::{LogSource, Logger};
use crate::token::{Position, PositionRange};
use crate::token::{Token, TokenType, TokenValue};

pub struct LexerDiagnostic {
    fatal: bool,
    diagnostic: Diagnostic,
}

impl LexerDiagnostic {
    fn new(diagnostic: Diagnostic) -> LexerDiagnostic {
        LexerDiagnostic {
            fatal: false,
            diagnostic,
        }
    }

    fn new_fatal(diagnostic: Diagnostic) -> LexerDiagnostic {
        LexerDiagnostic {
            fatal: true,
            diagnostic,
        }
    }
}

pub fn parse(code: &str) -> Vec<Token> {
    let mut tokens = vec![Token {
        token_type: TokenType::SOF,
        value: TokenValue::None,
        position: PositionRange::new(Position::new(0, 0)),
    }];

    let mut lexer = Lexer::new(code);

    while let Some(token) = lexer.next_token() {
        let is_eof = token.token_type == TokenType::EOF;

        Logger::log_debug(&lexer, format!("Parsed token: {:?}", token).as_str());
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    Logger::log_info(&lexer, &format!("Parsed {} tokens", tokens.len()));

    tokens
}

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub cur: Option<char>,
    pub position: Position,
    pub index: i32,
    pub token_start: Option<PositionRange>,
}

impl LogSource for Lexer<'_> {
    fn get_source(&self) -> String {
        "Lexer".to_string()
    }
}

impl Lexer<'_> {
    fn new(code: &str) -> Lexer {
        let mut chars = code.chars().peekable();
        let cur = chars.next();

        Lexer {
            chars,
            cur,
            position: Position::new(1, 1),
            index: 0,
            token_start: None,
        }
    }

    fn err_unknown_symbol(&self) -> LexerDiagnostic {
        let msg = format!("unknown symbol {}", self.cur.unwrap());
        let pos = PositionRange::new(self.position);
        let diagnostic = Diagnostic::new(error::LEX_ERR_UNKNOWN_SYMBOL, DiagnosticType::Error, pos, msg);

        LexerDiagnostic::new(diagnostic)
    }

    fn err_unclosed_quotes(&self, start: Position) -> LexerDiagnostic {
        let msg = "unterminated string".to_string();
        let pos = PositionRange::new(start).with_end(self.position);
        let diagnostic = Diagnostic::new(error::LEX_ERR_UNTERMINATED_STRING, DiagnosticType::Error, pos, msg);

        LexerDiagnostic::new_fatal(diagnostic)
    }

    fn err_decimal_literal_as_int(&self) -> LexerDiagnostic {
        let msg = "decimal literal cannot be used as int".to_string();
        let pos = PositionRange::new(self.position);
        let diagnostic = Diagnostic::new(error::LEX_ERR_DECIMAL_LITERAL_AS_INT, DiagnosticType::Error, pos, msg);

        LexerDiagnostic::new(diagnostic)
    }

    fn begin_token(&mut self) {
        self.token_start = Some(PositionRange::new(self.position));
    }

    fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.cur = self.chars.next();

        if self.cur == Some('\n') {
            self.position.next_line();
        } else {
            self.position.next_char();
        }

        self.cur
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn end_token(&mut self, token_type: TokenType, value: TokenValue) -> Result<Token, LexerDiagnostic> {
        let token_start = self.token_start.take().expect("There is no token to end");

        let ret = Ok(Token {
            token_type,
            value,
            position: token_start.with_end(self.position)
        });



        self.token_start = None;

        self.next();

        ret
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            self.begin_token();

            match self.try_next_token() {
                Ok(token) => break Some(token),
                Err(err) => {
                    if err.fatal {
                        break None;
                    }

                    self.next();
                }
            }
        }
    }

    fn try_next_token(&mut self) -> Result<Token, LexerDiagnostic> {
        let mut cur = match self.cur {
            Some(cur) => cur,
            None => return self.end_token(TokenType::EOF, TokenValue::None)
        };

        while Self::is_whitespace(cur) {
            if let Some(c) = self.next() {
                cur = c;
            } else {
                return self.end_token(TokenType::EOF, TokenValue::None);
            }
        }
        
        match cur {
            ';' => self.end_token(TokenType::Semicolon, TokenValue::None),
            ',' => self.end_token(TokenType::Comma, TokenValue::None),
            '.' => self.end_token(TokenType::Dot, TokenValue::None),
            ':' => self.end_token(TokenType::Colon, TokenValue::None),
            '{' => self.end_token(TokenType::LeftCurly, TokenValue::None),
            '}' => self.end_token(TokenType::RightCurly, TokenValue::None),
            '(' => self.end_token(TokenType::LeftParen, TokenValue::None),
            ')' => self.end_token(TokenType::RightParen, TokenValue::None),
            '[' => self.end_token(TokenType::LeftSquare, TokenValue::None),
            ']' => self.end_token(TokenType::RightSquare, TokenValue::None),
            '+' => self.end_token(TokenType::Plus, TokenValue::None),
            '*' => self.end_token(TokenType::Star, TokenValue::None),
            '-' => self.parse_pair('>', TokenType::Minus, TokenType::Arrow),
            '/' => self.parse_slash(),
            '!' => self.parse_pair('=', TokenType::Not, TokenType::NotEqual),
            '>' => self.parse_pair('=', TokenType::Greater, TokenType::GreaterEqual),
            '<' => self.parse_pair('=', TokenType::Less, TokenType::LessEqual),
            '=' => self.parse_pair('=', TokenType::Assignment, TokenType::Equal),
            '&' => self.end_token(TokenType::Ampersand, TokenValue::None),
            '\"' => self.parse_string(),
            _ => self.parse_other(),
        }
    }

    fn parse_other(&mut self) -> Result<Token, LexerDiagnostic> {
        let cur = self.cur.unwrap();

        if Self::is_alphabetic(cur) {
            self.parse_alphabetic()
        } else if Self::is_numeric(cur) {
            self.parse_numeric()
        } else {
            Err(self.err_unknown_symbol())
        }
    }

    fn parse_pair(&mut self, second_char: char, single: TokenType, paired: TokenType) -> Result<Token, LexerDiagnostic> {
        let peek = self.peek();
        
        if let Some(&peek) = peek {
            if peek == second_char {
                self.next();
                self.end_token(paired, TokenValue::None)
            } else {
                self.end_token(single, TokenValue::None)
            }
        } else {
            self.end_token(single, TokenValue::None)
        }
    }

    fn parse_slash(&mut self) -> Result<Token, LexerDiagnostic> {
        match self.peek().unwrap_or(&'\n') {
            '/' => {
                while let Some(c) = self.next() {
                    if c == '\n' { 
                        break; 
                    }
                }

                self.try_next_token()
            },
            _ => self.end_token(TokenType::Slash, TokenValue::None)
        }
    }
    
    fn parse_string(&mut self) -> Result<Token, LexerDiagnostic> {
        let mut value = String::new();

        let start = self.position;

        let mut string_ended = false;
    
        while let Some(c) = self.next() {
            if c == '\"' {
                string_ended = true;
                break; 
            }
    
            value.push(c);
        }

        if !string_ended {
            return Err(self.err_unclosed_quotes(start))
        }
    
        self.end_token(TokenType::StringLiteral, TokenValue::String(value))
    }

    fn parse_numeric(&mut self) -> Result<Token, LexerDiagnostic> {
        let mut whole_part: u64 = 0;
        let mut frac_part: u64 = 0;
    
        let mut is_decimal = false;
        let mut decimal_places = 0;
    
        loop {
            let cur = self.cur.unwrap();

            if is_decimal {
                frac_part = 10 * frac_part + cur.to_digit(10).unwrap() as u64;
                decimal_places += 1;
            } else {
                whole_part = 10 * whole_part + cur.to_digit(10).unwrap() as u64;
            }
    
            if let Some(&peek) = self.peek() {
                if peek == '.' { 
                    if !is_decimal {
                        is_decimal = true;

                        self.next();
                        self.next();
                    } else {
                        break;
                    }
                } else if !Self::is_numeric(peek) {
                    break;
                } else {
                    self.next();
                }
            } else {
                break;
            }
        }

    
        let value = whole_part as f64 + frac_part as f64 / (10_i32.pow(decimal_places) as f64);
        
        let peek = self.peek().cloned();

        match peek {
            Some('d') => {
                self.next();
                self.end_token(TokenType::DoubleLiteral, TokenValue::Double(value))
            },
            Some('i') => {
                self.next();
                if !is_decimal {
                    self.end_token(TokenType::IntLiteral, TokenValue::Int(value as i64))
                } else {
                    Result::Err(self.err_decimal_literal_as_int())
                }
            },
            _ => {
                if !is_decimal {
                    self.end_token(TokenType::IntLiteral, TokenValue::Int(value as i64))
                } else {
                    self.end_token(TokenType::FloatLiteral, TokenValue::Double(value))
                }
            },
        }
    }
    
    fn parse_alphabetic(&mut self) -> Result<Token, LexerDiagnostic> {
        let mut word = String::new();
    
        loop {
            let cur = self.cur.unwrap();

            word.push(cur);
    
            if let Some(&peek) = self.peek() {
                if !Self::is_alphanumeric(peek) {
                    break;
                }
                
                self.next();
            } else {
                break;
            }
        }
    
        match word.as_str() {
            "if" => self.end_token(TokenType::If, TokenValue::None),
            "else" => self.end_token(TokenType::Else, TokenValue::None),
            "for" => self.end_token(TokenType::For, TokenValue::None),
            "return" => self.end_token(TokenType::Return, TokenValue::None),
            "fn" => self.end_token(TokenType::Fn, TokenValue::None),
            "this" => self.end_token(TokenType::This, TokenValue::None),
            "true" => self.end_token(TokenType::BoolLiteral, TokenValue::Bool(true)),
            "false" => self.end_token(TokenType::BoolLiteral,TokenValue::Bool(false)),
            "and" => self.end_token(TokenType::And, TokenValue::None),
            "or" => self.end_token(TokenType::Or, TokenValue::None),
            "while" => self.end_token(TokenType::While, TokenValue::None),
            "break" => self.end_token(TokenType::Break, TokenValue::None),
            "loop" => self.end_token(TokenType::Loop, TokenValue::None),
            "int" => self.end_token(TokenType::Int, TokenValue::None),
            "double" => self.end_token(TokenType::Double, TokenValue::None),
            "bool" => self.end_token(TokenType::Bool, TokenValue::None),
            "str" => self.end_token(TokenType::String, TokenValue::None),
            "let" => self.end_token(TokenType::Let, TokenValue::None),
            "print" => self.end_token(TokenType::Print, TokenValue::None),
            "rand" => self.end_token(TokenType::Rand, TokenValue::None),
            "input" => self.end_token(TokenType::Input, TokenValue::None),
            "struct" => self.end_token(TokenType::Struct, TokenValue::None),
            "alloc" => self.end_token(TokenType::Alloc, TokenValue::None),
            "putc" => self.end_token(TokenType::Putc, TokenValue::None),
            "getc" => self.end_token(TokenType::Getc, TokenValue::None),
            _ => self.end_token(TokenType::Identifier, TokenValue::String(word)),
        }
    }
    
    fn is_whitespace(c: char) -> bool {
        c == '\t' || c == '\n' || c == '\r' || c == ' '
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alphabetic(c) || Self::is_numeric(c)
    }
    
    fn is_alphabetic(c: char) -> bool {
        (c >= 'A' && c <= 'Z')
        || (c >= 'a' && c <= 'z')
        || (c == '_')
    }
    
    fn is_numeric(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}