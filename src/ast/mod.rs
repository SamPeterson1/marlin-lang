mod arguments;
mod assignment_expr;
mod binary_expr;
mod block_expr;
mod constructor_call;
mod constructor_item;
mod declaration_expr;
mod delete_expr;
mod exit_expr;
mod function_item;
mod if_expr;
mod impl_item;
mod literal_expr;
mod loop_expr;
mod main_item;
mod member_access;
mod new_array_expr;
mod parameters;
mod parsed_type;
mod program;
mod struct_item;
mod unary_expr;
mod var_expr;

pub use arguments::Arguments;
pub use assignment_expr::AssignmentExpr;
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use block_expr::BlockExpr;
pub use constructor_call::ConstructorCallExpr;
pub use constructor_item::ConstructorItem;
pub use declaration_expr::{DeclarationExpr, DeclarationId};
pub use delete_expr::DeleteExpr;
pub use exit_expr::{ExitExpr, ExitType};
pub use function_item::FunctionItem;
pub use if_expr::IfExpr;
pub use impl_item::ImplItem;
pub use literal_expr::{Literal, LiteralExpr};
pub use loop_expr::LoopExpr;
pub use main_item::MainItem;
pub use member_access::{AccessType, MemberAccess};
pub use new_array_expr::NewArrayExpr;
pub use parameters::Parameters;
pub use parsed_type::{ArrayModifier, ParsedBaseType, ParsedUnitType, ParsedType};
pub use program::Program;
pub use struct_item::StructItem;
pub use unary_expr::{UnaryExpr, UnaryOperator};
pub use var_expr::{VarExpr, VarId};

use crate::lexer::token::Positioned;

use erased_serde::serialize_trait_object;
use std::any::Any;

pub trait ASTVisitable: AcceptsASTVisitor<()> {}

pub trait AcceptsASTVisitor<T> {
    fn accept_visitor<'ast>(&'ast self, visitor: &mut dyn ASTVisitor<'ast, T>) -> T;
}

pub trait ASTNode: ASTVisitable + Positioned + erased_serde::Serialize {
    fn as_any(&self) -> &dyn Any;
}

serialize_trait_object!(ASTNode);

pub trait ASTVisitor<'ast, T> {
    fn visit_binary(&mut self, _node: &'ast BinaryExpr) -> T { unimplemented!() }
    fn visit_unary(&mut self, _node: &'ast UnaryExpr) -> T { unimplemented!() }
    fn visit_literal(&mut self, _node: &'ast LiteralExpr) -> T { unimplemented!() }
    fn visit_member_access(&mut self, _node: &'ast MemberAccess) -> T { unimplemented!() }
    fn visit_var(&mut self, _node: &'ast VarExpr) -> T { unimplemented!() }
    fn visit_if(&mut self, _node: &'ast IfExpr) -> T { unimplemented!() }
    fn visit_assignment(&mut self, _node: &'ast AssignmentExpr) -> T { unimplemented!() }
    fn visit_delete(&mut self, _node: &'ast DeleteExpr) -> T { unimplemented!() }
    fn visit_declaration(&mut self, _node: &'ast DeclarationExpr) -> T { unimplemented!() }
    fn visit_block(&mut self, _node: &'ast BlockExpr) -> T { unimplemented!() }
    fn visit_loop(&mut self, _node: &'ast LoopExpr) -> T { unimplemented!() }
    fn visit_exit(&mut self, _node: &'ast ExitExpr) -> T { unimplemented!() }
    fn visit_constructor_call(&mut self, _node: &'ast ConstructorCallExpr) -> T { unimplemented!() }
    fn visit_new_array(&mut self, _node: &'ast NewArrayExpr) -> T { unimplemented!() }
    fn visit_impl(&mut self, _node: &'ast ImplItem) -> T { unimplemented!() }
    fn visit_function(&mut self, _node: &'ast FunctionItem) -> T { unimplemented!() }
    fn visit_struct(&mut self, _node: &'ast StructItem) -> T { unimplemented!() }
    fn visit_constructor(&mut self, _node: &'ast ConstructorItem) -> T { unimplemented!() }
    fn visit_main(&mut self, _node: &'ast MainItem) -> T { unimplemented!() }
    fn visit_program(&mut self, _node: &'ast Program) -> T { unimplemented!() }
}

#[macro_export]
macro_rules! impl_ast_node {
    ($Name: ident, $VisitFunction: ident) => {
        impl crate::ast::ASTNode for $Name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl crate::ast::ASTVisitable for $Name {}
        
        impl<T> crate::ast::AcceptsASTVisitor<T> for $Name {
            fn accept_visitor<'ast>(&'ast self, visitor: &mut dyn crate::ast::ASTVisitor<'ast, T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}