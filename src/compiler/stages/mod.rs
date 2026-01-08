use std::{marker::PhantomData, ops::Deref};

use serde::Serialize;
use crate::ast::*;

pub mod local_resolved;


#[derive(Serialize)]
pub struct Parsed;
#[derive(Serialize)]
pub struct LocalResolved;
#[derive(Serialize)]
pub struct GlobalResolved;

pub trait Phase: Send + Sync + Serialize {}

pub trait NextPhase {
    type Next: Phase;
}

impl Phase for Parsed {}
impl NextPhase for Parsed {
    type Next = LocalResolved;
}

impl Phase for LocalResolved {}
impl NextPhase for LocalResolved {
    type Next = GlobalResolved;
}

impl Phase for GlobalResolved {}
impl NextPhase for GlobalResolved {
    type Next = GlobalResolved;
}

#[repr(C)]
#[derive(Serialize)]
pub struct VisitConfirmation<P: Phase> {
    ast_id: AstId,
    _marker: PhantomData<P>,
}

impl<P: Phase> VisitConfirmation<P> {
    fn new(ast_id: AstId) -> Self {
        Self {
            ast_id,
            _marker: PhantomData,
        }
    }

    pub fn with_data<T>(self, data: T) -> VisitResult<P, T> {
        VisitResult {
            ast_id: self.ast_id,
            data,
            _marker: PhantomData,
        }
    }

    pub fn verify(&self, node: &impl ASTNode<P>) -> bool {
        self.ast_id == node.get_id()
    }
}

impl<P: Phase> From<VisitConfirmation<P>> for VisitResult<P, ()> {
    fn from(confirmation: VisitConfirmation<P>) -> Self {
        confirmation.with_data(())
    }
}

#[repr(C)]
pub struct VisitResult<P: Phase, T = ()> {
    ast_id: AstId,
    _marker: std::marker::PhantomData<P>,
    data: T,
}

impl<P: Phase, T> VisitResult<P, T> {
    pub fn verify(visit_result: &VisitResult<P, T>, node: &impl ASTNode<P>) -> bool {
        visit_result.ast_id == node.get_id()
    }
}

impl<P: Phase, T> Deref for VisitResult<P, T> {
    type Target = VisitConfirmation<P>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // SAFETY: VisitConfirmation and the first two fields of VisitResult have the same layout
            &*(self as *const Self as *const VisitConfirmation<P>)
        }
    }
}

impl<P: Phase + NextPhase, T> VisitResult<P, T> {
    pub unsafe fn transmute_array_access(self, node: ArrayAccess<P>) -> ArrayAccess<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_assignment(self, node: AssignmentExpr<P>) -> AssignmentExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }
    
    pub unsafe fn transmute_binary(self, node: BinaryExpr<P>) -> BinaryExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_block(self, node: BlockExpr<P>) -> BlockExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_cast(self, node: CastExpr<P>) -> CastExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_constructor(self, node: ConstructorItem<P>) -> ConstructorItem<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_constructor_call(self, node: ConstructorCallExpr<P>) -> ConstructorCallExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_declaration(self, node: DeclarationExpr<P>) -> DeclarationExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_delete(self, node: DeleteExpr<P>) -> DeleteExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_exit(self, node: ExitExpr<P>) -> ExitExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_function(self, node: FunctionItem<P>) -> FunctionItem<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_function_access(self, node: FunctionAccess<P>) -> FunctionAccess<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_if(self, node: IfExpr<P>) -> IfExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_impl(self, node: ImplItem<P>) -> ImplItem<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_literal(self, node: LiteralExpr<P>) -> LiteralExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_loop(self, node: LoopExpr<P>) -> LoopExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_new_array(self, node: NewArrayExpr<P>) -> NewArrayExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_scope(self, node: Scope<P>) -> Scope<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_struct(self, node: StructItem<P>) -> StructItem<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_struct_access(self, node: StructAccess<P>) -> StructAccess<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_unary(self, node: UnaryExpr<P>) -> UnaryExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }

    pub unsafe fn transmute_var(self, node: VarExpr<P>) -> VarExpr<P::Next> {
        assert!(VisitResult::verify(&self, &node));
        unsafe { std::mem::transmute(node) }
    }
}

