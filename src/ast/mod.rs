mod assignment_expr;
mod binary_expr;
mod block_expr;
mod cast;
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
mod struct_access;
mod array_access;
mod function_access;
mod new_array_expr;
mod parsed_type;
mod path;
mod scope;
mod struct_item;
mod unary_expr;
mod var_expr;

pub use assignment_expr::AssignmentExpr;
pub use binary_expr::{BinaryExpr, BinaryOperator};
pub use block_expr::BlockExpr;
pub use cast::CastExpr;
pub use constructor_call::ConstructorCallExpr;
pub use constructor_item::ConstructorItem;
pub use declaration_expr::DeclarationExpr;
pub use delete_expr::DeleteExpr;
pub use exit_expr::{ExitExpr, ExitType};
pub use function_item::FunctionItem;
pub use if_expr::IfExpr;
pub use impl_item::ImplItem;
pub use literal_expr::{Literal, LiteralExpr};
pub use loop_expr::LoopExpr;
pub use array_access::ArrayAccess;
pub use function_access::FunctionAccess;
pub use struct_access::StructAccess;
pub use new_array_expr::NewArrayExpr;
pub use parsed_type::{ParsedType, ParsedTypeEnum};
pub use path::Path;
pub use scope::{Require, Scope};
pub use struct_item::StructItem;
pub use unary_expr::{UnaryExpr, UnaryOperator};
pub use var_expr::VarExpr;


use serde::Serialize;
use std::sync::Mutex;

use crate::{compiler::stages::{Parsed, Phase}, lexer::token::Positioned};

#[derive(Serialize, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct AstId(usize);

impl ToString for AstId {
    fn to_string(&self) -> String {
        format!("ast_{}", self.0)
    }
}

static AST_ID_COUNTER: Mutex<AstId> = Mutex::new(AstId(0));

pub trait ASTNode<P: Phase>: Positioned + Send + Sync + Serialize {
    fn get_id(&self) -> AstId;
}

#[derive(Serialize)]
pub enum ASTEnum<P: Phase = Parsed> {
    ArrayAccess(Box<ArrayAccess<P>>),
    Assignment(Box<AssignmentExpr<P>>),
    Binary(Box<BinaryExpr<P>>),
    Block(Box<BlockExpr<P>>),
    Cast(Box<CastExpr<P>>),
    Constructor(Box<ConstructorItem<P>>),
    ConstructorCall(Box<ConstructorCallExpr<P>>),
    Declaration(Box<DeclarationExpr<P>>),
    Delete(Box<DeleteExpr<P>>),
    Exit(Box<ExitExpr<P>>),
    Function(Box<FunctionItem<P>>),
    FunctionAccess(Box<FunctionAccess<P>>),
    If(Box<IfExpr<P>>),
    Impl(Box<ImplItem<P>>),
    Literal(Box<LiteralExpr<P>>),
    Loop(Box<LoopExpr<P>>),
    NewArray(Box<NewArrayExpr<P>>),
    Scope(Box<Scope<P>>),
    Struct(Box<StructItem<P>>),
    StructAccess(Box<StructAccess<P>>),
    Unary(Box<UnaryExpr<P>>),
    Var(Box<VarExpr<P>>)
}

impl<P: Phase> From<Box<ArrayAccess<P>>> for ASTEnum<P> {
    fn from(node: Box<ArrayAccess<P>>) -> Self {
        ASTEnum::ArrayAccess(node)
    }
}

impl<P: Phase> From<Box<AssignmentExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<AssignmentExpr<P>>) -> Self {
        ASTEnum::Assignment(node)
    }
}

impl<P: Phase> From<Box<BinaryExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<BinaryExpr<P>>) -> Self {
        ASTEnum::Binary(node)
    }
}

impl<P: Phase> From<Box<BlockExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<BlockExpr<P>>) -> Self {
        ASTEnum::Block(node)
    }
}

impl<P: Phase> From<Box<CastExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<CastExpr<P>>) -> Self {
        ASTEnum::Cast(node)
    }
}

impl<P: Phase> From<Box<ConstructorItem<P>>> for ASTEnum<P> {
    fn from(node: Box<ConstructorItem<P>>) -> Self {
        ASTEnum::Constructor(node)
    }
}

impl<P: Phase> From<Box<ConstructorCallExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<ConstructorCallExpr<P>>) -> Self {
        ASTEnum::ConstructorCall(node)
    }
}

impl<P: Phase> From<Box<DeclarationExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<DeclarationExpr<P>>) -> Self {
        ASTEnum::Declaration(node)
    }
}

impl<P: Phase> From<Box<DeleteExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<DeleteExpr<P>>) -> Self {
        ASTEnum::Delete(node)
    }
}

impl<P: Phase> From<Box<ExitExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<ExitExpr<P>>) -> Self {
        ASTEnum::Exit(node)
    }
}

impl<P: Phase> From<Box<FunctionItem<P>>> for ASTEnum<P> {
    fn from(node: Box<FunctionItem<P>>) -> Self {
        ASTEnum::Function(node)
    }
}

impl<P: Phase> From<Box<FunctionAccess<P>>> for ASTEnum<P> {
    fn from(node: Box<FunctionAccess<P>>) -> Self {
        ASTEnum::FunctionAccess(node)
    }
}

impl<P: Phase> From<Box<IfExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<IfExpr<P>>) -> Self {
        ASTEnum::If(node)
    }
}

impl<P: Phase> From<Box<ImplItem<P>>> for ASTEnum<P> {
    fn from(node: Box<ImplItem<P>>) -> Self {
        ASTEnum::Impl(node)
    }
}

