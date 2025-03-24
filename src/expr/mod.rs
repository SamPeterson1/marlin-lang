pub mod assignment_expr;
pub mod binary_expr;
pub mod block_expr;
pub mod break_expr;
pub mod call_expr;
pub mod declaration_expr;
pub mod get_address_expr;
pub mod get_char_expr;
pub mod if_expr;
pub mod literal_expr;
pub mod loop_expr;
pub mod put_char_expr;
pub mod static_array_expr;
pub mod struct_initializer_expr;
pub mod unary_expr;
pub mod var_expr;

use std::fmt;

use assignment_expr::AssignmentExpr;
use binary_expr::BinaryExpr;
use block_expr::BlockExpr;
use break_expr::BreakExpr;
use call_expr::CallExpr;
use declaration_expr::DeclarationExpr;
use get_address_expr::GetAddressExpr;
use get_char_expr::GetCharExpr;
use if_expr::IfExpr;
use literal_expr::LiteralExpr;
use loop_expr::LoopExpr;
use put_char_expr::PutCharExpr;
use static_array_expr::StaticArrayExpr;
use struct_initializer_expr::StructInitializerExpr;
use unary_expr::UnaryExpr;
use var_expr::VarExpr;

use crate::{token::PositionRange, types::resolved_type::ResolvedType};

pub trait Expr: ExprVisitable<Option<ResolvedType>> + ExprVisitable<()> + fmt::Display {
    fn get_position(&self) -> &PositionRange;
}

pub trait ExprVisitable<T> {
    fn accept_visitor(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> T;
    fn visit_unary(&mut self, expr: &UnaryExpr) -> T;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> T;
    fn visit_var(&mut self, expr: &VarExpr) -> T;
    fn visit_if(&mut self, expr: &IfExpr) -> T;
    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> T;
    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> T;
    fn visit_block(&mut self, expr: &BlockExpr) -> T;
    fn visit_loop(&mut self, expr: &LoopExpr) -> T;
    fn visit_break(&mut self, expr: &BreakExpr) -> T;
    fn visit_call(&mut self, expr: &CallExpr) -> T;
    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> T;
    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> T;
    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> T;
    fn visit_put_char(&mut self, expr: &PutCharExpr) -> T;
    fn visit_get_char(&mut self, expr: &GetCharExpr) -> T;
}

#[macro_export]
macro_rules! impl_expr {
    ($Name: ident, $VisitFunction: ident) => {
        use crate::expr::{ExprVisitor, ExprVisitable};

        impl Expr for $Name {
            fn get_position(&self) -> &PositionRange {
                &self.position
            }
        }

        impl<T> ExprVisitable<T> for $Name {
            fn accept_visitor(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}