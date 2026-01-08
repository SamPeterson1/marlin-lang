pub mod token;

#[cfg(test)]
pub mod tests;

use std::{iter::Peekable, str::Chars};

use crate::logger::{Log, LogTarget};
use crate::diagnostic::{Diagnostic, ErrMsg};
use crate::lexer::token::{Position, PositionRange};
use crate::lexer::token::{Token, TokenType};

pub struct Lexer<'ctx> {
    log_target: &'ctx dyn LogTarget,
    diagnostics: &'ctx mut Vec<Diagnostic>,
    chars: Peekable<Chars<'ctx>>,
    next_position: Position, // Position of the next character to be read
    token_range: PositionRange, // Range of the current token being parsed
}

impl Log for Lexer<'_> {
    fn get_source(&self) -> String {
        format!("Lexer")
    }
}

impl<'ctx> Lexer<'ctx> {
    pub fn new(log_target: &'ctx dyn LogTarget, code: &'ctx str, diagnostics: &'ctx mut Vec<Diagnostic>) -> Lexer<'ctx> {
        let chars = code.chars().peekable();
        let next_position = Position::new(1, 1);
        
        Lexer {
            log_target,
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
                self.log_debug(self.log_target, format!("Parsed token: {:?}", token));
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenType::EOF,
            PositionRange::new(self.next_position),
        ));

        self.log_debug(self.log_target, "Reached end of file");
        self.log_info(self.log_target, format!("Parsed {} tokens", tokens.len()));

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
            '^' => Some(self.end_token_consume(TokenType::Carat)),
            '%' => Some(self.end_token_consume(TokenType::Percentage)),
            '~' => Some(self.end_token_consume(TokenType::Tilda)),
            '|' => Some(self.end_token_consume(TokenType::Bar)),
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
            ':' => Some(self.parse_pair(&[(':', TokenType::DoubleColon)], TokenType::Colon)),
            '-' => Some(self.parse_pair(&[('>', TokenType::Arrow)], TokenType::Minus)),
            '!' => Some(self.parse_pair(&[('=', TokenType::NotEqual)], TokenType::Not)),
            '>' => Some(self.parse_pair(&[('=', TokenType::GreaterEqual), ('>', TokenType::RightShift)], TokenType::Greater)),
            '<' => Some(self.parse_pair(&[('=', TokenType::LessEqual), ('<', TokenType::LeftShift)], TokenType::Less)),
            '=' => Some(self.parse_pair(&[('=', TokenType::Equal)], TokenType::Assignment)),
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
        } else if Self::is_numeric(peek, 10) {
            self.parse_numeric()
        } else {
            let cur = self.next_char()?;
            self.diagnostics.push(ErrMsg::UnknownSymbol(cur).make_diagnostic(self.token_range));
            None
        }
    }

    // Parses either a single character token or a paired character token
    fn parse_pair(&mut self, pairs: &[(char, TokenType)], single: TokenType) -> Token {
        self.next(); // Consume first character

        let peek = self.peek();
        
        if let Some(&peek) = peek {
            for (second_char, paired) in pairs {
                if peek == *second_char {
                    return self.end_token_consume(paired.clone());
                }
            }
            self.end_token(single)
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
                self.log_debug(self.log_target, "Parsing comment");
                while let Some(c) = self.next_char() {
                    if c == '\n' { 
                        self.log_debug(self.log_target, format!("End of comment reached {}", c));
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

        let mut radix: u64 = 10;

        if self.peek() == Some(&'0') {
            self.next(); // Consume '0'
            match self.peek() {
                Some('x') => {
                    radix = 16;
                    self.next(); // Consume 'x'
                },
                Some('b') => {
                    radix = 2;
                    self.next(); // Consume 'b'
                },
                Some('o') => {
                    radix = 8;
                    self.next(); // Consume 'o'
                },
                _ => {}
            }
        }
    
        while let Some(peek) = self.peek() {
            if *peek == '.' { 
                if !is_decimal {
                    is_decimal = true;

                    self.next();
                } else {
                    break;
                }
            } else if !Self::is_numeric(*peek, radix as u32) {
                break;
            }
            
            let cur = self.next_char().unwrap();

            if is_decimal {
                frac_part = radix * frac_part + cur.to_digit(radix as u32).unwrap() as u64;
                decimal_places += 1;
            } else {
                whole_part = radix * whole_part + cur.to_digit(radix as u32).unwrap() as u64;
            }
        }

    
        let value = whole_part as f64 + frac_part as f64 / (radix.pow(decimal_places) as f64);
        
        if self.peek() == Some(&'_') {
            self.next(); // Consume suffix underscore
        }

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
            "extern" => self.end_token(TokenType::Extern),
            "scope" => self.end_token(TokenType::Scope),
            "from" => self.end_token(TokenType::From),
            "require" => self.end_token(TokenType::Require),
            "void" => self.end_token(TokenType::Void),
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
            "as" => self.end_token(TokenType::As),
            _ => self.end_token(TokenType::Identifier(word)),
        }
    }
    
    fn is_whitespace(c: char) -> bool {
        c == '\t' || c == '\n' || c == '\r' || c == ' '
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alphabetic(c) || Self::is_numeric(c, 10)
    }
    
    fn is_alphabetic(c: char) -> bool {
        (c >= 'A' && c <= 'Z')
        || (c >= 'a' && c <= 'z')
        || (c == '_')
    }
    
    fn is_numeric(c: char, radix: u32) -> bool {
        c.to_digit(radix).is_some()
    }
}