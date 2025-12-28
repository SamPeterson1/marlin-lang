use crate::diagnostic::Diagnostic;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::Positioned;
use crate::lexer::Lexer;
use crate::logger::CONSOLE_LOGGER;

fn tokenize(code: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut diagnostics = Vec::new();
    let lexer = Lexer::new(&CONSOLE_LOGGER, code, &mut diagnostics);
    let tokens = lexer.parse();
    (tokens, diagnostics)
}

#[test]
fn test_single_character_tokens() {
    let (tokens, _) = tokenize("$:;,.{}()[]+-*&%^");
    
    let expected = vec![
        TokenType::DollarSign,
        TokenType::Colon,
        TokenType::Semicolon,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::LeftCurly,
        TokenType::RightCurly,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftSquare,
        TokenType::RightSquare,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Ampersand,
        TokenType::Percentage,
        TokenType::Carat,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_paired_character_tokens() {
    let (tokens, _) = tokenize("-> != >= <= == << >>");
    
    let expected = vec![
        TokenType::Arrow,
        TokenType::NotEqual,
        TokenType::GreaterEqual,
        TokenType::LessEqual,
        TokenType::Equal,
        TokenType::LeftShift,
        TokenType::RightShift,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_single_vs_paired_tokens() {
    let (tokens, _) = tokenize("- -> ! != > >= < <= = == << >>");
    
    let expected = vec![
        TokenType::Minus,
        TokenType::Arrow,
        TokenType::Not,
        TokenType::NotEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Assignment,
        TokenType::Equal,
        TokenType::LeftShift,
        TokenType::RightShift,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_keywords() {
    let (tokens, _) = tokenize("main delete result impl if else for return fn while break loop let struct new");
    
    let expected = vec![
        TokenType::Main,
        TokenType::Delete,
        TokenType::Result,
        TokenType::Impl,
        TokenType::If,
        TokenType::Else,
        TokenType::For,
        TokenType::Return,
        TokenType::Fn,
        TokenType::While,
        TokenType::Break,
        TokenType::Loop,
        TokenType::Let,
        TokenType::Struct,
        TokenType::New,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_type_keywords() {
    let (tokens, _) = tokenize("int double bool char");
    
    let expected = vec![
        TokenType::Int,
        TokenType::Double,
        TokenType::Bool,
        TokenType::Char,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_boolean_keywords() {
    let (tokens, _) = tokenize("true false and or");
    
    let expected = vec![
        TokenType::BoolLiteral(true),
        TokenType::BoolLiteral(false),
        TokenType::And,
        TokenType::Or,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_identifiers() {
    let (tokens, _) = tokenize("variable_name _underscore camelCase PascalCase snake_case");
    
    assert_eq!(tokens.len(), 6); // 5 identifiers + EOF
    
    match &tokens[0].value {
        TokenType::Identifier(name) => assert_eq!(name, "variable_name"),
        _ => panic!("Expected identifier"),
    }
    
    match &tokens[1].value {
        TokenType::Identifier(name) => assert_eq!(name, "_underscore"),
        _ => panic!("Expected identifier"),
    }
    
    match &tokens[2].value {
        TokenType::Identifier(name) => assert_eq!(name, "camelCase"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_integer_literals() {
    let (tokens, _) = tokenize("123 456_i 0 999_i");
    
    let expected = vec![
        TokenType::IntLiteral(123),
        TokenType::IntLiteral(456),
        TokenType::IntLiteral(0),
        TokenType::IntLiteral(999),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_double_literals() {
    let (tokens, _) = tokenize("123.45 0.5_d 3.14159 42.0_d");
    
    let expected = vec![
        TokenType::DoubleLiteral(123.45),
        TokenType::DoubleLiteral(0.5),
        TokenType::DoubleLiteral(3.14159),
        TokenType::DoubleLiteral(42.0),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_string_literals() {
    let (tokens, diagnostics) = tokenize(r#""hello" "world with spaces" "empty string: " "" ""#);
    
    assert_eq!(tokens.len(), 5); // 4 strings + EOF
    assert_eq!(diagnostics.len(), 1); // 1 unterminated string error
    
    match &tokens[0].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[1].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "world with spaces"),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[2].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "empty string: "),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[3].value {
        TokenType::StringLiteral(s) => assert_eq!(s, ""),
        _ => panic!("Expected empty string literal"),
    }
}

#[test]
fn test_string_escape_sequences() {
    let (tokens, _) = tokenize(r#""line1\nline2" "tab\there" "quote\"inside" "backslash\\""#);
    
    assert_eq!(tokens.len(), 5); // 4 strings + EOF
    
    match &tokens[0].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "line1\nline2"),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[1].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "tab\there"),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[2].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "quote\"inside"),
        _ => panic!("Expected string literal"),
    }
    
    match &tokens[3].value {
        TokenType::StringLiteral(s) => assert_eq!(s, "backslash\\"),
        _ => panic!("Expected string literal"),
    }
}

#[test]
fn test_char_literals() {
    let (tokens, _) = tokenize(r#"'a' 'Z' '1' '_' '\n' '\t' '\''"#);
    
    let expected_chars = vec!['a', 'Z', '1', '_', '\n', '\t', '\''];
    
    assert_eq!(tokens.len(), expected_chars.len() + 1); // chars + EOF
    
    for (i, expected_char) in expected_chars.iter().enumerate() {
        match &tokens[i].value {
            TokenType::CharLiteral(c) => assert_eq!(c, expected_char),
            _ => panic!("Expected char literal at position {}", i),
        }
    }
}

#[test]
fn test_comments() {
    let (tokens, _) = tokenize("before // this is a comment\nafter");
    
    assert_eq!(tokens.len(), 3); // "before", "after", EOF
    
    match &tokens[0].value {
        TokenType::Identifier(name) => assert_eq!(name, "before"),
        _ => panic!("Expected identifier"),
    }
    
    match &tokens[1].value {
        TokenType::Identifier(name) => assert_eq!(name, "after"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_slash_vs_comment() {
    let (tokens, _) = tokenize("5 / 2 // division and comment");
    
    assert_eq!(tokens.len(), 4); // 5, /, 2, EOF
    
    let expected = vec![
        TokenType::IntLiteral(5),
        TokenType::Slash,
        TokenType::IntLiteral(2),
        TokenType::EOF,
    ];

    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_whitespace_handling() {
    let (tokens, _) = tokenize("   a\t\tb   \n  c\r\nd   ");
    
    assert_eq!(tokens.len(), 5); // a, b, c, d, EOF
    
    let identifiers = ["a", "b", "c", "d"];
    for (i, expected_id) in identifiers.iter().enumerate() {
        match &tokens[i].value {
            TokenType::Identifier(name) => assert_eq!(name, expected_id),
            _ => panic!("Expected identifier at position {}", i),
        }
    }
}

#[test]
fn test_position_tracking() {
    let (tokens, _) = tokenize("a\nb");
    
    assert_eq!(tokens.len(), 3); // a, b, EOF
    
    // First token 'a' should be at position 1:1
    let pos_a = tokens[0].get_position();
    assert_eq!(format!("{}", pos_a), "1:1-1:1");
    
    // Second token 'b' should be at position 2:1
    let pos_b = tokens[1].get_position();
    assert_eq!(format!("{}", pos_b), "2:1-2:1");
}

#[test]
fn test_position_tracking_detailed() {
    let (tokens, _) = tokenize("hello world\n  test");
    
    assert_eq!(tokens.len(), 4); // hello, world, test, EOF
    
    // "hello" at 1:1-1:5
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:5");
    
    // "world" at 1:7-1:11 
    assert_eq!(format!("{}", tokens[1].get_position()), "1:7-1:11");
    
    // "test" at 2:3-2:6 (after 2 spaces)
    assert_eq!(format!("{}", tokens[2].get_position()), "2:3-2:6");
}

#[test]
fn test_decimal_literal_as_int_error() {
    let (tokens, diagnostics) = tokenize("3.14_i");
    
    assert_eq!(tokens.len(), 1); // Only EOF (error prevented token creation)
    assert_eq!(diagnostics.len(), 1);
    
    // Check error message
    assert_eq!(diagnostics[0].message, "decimal literal cannot be used as int");
    
    // Check position covers the entire literal "3.14_i"
    assert_eq!(format!("{}", diagnostics[0].position), "1:1-1:6");
}

#[test]
fn test_unterminated_string_error() {
    let (tokens, diagnostics) = tokenize(r#""unterminated string"#);
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(diagnostics.len(), 1);
    
    // Check error message
    assert_eq!(diagnostics[0].message, "unterminated string");
    
    // Check position starts at quote and goes to end
    assert_eq!(format!("{}", diagnostics[0].position), "1:1-1:20");
}

#[test]
fn test_unterminated_char_error() {
    let (tokens, diagnostics) = tokenize("'unterminated");
    
    assert_eq!(tokens.len(), 3); // char token + identifier token + EOF
    assert_eq!(diagnostics.len(), 1);
    
    // First token should be a char literal with 'u' at position 1:1-1:2
    match &tokens[0].value {
        TokenType::CharLiteral(c) => {
            assert_eq!(*c, 'u');
            assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:2");
        },
        _ => panic!("Expected char literal with 'u'"),
    }
    
    // Second token should be an identifier with "nterminated" at position 1:3-1:13
    match &tokens[1].value {
        TokenType::Identifier(name) => {
            assert_eq!(name, "nterminated");
            assert_eq!(format!("{}", tokens[1].get_position()), "1:3-1:13");
        },
        _ => panic!("Expected identifier 'nterminated'"),
    }
    
    // Check error message and position
    assert_eq!(diagnostics[0].message, "unterminated char literal");
    assert_eq!(format!("{}", diagnostics[0].position), "1:1-1:2");
}

#[test]
fn test_unknown_escape_sequence_error() {
    let (tokens, diagnostics) = tokenize(r#"'\x'"#);
    
    assert_eq!(tokens.len(), 1); // EOF
    assert_eq!(diagnostics.len(), 1);
    
    // Check error message
    assert_eq!(diagnostics[0].message, "unknown escape sequence: \\x");
    
    // Check position covers the escape sequence
    assert_eq!(format!("{}", diagnostics[0].position), "1:2-1:3");
}

#[test]
fn test_unknown_symbol_error() {
    let (tokens, diagnostics) = tokenize("@#`");
    
    assert_eq!(tokens.len(), 1); // Only EOF (unknown symbols prevented token creation)
    assert_eq!(diagnostics.len(), 3); // Three unknown symbols
    
    // Check each error message and position
    assert_eq!(diagnostics[0].message, "unknown symbol @");
    assert_eq!(format!("{}", diagnostics[0].position), "1:1-1:1");
    
    assert_eq!(diagnostics[1].message, "unknown symbol #");
    assert_eq!(format!("{}", diagnostics[1].position), "1:2-1:2");
    
    assert_eq!(diagnostics[2].message, "unknown symbol `");
    assert_eq!(format!("{}", diagnostics[2].position), "1:3-1:3");
}

#[test]
fn test_string_positions() {
    let (tokens, _) = tokenize(r#""hello" "world""#);
    
    assert_eq!(tokens.len(), 3); // 2 strings + EOF
    
    // First string "hello" at 1:1-1:7 (including quotes)
    match &tokens[0].value {
        TokenType::StringLiteral(s) => {
            assert_eq!(s, "hello");
            assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:7");
        },
        _ => panic!("Expected string literal"),
    }
    
    // Second string "world" at 1:9-1:15 (including quotes)
    match &tokens[1].value {
        TokenType::StringLiteral(s) => {
            assert_eq!(s, "world");
            assert_eq!(format!("{}", tokens[1].get_position()), "1:9-1:15");
        },
        _ => panic!("Expected string literal"),
    }
}

#[test]
fn test_numeric_literal_positions() {
    let (tokens, diagnostics) = tokenize("123 45.67_i 89.0_d");
    
    assert_eq!(tokens.len(), 3); // 2 valid numbers + EOF (45.67_i causes error)
    assert_eq!(diagnostics.len(), 1); // 1 error for decimal literal as int
    
    // Integer 123 at 1:1-1:3
    match &tokens[0].value {
        TokenType::IntLiteral(n) => {
            assert_eq!(*n, 123);
            assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:3");
        },
        _ => panic!("Expected int literal"),
    }
    
    // 45.67_i should cause an error, so token[1] should be the double 89.0_d
    match &tokens[1].value {
        TokenType::DoubleLiteral(n) => {
            assert_eq!(*n, 89.0);
            assert_eq!(format!("{}", tokens[1].get_position()), "1:13-1:18");
        },
        _ => panic!("Expected double literal"),
    }
    
    // Check the error for 45.67_i
    assert_eq!(diagnostics[0].message, "decimal literal cannot be used as int");
    assert_eq!(format!("{}", diagnostics[0].position), "1:5-1:11");
}

#[test]
fn test_operator_positions() {
    let (tokens, _) = tokenize("->!=>=");
    
    assert_eq!(tokens.len(), 4); // 3 operators + EOF
    
    // Arrow "->" at 1:1-1:2
    assert_eq!(tokens[0].value, TokenType::Arrow);
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:2");
    
    // NotEqual "!=" at 1:3-1:4  
    assert_eq!(tokens[1].value, TokenType::NotEqual);
    assert_eq!(format!("{}", tokens[1].get_position()), "1:3-1:4");
    
    // GreaterEqual ">=" at 1:5-1:6
    assert_eq!(tokens[2].value, TokenType::GreaterEqual);
    assert_eq!(format!("{}", tokens[2].get_position()), "1:5-1:6");
}

#[test]
fn test_multiline_positions() {
    let (tokens, _) = tokenize("fn\nmain\n(\n)\n{\n}");
    
    assert_eq!(tokens.len(), 7); // fn, main, (, ), {, }, EOF
    
    // fn at 1:1-1:2
    assert_eq!(tokens[0].value, TokenType::Fn);
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:2");
    
    // main at 2:1-2:4
    assert_eq!(tokens[1].value, TokenType::Main);
    assert_eq!(format!("{}", tokens[1].get_position()), "2:1-2:4");
    
    // ( at 3:1-3:1
    assert_eq!(tokens[2].value, TokenType::LeftParen);
    assert_eq!(format!("{}", tokens[2].get_position()), "3:1-3:1");
    
    // ) at 4:1-4:1
    assert_eq!(tokens[3].value, TokenType::RightParen);
    assert_eq!(format!("{}", tokens[3].get_position()), "4:1-4:1");
    
    // { at 5:1-5:1
    assert_eq!(tokens[4].value, TokenType::LeftCurly);
    assert_eq!(format!("{}", tokens[4].get_position()), "5:1-5:1");
    
    // } at 6:1-6:1
    assert_eq!(tokens[5].value, TokenType::RightCurly);
    assert_eq!(format!("{}", tokens[5].get_position()), "6:1-6:1");
}

#[test]
fn test_complex_expression() {
    let (tokens, _) = tokenize("let answer = calculate(x + y * 2.5) -> bool;");
    
    let expected = vec![
        TokenType::Let,
        TokenType::Identifier("answer".to_string()),
        TokenType::Assignment,
        TokenType::Identifier("calculate".to_string()),
        TokenType::LeftParen,
        TokenType::Identifier("x".to_string()),
        TokenType::Plus,
        TokenType::Identifier("y".to_string()),
        TokenType::Star,
        TokenType::DoubleLiteral(2.5),
        TokenType::RightParen,
        TokenType::Arrow,
        TokenType::Bool,
        TokenType::Semicolon,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
    
    // Check positions for key tokens
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:3"); // "let"
    assert_eq!(format!("{}", tokens[1].get_position()), "1:5-1:10"); // "result"
    assert_eq!(format!("{}", tokens[13].get_position()), "1:44-1:44"); // ";"
}

#[test]
fn test_function_definition() {
    let (tokens, _) = tokenize("fn main() -> int { return 42_i; }");
    
    let expected = vec![
        TokenType::Fn,
        TokenType::Main,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::Arrow,
        TokenType::Int,
        TokenType::LeftCurly,
        TokenType::Return,
        TokenType::IntLiteral(42),
        TokenType::Semicolon,
        TokenType::RightCurly,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
    
    // Check positions for function definition structure
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:2"); // "fn"
    assert_eq!(format!("{}", tokens[1].get_position()), "1:4-1:7"); // "main"
    assert_eq!(format!("{}", tokens[6].get_position()), "1:18-1:18"); // "{"
    assert_eq!(format!("{}", tokens[10].get_position()), "1:33-1:33"); // "}"
}

#[test]
fn test_empty_input() {
    let (tokens, _) = tokenize("");
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(tokens[0].value, TokenType::EOF);
    
    // EOF should be at position 1:1 for empty input
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:1");
}

#[test]
fn test_only_whitespace() {
    let (tokens, _) = tokenize("   \t\n\r   ");
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(tokens[0].value, TokenType::EOF);
    
    // EOF should be after the end position after consuming whitespace
    assert_eq!(format!("{}", tokens[0].get_position()), "2:5-2:5");
}

#[test]
fn test_numeric_edge_cases() {
    let (tokens, _) = tokenize("0 0.0 123.456 999999999_i 0.000001_d");
    
    let expected = vec![
        TokenType::IntLiteral(0),
        TokenType::DoubleLiteral(0.0),
        TokenType::DoubleLiteral(123.456),
        TokenType::IntLiteral(999999999),
        TokenType::DoubleLiteral(0.000001),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
    
    // Check positions for numeric literals
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:1"); // "0"
    assert_eq!(format!("{}", tokens[1].get_position()), "1:3-1:5"); // "0.0"
    assert_eq!(format!("{}", tokens[2].get_position()), "1:7-1:13"); // "123.456"
    assert_eq!(format!("{}", tokens[3].get_position()), "1:15-1:25"); // "999999999_i"
    assert_eq!(format!("{}", tokens[4].get_position()), "1:27-1:36"); // "0.000001_d"
}

#[test]
fn test_binary_integers() {
    let (tokens, _) = tokenize("0b0 0b1 0b101 0b1111 0b10_i");
    
    let expected = vec![
        TokenType::IntLiteral(0),      // 0b0 = 0
        TokenType::IntLiteral(1),      // 0b1 = 1
        TokenType::IntLiteral(5),      // 0b101 = 5
        TokenType::IntLiteral(15),     // 0b1111 = 15
        TokenType::IntLiteral(2),      // 0b10_i = 2
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_binary_decimals() {
    let (tokens, _) = tokenize("0b0.1 0b1.1 0b10.01 0b11.11_d");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.5),     // 0b0.1 = 0.5
        TokenType::DoubleLiteral(1.5),     // 0b1.1 = 1.5
        TokenType::DoubleLiteral(2.25),    // 0b10.01 = 2.25
        TokenType::DoubleLiteral(3.75),    // 0b11.11_d = 3.75
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_binary_edge_cases() {
    let (tokens, _) = tokenize("0b0 0b1.0 0b0.0_d");
    
    let expected = vec![
        TokenType::IntLiteral(0),          // 0b0 = 0
        TokenType::DoubleLiteral(1.0),     // 0b1.0 = 1.0
        TokenType::DoubleLiteral(0.0),     // 0b0.0_d = 0.0
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_binary_decimal_as_int_error() {
    let (tokens, diagnostics) = tokenize("0b10.11_i");
    
    assert_eq!(tokens.len(), 1); // Only EOF (error prevented token creation)
    assert_eq!(diagnostics.len(), 1);
    
    assert_eq!(diagnostics[0].message, "decimal literal cannot be used as int");
}

#[test]
fn test_octal_integers() {
    let (tokens, _) = tokenize("0o0 0o7 0o10 0o77 0o100_i");
    
    let expected = vec![
        TokenType::IntLiteral(0),      // 0o0 = 0
        TokenType::IntLiteral(7),      // 0o7 = 7
        TokenType::IntLiteral(8),      // 0o10 = 8
        TokenType::IntLiteral(63),     // 0o77 = 63
        TokenType::IntLiteral(64),     // 0o100_i = 64
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_octal_decimals() {
    let (tokens, _) = tokenize("0o0.1 0o1.24 0o7.654");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.125),      // 0o0.1 = 0.125 (1/8)
        TokenType::DoubleLiteral(1.3125),     // 0o1.24 = 1.3125 (1 + 2/8 + 4/64)
        TokenType::DoubleLiteral(7.8359375),  // 0o7.654 = 7.8359375 (7 + 6/8 + 5/64 + 4/512)
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_octal_edge_cases() {
    let (tokens, _) = tokenize("0o0 0o1.0 0o0.0_d");
    
    let expected = vec![
        TokenType::IntLiteral(0),          // 0o0 = 0
        TokenType::DoubleLiteral(1.0),     // 0o1.0 = 1.0
        TokenType::DoubleLiteral(0.0),     // 0o0.0_d = 0.0
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_octal_decimal_as_int_error() {
    let (tokens, diagnostics) = tokenize("0o7.7_i");
    
    assert_eq!(tokens.len(), 1); // Only EOF (error prevented token creation)
    assert_eq!(diagnostics.len(), 1);
    
    assert_eq!(diagnostics[0].message, "decimal literal cannot be used as int");
}

#[test]
fn test_hexadecimal_integers() {
    let (tokens, _) = tokenize("0x0 0xF 0x10 0xFF 0xABC_i");
    
    let expected = vec![
        TokenType::IntLiteral(0),       // 0x0 = 0
        TokenType::IntLiteral(15),      // 0xF = 15
        TokenType::IntLiteral(16),      // 0x10 = 16
        TokenType::IntLiteral(255),     // 0xFF = 255
        TokenType::IntLiteral(2748),    // 0xABC_i = 2748
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_hexadecimal_decimals() {
    let (tokens, _) = tokenize("0x0.8 0x1.8 0xF.F 0xA.4_d");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.5),      // 0x0.8 = 0.5 (8/16)
        TokenType::DoubleLiteral(1.5),      // 0x1.8 = 1.5 (1 + 8/16)
        TokenType::DoubleLiteral(15.9375),  // 0xF.F = 15.9375 (15 + 15/16)
        TokenType::DoubleLiteral(10.25),    // 0xA.4_d = 10.25 (10 + 4/16)
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_hexadecimal_edge_cases() {
    let (tokens, _) = tokenize("0x0 0x1.0 0x0.0_d");
    
    let expected = vec![
        TokenType::IntLiteral(0),          // 0x0 = 0
        TokenType::DoubleLiteral(1.0),     // 0x1.0 = 1.0
        TokenType::DoubleLiteral(0.0),     // 0x0.0_d = 0.0
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_hexadecimal_decimal_as_int_error() {
    let (tokens, diagnostics) = tokenize("0xF.F_i");
    
    assert_eq!(tokens.len(), 1); // Only EOF (error prevented token creation)
    assert_eq!(diagnostics.len(), 1);
    
    assert_eq!(diagnostics[0].message, "decimal literal cannot be used as int");
}

#[test]
fn test_hexadecimal_case_insensitive() {
    let (tokens, _) = tokenize("0xABCD 0xabcd 0xAbCd");
    
    let expected = vec![
        TokenType::IntLiteral(43981),   // 0xABCD = 43981
        TokenType::IntLiteral(43981),   // 0xabcd = 43981
        TokenType::IntLiteral(43981),   // 0xAbCd = 43981
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_mixed_number_systems() {
    let (tokens, _) = tokenize("0b101 0o5 5 0x5");
    
    let expected = vec![
        TokenType::IntLiteral(5),   // 0b101 = 5
        TokenType::IntLiteral(5),   // 0o5 = 5
        TokenType::IntLiteral(5),   // 5 = 5
        TokenType::IntLiteral(5),   // 0x5 = 5
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_number_systems_with_decimals() {
    let (tokens, _) = tokenize("0b10.1 0o2.4 2.5 0x2.8");
    
    let expected = vec![
        TokenType::DoubleLiteral(2.5),   // 0b10.1 = 2.5
        TokenType::DoubleLiteral(2.5),   // 0o2.4 = 2.5
        TokenType::DoubleLiteral(2.5),   // 2.5 = 2.5
        TokenType::DoubleLiteral(2.5),   // 0x2.8 = 2.5
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_number_system_positions() {
    let (tokens, _) = tokenize("0b101 0o77 0xFF");
    
    assert_eq!(tokens.len(), 4); // 3 numbers + EOF
    
    // 0b101 at 1:1-1:5
    assert_eq!(tokens[0].value, TokenType::IntLiteral(5));
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:5");
    
    // 0o77 at 1:7-1:10
    assert_eq!(tokens[1].value, TokenType::IntLiteral(63));
    assert_eq!(format!("{}", tokens[1].get_position()), "1:7-1:10");
    
    // 0xFF at 1:12-1:15
    assert_eq!(tokens[2].value, TokenType::IntLiteral(255));
    assert_eq!(format!("{}", tokens[2].get_position()), "1:12-1:15");
}

#[test]
fn test_zero_prefix_only() {
    let (tokens, _) = tokenize("0 0_i 0_d 0.0");
    
    let expected = vec![
        TokenType::IntLiteral(0),
        TokenType::IntLiteral(0),
        TokenType::DoubleLiteral(0.0),
        TokenType::DoubleLiteral(0.0),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_binary_fractional_precision() {
    let (tokens, _) = tokenize("0b0.101 0b1.001 0b11.111");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.625),   // 0b0.101 = 0.625 (0.5 + 0.125)
        TokenType::DoubleLiteral(1.125),   // 0b1.001 = 1.125 (1 + 0.125)
        TokenType::DoubleLiteral(3.875),   // 0b11.111 = 3.875 (3 + 0.875)
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_octal_fractional_precision() {
    let (tokens, _) = tokenize("0o0.1 0o1.24 0o7.654");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.125),      // 0o0.1 = 0.125 (1/8)
        TokenType::DoubleLiteral(1.3125),     // 0o1.24 = 1.3125 (1 + 2/8 + 4/64)
        TokenType::DoubleLiteral(7.8359375),  // 0o7.654 = 7.8359375 (7 + 6/8 + 5/64 + 4/512)
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_hexadecimal_fractional_precision() {
    let (tokens, _) = tokenize("0x0.1 0x1.4 0xF.ABC");
    
    let expected = vec![
        TokenType::DoubleLiteral(0.0625),     // 0x0.1 = 0.0625 (1/16)
        TokenType::DoubleLiteral(1.25),       // 0x1.4 = 1.25 (1 + 4/16)
        TokenType::DoubleLiteral(15.6708984375),  // 0xF.ABC = 15.6708984375 (15 + 10/16 + 11/256 + 12/4096)
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_bitwise_operators() {
    let (tokens, _) = tokenize("& | ^ ~ << >>");
    
    let expected = vec![
        TokenType::Ampersand,
        TokenType::Bar,
        TokenType::Carat,
        TokenType::Tilda,
        TokenType::LeftShift,
        TokenType::RightShift,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_arithmetic_operators() {
    let (tokens, _) = tokenize("+ - * / %");
    
    let expected = vec![
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Percentage,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_shift_operators_in_expression() {
    let (tokens, _) = tokenize("x << 2 >> 1");
    
    let expected = vec![
        TokenType::Identifier("x".to_string()),
        TokenType::LeftShift,
        TokenType::IntLiteral(2),
        TokenType::RightShift,
        TokenType::IntLiteral(1),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_modulo_in_expression() {
    let (tokens, _) = tokenize("answer = a % b;");
    
    let expected = vec![
        TokenType::Identifier("answer".to_string()),
        TokenType::Assignment,
        TokenType::Identifier("a".to_string()),
        TokenType::Percentage,
        TokenType::Identifier("b".to_string()),
        TokenType::Semicolon,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_xor_in_expression() {
    let (tokens, _) = tokenize("flags = a ^ b ^ c;");
    
    let expected = vec![
        TokenType::Identifier("flags".to_string()),
        TokenType::Assignment,
        TokenType::Identifier("a".to_string()),
        TokenType::Carat,
        TokenType::Identifier("b".to_string()),
        TokenType::Carat,
        TokenType::Identifier("c".to_string()),
        TokenType::Semicolon,
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_complex_bitwise_expression() {
    let (tokens, _) = tokenize("(a & b) | (c ^ d) << 2 >> 1");
    
    let expected = vec![
        TokenType::LeftParen,
        TokenType::Identifier("a".to_string()),
        TokenType::Ampersand,
        TokenType::Identifier("b".to_string()),
        TokenType::RightParen,
        TokenType::Bar,
        TokenType::LeftParen,
        TokenType::Identifier("c".to_string()),
        TokenType::Carat,
        TokenType::Identifier("d".to_string()),
        TokenType::RightParen,
        TokenType::LeftShift,
        TokenType::IntLiteral(2),
        TokenType::RightShift,
        TokenType::IntLiteral(1),
        TokenType::EOF,
    ];

    assert_eq!(tokens.len(), expected.len());
    for (token, expected_type) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.value, *expected_type);
    }
}

#[test]
fn test_shift_operator_positions() {
    let (tokens, _) = tokenize("<< >>");
    
    assert_eq!(tokens.len(), 3); // << >> EOF
    
    // << at 1:1-1:2
    assert_eq!(tokens[0].value, TokenType::LeftShift);
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:2");
    
    // >> at 1:4-1:5
    assert_eq!(tokens[1].value, TokenType::RightShift);
    assert_eq!(format!("{}", tokens[1].get_position()), "1:4-1:5");
}

#[test]
fn test_modulo_and_xor_positions() {
    let (tokens, _) = tokenize("% ^");
    
    assert_eq!(tokens.len(), 3); // % ^ EOF
    
    // % at 1:1-1:1
    assert_eq!(tokens[0].value, TokenType::Percentage);
    assert_eq!(format!("{}", tokens[0].get_position()), "1:1-1:1");
    
    // ^ at 1:3-1:3
    assert_eq!(tokens[1].value, TokenType::Carat);
    assert_eq!(format!("{}", tokens[1].get_position()), "1:3-1:3");
}