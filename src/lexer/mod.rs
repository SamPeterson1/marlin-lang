pub mod token;

#[cfg(test)]
pub mod tests;

use std::{iter::Peekable, str::Chars};

use crate::logger::Log;
use crate::diagnostic::{Diagnostic, ErrMsg};
use crate::lexer::token::{Position, PositionRange};
use crate::lexer::token::{Token, TokenType};

pub struct Lexer<'ch, 'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
    chars: Peekable<Chars<'ch>>,
    next_position: Position, // Position of the next character to be read
    token_range: PositionRange, // Range of the current token being parsed
}

impl Log for Lexer<'_, '_> {
    fn get_source(&self) -> String {
        "Lexer".to_string()
    }
}

impl<'ch, 'diag> Lexer<'ch, 'diag> {
    pub fn new(code: &'ch str, diagnostics: &'diag mut Vec<Diagnostic>) -> Lexer<'ch, 'diag> {
        let chars = code.chars().peekable();
        let next_position = Position::new(1, 1);
        
        Lexer {
            diagnostics,
            chars,
            next_position,
            token_range: PositionRange::new(next_position),
        }
    }

    // Parses the input code into a vector of tokens
    // Ends with an EOF token
    pub fn parse(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.peek().is_some() {
            if let Some(token) = self.next_token() {
                self.log_debug(&format!("Parsed token: {:?}", token));
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenType::EOF,
            PositionRange::new(self.next_position),
        ));

        self.log_info(&format!("Parsed {} tokens", tokens.len()));

        tokens
    }

    fn next_char(&mut self) -> Option<char> {
        self.next().map(|(c, _)| c)
    }

