use std::{marker::PhantomData, ops::Deref};

use serde::Serialize;
use crate::ast::*;

#[derive(Serialize)]
pub struct Parsed;
#[derive(Serialize)]
pub struct LocalResolved;
#[derive(Serialize)]
pub struct GlobalResolved;

pub trait PhaseWitness<P: Phase> {}

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
pub struct VisitResult<P, W>
where
    P: Phase,
    W: PhaseWitness<P>,
{
    ast_id: AstId,
    witness: W,
    _marker: PhantomData<P>,
}

impl<P, W> VisitResult<P, W> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    pub fn new(ast_id: AstId, witness: W) -> Self {
        Self {
            ast_id,
            witness,
            _marker: PhantomData,
        }
    }

    pub fn verify(&self, node: &impl ASTNode<P>) -> bool {
        self.ast_id == node.get_id()
    }
}

impl<P, W> VisitResult<P, W> 
where
    P: Phase + NextPhase,
    W: PhaseWitness<P>,
{
    pub fn transmute_array_access(self, node: &ArrayAccess<P>) -> &ArrayAccess<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_assignment(self, node: &AssignmentExpr<P>) -> &AssignmentExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }
    
    pub fn transmute_binary(self, node: &BinaryExpr<P>) -> &BinaryExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_block(self, node: &BlockExpr<P>) -> &BlockExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_cast(self, node: &CastExpr<P>) -> &CastExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_constructor(self, node: &ConstructorItem<P>) -> &ConstructorItem<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_constructor_call(self, node: &ConstructorCallExpr<P>) -> &ConstructorCallExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_declaration(self, node: &DeclarationExpr<P>) -> &DeclarationExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_delete(self, node: &DeleteExpr<P>) -> &DeleteExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_exit(self, node: &ExitExpr<P>) -> &ExitExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_function(self, node: &FunctionItem<P>) -> &FunctionItem<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_function_access(self, node: &FunctionAccess<P>) -> &FunctionAccess<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_if(self, node: &IfExpr<P>) -> &IfExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_impl(self, node: &ImplItem<P>) -> &ImplItem<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_literal(self, node: &LiteralExpr<P>) -> &LiteralExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_loop(self, node: &LoopExpr<P>) -> &LoopExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_new_array(self, node: &NewArrayExpr<P>) -> &NewArrayExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_scope(self, node: &Scope<P>) -> &Scope<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_struct(self, node: &StructItem<P>) -> &StructItem<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_struct_access(self, node: &StructAccess<P>) -> &StructAccess<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_unary(self, node: &UnaryExpr<P>) -> &UnaryExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }

    pub fn transmute_var(self, node: &VarExpr<P>) -> &VarExpr<P::Next> {
        assert!(VisitResult::verify(&self, node));
        unsafe { std::mem::transmute(node) }
    }
}

pub trait ASTVisitor<'ast, P, W> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn visit_array_access(&mut self, node: &'ast ArrayAccess<P>) -> VisitResult<P, W>;
    fn visit_assignment(&mut self, node: &'ast AssignmentExpr<P>) -> VisitResult<P, W>;
    fn visit_binary(&mut self, node: &'ast BinaryExpr<P>) -> VisitResult<P, W>;
    fn visit_block(&mut self, node: &'ast BlockExpr<P>) -> VisitResult<P, W>;
    fn visit_cast(&mut self, node: &'ast CastExpr<P>) -> VisitResult<P, W>;
    fn visit_constructor(&mut self, node: &'ast ConstructorItem<P>) -> VisitResult<P, W>;
    fn visit_constructor_call(&mut self, node: &'ast ConstructorCallExpr<P>) -> VisitResult<P, W>;
    fn visit_declaration(&mut self, node: &'ast DeclarationExpr<P>) -> VisitResult<P, W>;
    fn visit_delete(&mut self, node: &'ast DeleteExpr<P>) -> VisitResult<P, W>;
    fn visit_exit(&mut self, node: &'ast ExitExpr<P>) -> VisitResult<P, W>;
    fn visit_function(&mut self, node: &'ast FunctionItem<P>) -> VisitResult<P, W>;
    fn visit_function_access(&mut self, node: &'ast FunctionAccess<P>) -> VisitResult<P, W>;
    fn visit_if(&mut self, node: &'ast IfExpr<P>) -> VisitResult<P, W>;
    fn visit_impl(&mut self, node: &'ast ImplItem<P>) -> VisitResult<P, W>;
    fn visit_literal(&mut self, node: &'ast LiteralExpr<P>) -> VisitResult<P, W>;
    fn visit_loop(&mut self, node: &'ast LoopExpr<P>) -> VisitResult<P, W>;
    fn visit_new_array(&mut self, node: &'ast NewArrayExpr<P>) -> VisitResult<P, W>;
    fn visit_scope(&mut self, node: &'ast Scope<P>) -> VisitResult<P, W>;
    fn visit_struct(&mut self, node: &'ast StructItem<P>) -> VisitResult<P, W>;
    fn visit_struct_access(&mut self, node: &'ast StructAccess<P>) -> VisitResult<P, W>;
    fn visit_unary(&mut self, node: &'ast UnaryExpr<P>) -> VisitResult<P, W>;
    fn visit_var(&mut self, node: &'ast VarExpr<P>) -> VisitResult<P, W>;
}

pub trait AcceptsASTVisitor<'ast, P, W> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W>;
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ASTEnum<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
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


impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ArrayAccess<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_array_access(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for AssignmentExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_assignment(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for BinaryExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_binary(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for BlockExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_block(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for CastExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_cast(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ConstructorItem<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_constructor(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ConstructorCallExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_constructor_call(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for DeclarationExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_declaration(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for DeleteExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_delete(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ExitExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_exit(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for FunctionItem<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_function(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for FunctionAccess<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_function_access(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for IfExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_if(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for ImplItem<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_impl(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for LiteralExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_literal(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for LoopExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_loop(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for NewArrayExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_new_array(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for Scope<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_scope(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for StructItem<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_struct(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for StructAccess<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_struct_access(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for UnaryExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_unary(self)
    }
}

impl<'ast, P, W> AcceptsASTVisitor<'ast, P, W> for VarExpr<P> 
where
    P: Phase,
    W: PhaseWitness<P>,
{
    fn accept_visitor(&'ast self, visitor: &mut impl ASTVisitor<'ast, P, W>) -> VisitResult<P, W> {
        visitor.visit_var(self)
    }
}