pub trait ASTVisitor<'ast, P, T = ()> 
where
    P: Phase
{
    fn visit_array_access(&mut self, node: &'ast ArrayAccess<P>) -> VisitResult<P, T>;
    fn visit_assignment(&mut self, node: &'ast AssignmentExpr<P>) -> VisitResult<P, T>;
    fn visit_binary(&mut self, node: &'ast BinaryExpr<P>) -> VisitResult<P, T>;
    fn visit_block(&mut self, node: &'ast BlockExpr<P>) -> VisitResult<P, T>;
    fn visit_cast(&mut self, node: &'ast CastExpr<P>) -> VisitResult<P, T>;
    fn visit_constructor(&mut self, node: &'ast ConstructorItem<P>) -> VisitResult<P, T>;
    fn visit_constructor_call(&mut self, node: &'ast ConstructorCallExpr<P>) -> VisitResult<P, T>;
    fn visit_declaration(&mut self, node: &'ast DeclarationExpr<P>) -> VisitResult<P, T>;
    fn visit_delete(&mut self, node: &'ast DeleteExpr<P>) -> VisitResult<P, T>;
    fn visit_exit(&mut self, node: &'ast ExitExpr<P>) -> VisitResult<P, T>;
    fn visit_function(&mut self, node: &'ast FunctionItem<P>) -> VisitResult<P, T>;
    fn visit_function_access(&mut self, node: &'ast FunctionAccess<P>) -> VisitResult<P, T>;
    fn visit_if(&mut self, node: &'ast IfExpr<P>) -> VisitResult<P, T>;
    fn visit_impl(&mut self, node: &'ast ImplItem<P>) -> VisitResult<P, T>;
    fn visit_literal(&mut self, node: &'ast LiteralExpr<P>) -> VisitResult<P, T>;
    fn visit_loop(&mut self, node: &'ast LoopExpr<P>) -> VisitResult<P, T>;
    fn visit_new_array(&mut self, node: &'ast NewArrayExpr<P>) -> VisitResult<P, T>;
    fn visit_scope(&mut self, node: &'ast Scope<P>) -> VisitResult<P, T>;
    fn visit_struct(&mut self, node: &'ast StructItem<P>) -> VisitResult<P, T>;
    fn visit_struct_access(&mut self, node: &'ast StructAccess<P>) -> VisitResult<P, T>;
    fn visit_unary(&mut self, node: &'ast UnaryExpr<P>) -> VisitResult<P, T>;
    fn visit_var(&mut self, node: &'ast VarExpr<P>) -> VisitResult<P, T>;
}

pub trait AcceptsASTVisitor<'ast, P: Phase, T = ()> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T>;
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ASTEnum<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        match self {
            ASTEnum::ArrayAccess(node) => visitor.visit_array_access(node),
            ASTEnum::Assignment(node) => visitor.visit_assignment(node),
            ASTEnum::Binary(node) => visitor.visit_binary(node),
            ASTEnum::Block(node) => visitor.visit_block(node),
            ASTEnum::Cast(node) => visitor.visit_cast(node),
            ASTEnum::Constructor(node) => visitor.visit_constructor(node),
            ASTEnum::ConstructorCall(node) => visitor.visit_constructor_call(node),
            ASTEnum::Declaration(node) => visitor.visit_declaration(node),
            ASTEnum::Delete(node) => visitor.visit_delete(node),
            ASTEnum::Exit(node) => visitor.visit_exit(node),
            ASTEnum::Function(node) => visitor.visit_function(node),
            ASTEnum::FunctionAccess(node) => visitor.visit_function_access(node),
            ASTEnum::If(node) => visitor.visit_if(node),
            ASTEnum::Impl(node) => visitor.visit_impl(node),
            ASTEnum::Literal(node) => visitor.visit_literal(node),
            ASTEnum::Loop(node) => visitor.visit_loop(node),
            ASTEnum::NewArray(node) => visitor.visit_new_array(node),
            ASTEnum::Scope(node) => visitor.visit_scope(node),
            ASTEnum::Struct(node) => visitor.visit_struct(node),
            ASTEnum::StructAccess(node) => visitor.visit_struct_access(node),
            ASTEnum::Unary(node) => visitor.visit_unary(node),
            ASTEnum::Var(node) => visitor.visit_var(node),
        }
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ArrayAccess<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_array_access(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for AssignmentExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_assignment(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for BinaryExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_binary(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for BlockExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_block(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for CastExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_cast(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ConstructorItem<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_constructor(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ConstructorCallExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_constructor_call(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for DeclarationExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_declaration(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for DeleteExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_delete(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ExitExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_exit(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for FunctionItem<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_function(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for FunctionAccess<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_function_access(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for IfExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_if(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for ImplItem<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_impl(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for LiteralExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_literal(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for LoopExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_loop(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for NewArrayExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_new_array(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for Scope<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_scope(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for StructItem<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_struct(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for StructAccess<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_struct_access(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for UnaryExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_unary(self)
    }
}

impl<'ast, P: Phase, T> AcceptsASTVisitor<'ast, P, T> for VarExpr<P> {
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, T>) -> VisitResult<P, T> {
        visitor.visit_var(self)
    }
}