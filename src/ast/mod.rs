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
pub mod struct_item;
pub mod function_item;

use assignment_expr::AssignmentExpr;
use binary_expr::BinaryExpr;
use block_expr::BlockExpr;
use break_expr::BreakExpr;
use call_expr::CallExpr;
use declaration_expr::DeclarationExpr;
use erased_serde::serialize_trait_object;
use get_address_expr::GetAddressExpr;
use get_char_expr::GetCharExpr;
use if_expr::IfExpr;
use literal_expr::LiteralExpr;
use loop_expr::LoopExpr;
use put_char_expr::PutCharExpr;
use serde::Serialize;
use static_array_expr::StaticArrayExpr;
use struct_initializer_expr::StructInitializerExpr;
use unary_expr::UnaryExpr;
use var_expr::VarExpr;

use crate::{ast::{function_item::FunctionItem, struct_item::StructItem}, token::{PositionRange, Positioned}, types::resolved_type::ResolvedType};

pub trait ASTVisitable: AcceptsASTVisitor<Option<ResolvedType>> + AcceptsASTVisitor<()> {}

pub trait AcceptsASTVisitor<T> {
    fn accept_visitor(&self, visitor: &mut dyn ASTVisitor<T>) -> T;
}

pub trait ASTNode: ASTVisitable + Positioned + erased_serde::Serialize {}
serialize_trait_object!(ASTNode);

#[derive(Serialize)]
pub struct ASTWrapper<E> {
    pub data: E,
    pub position: PositionRange
}

impl<E> Positioned for ASTWrapper<E> {
    fn get_position(&self) -> &PositionRange {
        &self.position
    }
}

pub trait ASTVisitor<T> {
    fn visit_binary(&mut self, _node: &ASTWrapper<BinaryExpr>) -> T { unimplemented!() }
    fn visit_unary(&mut self, _node: &ASTWrapper<UnaryExpr>) -> T { unimplemented!() }
    fn visit_literal(&mut self, _node: &ASTWrapper<LiteralExpr>) -> T { unimplemented!() }
    fn visit_var(&mut self, _node: &ASTWrapper<VarExpr>) -> T { unimplemented!() }
    fn visit_if(&mut self, _node: &ASTWrapper<IfExpr>) -> T { unimplemented!() }
    fn visit_assignment(&mut self, _node: &ASTWrapper<AssignmentExpr>) -> T { unimplemented!() }
    fn visit_declaration(&mut self, _node: &ASTWrapper<DeclarationExpr>) -> T { unimplemented!() }
    fn visit_block(&mut self, _node: &ASTWrapper<BlockExpr>) -> T { unimplemented!() }
    fn visit_loop(&mut self, _node: &ASTWrapper<LoopExpr>) -> T { unimplemented!() }
    fn visit_break(&mut self, _node: &ASTWrapper<BreakExpr>) -> T { unimplemented!() }
    fn visit_call(&mut self, _node: &ASTWrapper<CallExpr>) -> T { unimplemented!() }
    fn visit_struct_initializer(&mut self, _node: &ASTWrapper<StructInitializerExpr>) -> T { unimplemented!() }
    fn visit_get_address(&mut self, _node: &ASTWrapper<GetAddressExpr>) -> T { unimplemented!() }
    fn visit_static_array(&mut self, _node: &ASTWrapper<StaticArrayExpr>) -> T { unimplemented!() }
    fn visit_put_char(&mut self, _node: &ASTWrapper<PutCharExpr>) -> T { unimplemented!() }
    fn visit_get_char(&mut self, _node: &ASTWrapper<GetCharExpr>) -> T { unimplemented!() }
    fn visit_struct(&mut self, _node: &ASTWrapper<StructItem>) -> T { unimplemented!() }
    fn visit_function(&mut self, _node: &ASTWrapper<FunctionItem>) -> T { unimplemented!() }
}

#[macro_export]
macro_rules! impl_ast_node {
    ($Name: ident, $VisitFunction: ident) => {
        impl crate::ast::ASTNode for crate::ast::ASTWrapper<$Name> {}
        impl crate::ast::ASTVisitable for crate::ast::ASTWrapper<$Name> {}

        impl<T> crate::ast::AcceptsASTVisitor<T> for crate::ast::ASTWrapper<$Name> {
            fn accept_visitor(&self, visitor: &mut dyn crate::ast::ASTVisitor<T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}