impl<P: Phase> From<Box<LiteralExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<LiteralExpr<P>>) -> Self {
        ASTEnum::Literal(node)
    }
}

impl<P: Phase> From<Box<LoopExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<LoopExpr<P>>) -> Self {
        ASTEnum::Loop(node)
    }
}

impl<P: Phase> From<Box<NewArrayExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<NewArrayExpr<P>>) -> Self {
        ASTEnum::NewArray(node)
    }
}

impl<P: Phase> From<Box<Scope<P>>> for ASTEnum<P> {
    fn from(node: Box<Scope<P>>) -> Self {
        ASTEnum::Scope(node)
    }
}

impl<P: Phase> From<Box<StructItem<P>>> for ASTEnum<P> {
    fn from(node: Box<StructItem<P>>) -> Self {
        ASTEnum::Struct(node)
    }
}

impl<P: Phase> From<Box<StructAccess<P>>> for ASTEnum<P> {
    fn from(node: Box<StructAccess<P>>) -> Self {
        ASTEnum::StructAccess(node)
    }
}

impl<P: Phase> From<Box<UnaryExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<UnaryExpr<P>>) -> Self {
        ASTEnum::Unary(node)
    }
}

impl<P: Phase> From<Box<VarExpr<P>>> for ASTEnum<P> {
    fn from(node: Box<VarExpr<P>>) -> Self {
        ASTEnum::Var(node)
    }
}

impl<P: Phase> Positioned for ASTEnum<P> {
    fn get_position(&self) -> &crate::lexer::token::PositionRange {
        match self {
            Self::ArrayAccess(node) => node.get_position(),
            Self::Assignment(node) => node.get_position(),
            Self::Binary(node) => node.get_position(),
            Self::Block(node) => node.get_position(),
            Self::Cast(node) => node.get_position(),
            Self::Constructor(node) => node.get_position(),
            Self::ConstructorCall(node) => node.get_position(),
            Self::Declaration(node) => node.get_position(),
            Self::Delete(node) => node.get_position(),
            Self::Exit(node) => node.get_position(),
            Self::Function(node) => node.get_position(),
            Self::FunctionAccess(node) => node.get_position(),
            Self::If(node) => node.get_position(),
            Self::Impl(node) => node.get_position(),
            Self::Literal(node) => node.get_position(),
            Self::Loop(node) => node.get_position(),
            Self::NewArray(node) => node.get_position(),
            Self::Scope(node) => node.get_position(),
            Self::Struct(node) => node.get_position(),
            Self::StructAccess(node) => node.get_position(),
            Self::Unary(node) => node.get_position(),
            Self::Var(node) => node.get_position(),
        }
    }
}

impl<P: Phase> ASTNode<P> for ASTEnum<P> {
    fn get_id(&self) -> AstId {
        match self {
            Self::ArrayAccess(node) => node.get_id(),
            Self::Assignment(node) => node.get_id(),
            Self::Binary(node) => node.get_id(),
            Self::Block(node) => node.get_id(),
            Self::Cast(node) => node.get_id(),
            Self::Constructor(node) => node.get_id(),
            Self::ConstructorCall(node) => node.get_id(),
            Self::Declaration(node) => node.get_id(),
            Self::Delete(node) => node.get_id(),
            Self::Exit(node) => node.get_id(),
            Self::Function(node) => node.get_id(),
            Self::FunctionAccess(node) => node.get_id(),
            Self::If(node) => node.get_id(),
            Self::Impl(node) => node.get_id(),
            Self::Literal(node) => node.get_id(),
            Self::Loop(node) => node.get_id(),
            Self::NewArray(node) => node.get_id(),
            Self::Scope(node) => node.get_id(),
            Self::Struct(node) => node.get_id(),
            Self::StructAccess(node) => node.get_id(),
            Self::Unary(node) => node.get_id(),
            Self::Var(node) => node.get_id(),
        }
    }
}

#[macro_export]
macro_rules! new_ast_id {
    () => {
        {
            let mut ast_id = crate::ast::AST_ID_COUNTER.lock().unwrap();
            ast_id.0 += 1;
            ast_id.clone()
        }
    };
}

#[macro_export]
macro_rules! impl_ast_node {
    ($Name: ident, $VisitFunction: ident) => {
        impl<P: crate::ast::Phase> crate::lexer::token::Positioned for $Name<P> {
            fn get_position(&self) -> &crate::lexer::token::PositionRange {
                &self.position
            }
        }

        impl<P: crate::ast::Phase> crate::ast::ASTNode<P> for $Name<P> {
            fn get_id(&self) -> crate::ast::AstId {
                self.id
            }
        }
    }
}

#[macro_export]
macro_rules! impl_var_resolved {
    ($Name: ident) => {
        impl $Name<crate:ast::VarResolved> {
            fn get_decl_id(&self) -> crate::ast::AstId {
                self.decl_id.unwrap()
            }
        }

        impl $Name<crate::ast::TypeResolved> {
            fn get_decl_id(&self) -> crate::resolver::TypeId {
                self.decl_id.unwrap()
            }
        }

        impl $Name<crate::ast::TypeChecked> {
            fn get_decl_id(&self) -> crate::resolver::TypeId {
                self.decl_id.unwrap()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_type_checked {
    // With type_id field
    ($Name: ident, $type_id_field: ident) => {
        impl $Name<crate::compiler::TypeChecked> {
            pub fn get_type_id(&self) -> crate::resolver::TypeId {
                self.$type_id_field.unwrap()
            }
        }
    };
}

