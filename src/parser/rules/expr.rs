use crate::{expr::{Expr, put_char_expr::PutCharExpr}, logger::Log, parser::{ExprParser, ParseRule, rules::{array_allocation::ArrayAllocationRule, block::BlockRule, break_expr::BreakRule, declaration::DeclarationRule, for_loop::ForLoopRule, getc::GetcRule, if_block::IfBlockRule, loop_expr::LoopRule, putc::PutcRule, struct_initializer::StructInitializerRule, while_loop::{WhileLoopRule}}}, token::TokenType};
use std::fmt;

pub struct ExprRule {}

impl fmt::Display for ExprRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expr")
    }
}

impl ParseRule<Box<dyn Expr>> for ExprRule {
    fn parse(&self, parser: &mut ExprParser) -> Option<Box<dyn Expr>> {
        parser.log_debug(&format!("Entering expression parser. Current token {:?}", parser.cur()));
    
        match parser.cur().token_type {
            TokenType::If => parser.apply_rule_boxed(IfBlockRule {}),
            TokenType::For => parser.apply_rule_boxed(ForLoopRule {}),
            TokenType::While => parser.apply_rule_boxed(WhileLoopRule {}),
            TokenType::Loop => parser.apply_rule_boxed(LoopRule {}),
            TokenType::Break => parser.apply_rule_boxed(BreakRule {}),
            TokenType::Alloc => parser.apply_rule_boxed(ArrayAllocationRule {}),
            TokenType::Putc => parser.apply_rule_boxed(PutcRule {}),
            TokenType::Getc => parser.apply_rule_boxed(GetcRule {}),
            TokenType::LeftCurly => parser.apply_rule_boxed(BlockRule {}),
            TokenType::Identifier => {
                if parser.peek().token_type == TokenType::LeftCurly {
                    parser.apply_rule_boxed(StructInitializerRule {})
                } else {
                    parser.apply_rule(DeclarationRule {})
                }
            },
            _ => parser.apply_rule(DeclarationRule {})
        }  
    }
}