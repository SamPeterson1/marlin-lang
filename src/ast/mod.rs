pub mod arguments;
pub mod assignment_expr;
pub mod binary_expr;
pub mod block_expr;
pub mod constructor_call;
pub mod constructor_item;
pub mod declaration_expr;
pub mod delete_expr;
pub mod exit_expr;
pub mod function_item;
pub mod if_expr;
pub mod impl_item;
pub mod literal_expr;
pub mod loop_expr;
pub mod main_item;
pub mod member_access;
pub mod new_array_expr;
pub mod parameters;
pub mod parsed_type;
pub mod program;
pub mod struct_item;
pub mod unary_expr;
pub mod var_expr;

use assignment_expr::AssignmentExpr;
use binary_expr::BinaryExpr;
use block_expr::BlockExpr;
use declaration_expr::DeclarationExpr;
use erased_serde::serialize_trait_object;
use if_expr::IfExpr;
use literal_expr::LiteralExpr;
use loop_expr::LoopExpr;
use unary_expr::UnaryExpr;

use crate::ast::delete_expr::DeleteExpr;
use crate::ast::{constructor_call::ConstructorCallExpr, constructor_item::ConstructorItem, exit_expr::ExitExpr, function_item::FunctionItem, impl_item::ImplItem, main_item::MainItem, member_access::MemberAccess, new_array_expr::NewArrayExpr, struct_item::StructItem, var_expr::VarExpr};
use crate::lexer::token::Positioned;

pub trait ASTVisitable: AcceptsASTVisitor<()> {}

pub trait AcceptsASTVisitor<T> {
    fn accept_visitor(&self, visitor: &mut dyn ASTVisitor<T>) -> T;
}

pub trait ASTNode: ASTVisitable + Positioned + erased_serde::Serialize {}
serialize_trait_object!(ASTNode);

pub trait ASTVisitor<T> {
    fn visit_binary(&mut self, _node: &BinaryExpr) -> T { unimplemented!() }
    fn visit_unary(&mut self, _node: &UnaryExpr) -> T { unimplemented!() }
    fn visit_literal(&mut self, _node: &LiteralExpr) -> T { unimplemented!() }
    fn visit_member_access(&mut self, _node: &MemberAccess) -> T { unimplemented!() }
    fn visit_var(&mut self, _node: &VarExpr) -> T { unimplemented!() }
    fn visit_if(&mut self, _node: &IfExpr) -> T { unimplemented!() }
    fn visit_assignment(&mut self, _node: &AssignmentExpr) -> T { unimplemented!() }
    fn visit_delete(&mut self, _node: &DeleteExpr) -> T { unimplemented!() }
    fn visit_declaration(&mut self, _node: &DeclarationExpr) -> T { unimplemented!() }
    fn visit_block(&mut self, _node: &BlockExpr) -> T { unimplemented!() }
    fn visit_loop(&mut self, _node: &LoopExpr) -> T { unimplemented!() }
    fn visit_exit(&mut self, _node: &ExitExpr) -> T { unimplemented!() }
    fn visit_constructor_call(&mut self, _node: &ConstructorCallExpr) -> T { unimplemented!() }
    fn visit_new_array(&mut self, _node: &NewArrayExpr) -> T { unimplemented!() }
    fn visit_impl(&mut self, _node: &ImplItem) -> T { unimplemented!() }
    fn visit_function(&mut self, _node: &FunctionItem) -> T { unimplemented!() }
    fn visit_struct(&mut self, _node: &StructItem) -> T { unimplemented!() }
    fn visit_constructor(&mut self, _node: &ConstructorItem) -> T { unimplemented!() }
    fn visit_main(&mut self, _node: &MainItem) -> T { unimplemented!() }
}

#[macro_export]
macro_rules! impl_ast_node {
    ($Name: ident, $VisitFunction: ident) => {
        impl crate::ast::ASTNode for $Name {}
        impl crate::ast::ASTVisitable for $Name {}
        
        impl<T> crate::ast::AcceptsASTVisitor<T> for $Name {
            fn accept_visitor(&self, visitor: &mut dyn crate::ast::ASTVisitor<T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}