    fn next(&mut self) -> Option<(char, Position)> {
        let char = self.chars.next();
        let position = self.next_position;

        self.token_range = self.token_range.with_end(position);

        if char == Some('\n') {
            self.next_position.next_line();
        } else {
            self.next_position.next_char();
        }

        Some((char?, position))
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn begin_token(&mut self, start: Position) {
        self.token_range = PositionRange::new(start);
    }

    // Completes the current token and returns it after consuming the next character
    fn end_token_consume(&mut self, value: TokenType) -> Token {
        self.next();

        Token::new(value, self.token_range)
    }
    
    // Completes the current token without consuming any characters
    fn end_token(&mut self, value: TokenType) -> Token {
        Token::new(value, self.token_range)
    }

    fn next_token(&mut self) -> Option<Token> {
        // Consume whitespaces
        loop {
            if !Self::is_whitespace(*self.peek()?) {
                break;
            } else {
                self.next();
            }
        }

        // Only start parsing token after whitespace
        self.begin_token(self.next_position);
        
        match *self.peek()? {
            ':' => Some(self.end_token_consume(TokenType::Colon)),
            '$' => Some(self.end_token_consume(TokenType::DollarSign)),
            ';' => Some(self.end_token_consume(TokenType::Semicolon)),
            ',' => Some(self.end_token_consume(TokenType::Comma)),
            '.' => Some(self.end_token_consume(TokenType::Dot)),
            '{' => Some(self.end_token_consume(TokenType::LeftCurly)),
            '}' => Some(self.end_token_consume(TokenType::RightCurly)),
            '(' => Some(self.end_token_consume(TokenType::LeftParen)),
            ')' => Some(self.end_token_consume(TokenType::RightParen)),
            '[' => Some(self.end_token_consume(TokenType::LeftSquare)),
            ']' => Some(self.end_token_consume(TokenType::RightSquare)),
            '+' => Some(self.end_token_consume(TokenType::Plus)),
            '*' => Some(self.end_token_consume(TokenType::Star)),
            '&' => Some(self.end_token_consume(TokenType::Ampersand)),
            '-' => Some(self.parse_pair('>', TokenType::Minus, TokenType::Arrow)),
            '!' => Some(self.parse_pair('=', TokenType::Not, TokenType::NotEqual)),
            '>' => Some(self.parse_pair('=', TokenType::Greater, TokenType::GreaterEqual)),
            '<' => Some(self.parse_pair('=', TokenType::Less, TokenType::LessEqual)),
            '=' => Some(self.parse_pair('=', TokenType::Assignment, TokenType::Equal)),
            '/' => self.parse_slash(),
            '\"' => self.parse_string(),
            '\'' => self.parse_char(),
            peek => self.parse_other(peek),
        }
    }

    // Parses either numeric literals or identifiers/keywords
    fn parse_other(&mut self, peek: char) -> Option<Token> {
        if Self::is_alphabetic(peek) {
            Some(self.parse_alphabetic())
        } else if Self::is_numeric(peek) {
            self.parse_numeric()
        } else {
            let cur = self.next_char()?;
            self.diagnostics.push(ErrMsg::UnknownSymbol(cur).make_diagnostic(self.token_range));
            None
        }
    }

    // Parses either a single character token or a paired character token
    fn parse_pair(&mut self, second_char: char, single: TokenType, paired: TokenType) -> Token {
        self.next(); // Consume first character

        let peek = self.peek();
        
        if let Some(&peek) = peek {
            if peek == second_char {
                self.next();
                self.end_token(paired)
            } else {
                self.end_token(single)
            }
        } else {
            self.end_token(single)
        }
    }

    // Parses either a comment or a slash
    // Must be called when the peeked character is a slash
    fn parse_slash(&mut self) -> Option<Token> {
        self.next(); // Consume first slash

        match self.peek() {
            Some('/') => {
                // It's a comment, consume until end of line
                self.log_debug("Parsing comment");
                while let Some(c) = self.next_char() {
                    if c == '\n' { 
                        self.log_debug(&format!("End of comment reached {}", c));
                        break;
                    }
                }

                None
            },
            _ => Some(self.end_token(TokenType::Slash))
        }
    }

    // Parses an escape sequence
    fn parse_escape(&mut self) -> Option<char> {
        let (_, slash_pos) = self.next()?;
        let (c, c_pos) = self.next()?;

        match c {
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '\'' => Some('\''),
            '\\' => Some('\\'),
            '\"' => Some('\"'),
            other => {
                let diagnostic_pos = PositionRange::new(slash_pos).with_end(c_pos);
                self.diagnostics.push(ErrMsg::UnknownEscapeSequence(other).make_diagnostic(diagnostic_pos));
                None
            },
        }
    }

    // Must be called when the peeked character is a single quote
    fn parse_char(&mut self) -> Option<Token> {
        self.next(); // Consume opening quote

        let c = match self.peek() {
            Some('\\') => self.parse_escape(),
            Some(_) => Some(self.next_char().unwrap()),
            _ => return None
        };

        if self.peek() != Some(&'\'') {
            self.diagnostics.push(ErrMsg::UnterminatedChar.make_diagnostic(self.token_range));
        } else {
            self.next(); // Consume closing quote
        }

        Some(self.end_token(TokenType::CharLiteral(c?)))
    }

    // Must be called when the peeked character is a quote
    fn parse_string(&mut self) -> Option<Token> {
        let mut value = String::new();

        self.next(); // Consume opening quote

        let mut string_ended = false;
    
        while let Some(&c) = self.peek() {
            if c == '\\' {
                if let Some(escaped_char) = self.parse_escape() {
                    value.push(escaped_char);
                }
            } else if c == '\"' {
                self.next(); // Consume closing quote
                string_ended = true;
                break; 
            } else {
                value.push(self.next_char().unwrap());
            }
        }

        if !string_ended {
            self.diagnostics.push(ErrMsg::UnterminatedString.make_diagnostic(self.token_range));
            return None;
        }
    
        Some(self.end_token(TokenType::StringLiteral(value)))
    }

    // Parses a numeric literal
    // Must be called when the peeked character is numeric
    fn parse_numeric(&mut self) -> Option<Token> {
        let mut whole_part: u64 = 0;
        let mut frac_part: u64 = 0;
    
        let mut is_decimal = false;
        let mut decimal_places = 0;
    
        while let Some(cur) = self.next_char() {
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
                    } else {
                        break;
                    }
                } else if !Self::is_numeric(peek) {
                    break;
                }
            }
        }

    
        let value = whole_part as f64 + frac_part as f64 / (10_i32.pow(decimal_places) as f64);
        
        let peek = self.peek().cloned();

        match peek {
            Some('d') => {
                self.next();
                Some(self.end_token(TokenType::DoubleLiteral(value)))
            },
            Some('i') => {
                self.next();
                if !is_decimal {
                    Some(self.end_token(TokenType::IntLiteral(value as i64)))
                } else {
                    self.diagnostics.push(ErrMsg::DecimalLiteralAsInt.make_diagnostic(self.token_range));
                    None
                }
            },
            _ => {
                if !is_decimal {
                    Some(self.end_token(TokenType::IntLiteral(value as i64)))
                } else {
                    Some(self.end_token(TokenType::DoubleLiteral(value)))
                }
            },
        }
    }
    
    // Parses keywords or identifiers
    fn parse_alphabetic(&mut self) -> Token {
        let mut word = String::new();
    
        while let Some(cur) = self.next_char() {
            word.push(cur);
    
            if let Some(&peek) = self.peek() {
                if !Self::is_alphanumeric(peek) {
                    break;
                }                
            } else {
                break;
            }
        }
    
        match word.as_str() {
            "main" => self.end_token(TokenType::Main),
            "delete" => self.end_token(TokenType::Delete),
            "result" => self.end_token(TokenType::Result),
            "impl" => self.end_token(TokenType::Impl),
            "if" => self.end_token(TokenType::If),
            "else" => self.end_token(TokenType::Else),
            "for" => self.end_token(TokenType::For),
            "return" => self.end_token(TokenType::Return),
            "fn" => self.end_token(TokenType::Fn),
            "true" => self.end_token(TokenType::BoolLiteral(true)),
            "false" => self.end_token(TokenType::BoolLiteral(false)),
            "and" => self.end_token(TokenType::And),
            "or" => self.end_token(TokenType::Or),
            "while" => self.end_token(TokenType::While),
            "break" => self.end_token(TokenType::Break),
            "loop" => self.end_token(TokenType::Loop),
            "int" => self.end_token(TokenType::Int),
            "double" => self.end_token(TokenType::Double),
            "bool" => self.end_token(TokenType::Bool),
            "char" => self.end_token(TokenType::Char),
            "let" => self.end_token(TokenType::Let),
            "struct" => self.end_token(TokenType::Struct),
            "new" => self.end_token(TokenType::New),
            _ => self.end_token(TokenType::Identifier(word)),
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