use std::fmt;

use crate::{expr::{Expr, binary_expr::BinaryExpr, declaration_expr::DeclarationExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::{assignment::AssignmentRule, boolean_factor::BooleanFactorRule, expr::ExprRule}}, token::{Position, PositionRange, TokenType}};

pub struct DeclarationRule {}

impl fmt::Display for DeclarationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Declaration")
    }
}

//declaration: LET [type] IDENTIFIER ASSIGNMENT [expression]? SEMICOLON | [assignment]
impl ParseRule<Box<dyn Expr>> for DeclarationRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {
        parser.log_debug(&format!("Entering declaration parser. Current token {:?}", parser.cur()));
        if let Some(let_token) = parser.try_consume(TokenType::Let) {
            let opt_type = parser.try_type();
            let declaration_type = parser.some_or_diagnostic(opt_type, diagnostic::err_expected_declaration_type(PositionRange::new(Position::new(0, 0))));
            parser.log_parse_result(&declaration_type, "declaration type");
    
            let declaration_name = parser.consume_or_diagnostic(TokenType::Identifier, diagnostic::err_expected_declaration_name(PositionRange::new(Position::new(0, 0))))
                .map(|x| x.get_string().to_string());
            parser.log_parse_result(&declaration_name, "declaration name");
    
            parser.consume_or_diagnostic(TokenType::Assignment, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Assignment));
    
            let expr = parser.apply_rule(ExprRule {});
            parser.log_parse_result(&expr, "declaration expression");
    
            parser.consume_or_diagnostic(TokenType::Semicolon, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Semicolon));
    
            let position = PositionRange::concat(&let_token.position, &parser.prev().position);
    
            parser.declaration_expr_id_counter += 1;
            Some(Box::new(DeclarationExpr::new(parser.declaration_expr_id_counter, declaration_name?, declaration_type?, expr?, position)))
        } else {
            Some(parser.apply_rule(AssignmentRule {})?)
        }
    }
    
}
