use std::fmt;

use crate::{expr::{ASTWrapper, get_char_expr::GetCharExpr, if_expr::IfExpr, put_char_expr::PutCharExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::{block::BlockRule, inline_expr::InlineExprRule}}, token::{Position, PositionRange, TokenType}};

pub struct IfBlockRule {}

impl fmt::Display for IfBlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfBlock")
    }
}

//if: IF [inline_expression] [block] [elif]* [else]?

impl ParseRule<ASTWrapper<IfExpr>> for IfBlockRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<ASTWrapper<IfExpr>> {
        parser.log_debug(&format!("Entering if parser. Current token {:?}", parser.cur()));
        let if_token = parser.advance();  
        
        let condition = parser.apply_rule(InlineExprRule {});
        parser.log_parse_result(&condition, "if condition");
    
        let success = parser.apply_rule_boxed(BlockRule {});
        parser.log_parse_result(&success, "if success");
    
        let fail = if parser.try_consume(TokenType::Else).is_some(){
            let fail = match parser.cur().token_type {
                TokenType::If => parser.apply_rule_boxed(IfBlockRule {}),
                _ => parser.apply_rule_boxed(BlockRule {})
            };
    
            parser.log_parse_result(&fail, "if fail");
            fail
        } else {
            parser.log_debug(&format!("No else block found"));
            None
        };
    
        let position = PositionRange::concat(&if_token.position, &parser.prev().position);
    
        Some(ASTWrapper::new_if(condition?, success?, fail, position))
    }
}
