use std::fmt;

use crate::{expr::{ASTNode, ASTWrapper, assignment_expr::AssignmentExpr, var_expr::VarExpr}, logger::Log, parser::{ExprParser, ParseRule, diagnostic, rules::{expr::ExprRule, inline_expr::InlineExprRule, var::VarRule}}, token::{Position, PositionRange, TokenType}};

pub struct AssignmentRule {}

impl fmt::Display for AssignmentRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assignment")
    }
}

//Assignment: [var] Type[IntLiteral]
//Ex: alloc int[5]

//assignment: [var] ASSIGNMENT [expression] SEMICOLON

impl AssignmentRule {
    fn try_assignment(&self, parser: &mut ExprParser) -> Option<ASTWrapper<VarExpr>> {
        parser.push_ptr();

        parser.log_debug("Trying to parse assignment");
        let var_expr = parser.apply_rule(VarRule {});

        if let Some(var_expr) = &var_expr {
            parser.log_debug(&format!("Parsed var expr: {}", serde_json::to_string(var_expr).unwrap()));
        } else {
            parser.log_debug("Did not parse var expr");
        }

        if var_expr.is_none() || parser.try_consume(TokenType::Assignment).is_none() {
            parser.log_debug("No assignment found");
            parser.pop_ptr();

            return None;
        }

        parser.commit_ptr();
        parser.log_debug("Found assignment");

        var_expr
    }
}

impl ParseRule<Box<dyn ASTNode>> for AssignmentRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn ASTNode>> {
        match self.try_assignment(parser) {
            Some(asignee) => {
                let expr = parser.apply_rule(ExprRule {});

                parser.log_parse_result(&expr, "assignment expression");
            
                parser.consume_or_diagnostic(TokenType::Semicolon, diagnostic::err_expected_token(PositionRange::new(Position::new(0, 0)), TokenType::Semicolon));
            
                Some(Box::new(ASTWrapper::new_assignment(asignee, expr?)))
            },
            None => parser.apply_rule(InlineExprRule {})
        }
    }
}