use std::{collections::HashMap, hash::Hasher, rc::Rc};

use crate::{environment::{Literal, ParsedType, ResolvedType}, operator::{self, BinaryOperator, UnaryOperator}, token::{Position, PositionRange, Token}};

pub trait ExprVisitable<T> {
    fn accept_visitor(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}

pub trait Expr: ExprVisitable<ResolvedType> + ExprVisitable<()> + std::fmt::Debug {
    fn get_position(&self) -> &PositionRange;
}

pub trait ExprVisitor<T> {
    fn visit_empty(&mut self, expr: &EmptyExpr) -> T;
    fn visit_binary(&mut self, expr: &BinaryExpr) -> T;
    fn visit_unary(&mut self, expr: &UnaryExpr) -> T;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> T;
    fn visit_var(&mut self, expr: &VarExpr) -> T;
    fn visit_if(&mut self, expr: &IfExpr) -> T;
    fn visit_assignment(&mut self, expr: &AssignmentExpr) -> T;
    fn visit_declaration(&mut self, expr: &DeclarationExpr) -> T;
    fn visit_block(&mut self, expr: &BlockExpr) -> T;
    fn visit_print(&mut self, expr: &PrintExpr) -> T;
    fn visit_rand(&mut self, expr: &RandExpr) -> T;
    fn visit_loop(&mut self, expr: &LoopExpr) -> T;
    fn visit_break(&mut self, expr: &BreakExpr) -> T;
    fn visit_input(&mut self, expr: &InputExpr) -> T;
    fn visit_call(&mut self, expr: &CallExpr) -> T;
    fn visit_struct_initializer(&mut self, expr: &StructInitializerExpr) -> T;
    fn visit_get_address(&mut self, expr: &GetAddressExpr) -> T;
    fn visit_static_array(&mut self, expr: &StaticArrayExpr) -> T;
    fn visit_put_char(&mut self, expr: &PutCharExpr) -> T;
    fn visit_get_char(&mut self, expr: &GetCharExpr) -> T;
}

macro_rules! impl_expr {
    ($Name: ident, $VisitFunction: ident) => {
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

#[derive(Debug)]
pub struct EmptyExpr {
    pub position: PositionRange
}

impl EmptyExpr {
    pub fn new(position: PositionRange) -> Box<dyn Expr> {
        Box::new(EmptyExpr {position})
    }
}

impl_expr!(EmptyExpr, visit_empty);

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: Box<dyn BinaryOperator>,
    pub position: PositionRange
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, right: Box<dyn Expr>, operator_token: &Token) -> Box<dyn Expr> {
        let operator = operator::as_binary_operator(operator_token);

        let position = PositionRange::concat(left.get_position(), right.get_position());

        Box::new(BinaryExpr {
            left,
            right,
            operator,
            position
        })
    }
}

impl_expr!(BinaryExpr, visit_binary);

#[derive(Debug)]
pub struct UnaryExpr {
    pub expr: Box<dyn Expr>,
    pub operator: Box<dyn UnaryOperator>,
    pub position: PositionRange
}

impl UnaryExpr {
    pub fn new(expr: Box<dyn Expr>, operator_token: &Token) -> Box<dyn Expr> {
        let operator = operator::as_unary_operator(operator_token);

        let position = PositionRange::concat(expr.get_position(), &operator_token.position);

        Box::new(UnaryExpr {
            expr,
            operator,
            position
        })
    }
}

impl_expr!(UnaryExpr, visit_unary);

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
    pub parsed_type: ParsedType,
    pub position: PositionRange,
}

impl LiteralExpr {
    pub fn new(value: Literal, parsed_type: ParsedType, position: PositionRange) -> Box<dyn Expr> {
        Box::new(LiteralExpr {
            value,
            parsed_type,
            position
        })
    }
}

impl_expr!(LiteralExpr, visit_literal);

#[derive(Debug, Clone)]
pub enum MemberAccess {
    Direct(String),
    Indirect(String)
}

#[derive(Debug, Clone)]
pub struct VarExpr {
    pub id: i32,
    pub identifier: Rc<String>,
    pub member_accesses: Rc<Vec<MemberAccess>>,
    pub array_accesses: Rc<Vec<Box<dyn Expr>>>,
    pub n_derefs: i32,
    pub position: PositionRange
}

impl Eq for VarExpr {}

impl PartialEq for VarExpr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for VarExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl VarExpr {
    pub fn new_unboxed(id: i32, n_derefs: i32, identifier: String, member_accesses: Vec<MemberAccess>, array_accesses: Vec<Box<dyn Expr>>) -> VarExpr {
        VarExpr {
            id,
            n_derefs,
            identifier: Rc::new(identifier),
            member_accesses: Rc::new(member_accesses),
            array_accesses: Rc::new(array_accesses),
            position: PositionRange::new(Position::new(0, 0))
        }
    }

    pub fn clone(var_expr: &VarExpr) -> VarExpr {
        VarExpr {
            id: var_expr.id,
            n_derefs: var_expr.n_derefs,
            identifier: Rc::clone(&var_expr.identifier),
            member_accesses: Rc::clone(&var_expr.member_accesses),
            array_accesses: Rc::clone(&var_expr.array_accesses),
            position: var_expr.position
        }
    }
}

impl_expr!(VarExpr, visit_var);

#[derive(Debug)]
pub struct IfExpr {
    pub condition: Box<dyn Expr>,
    pub success: Box<dyn Expr>,
    pub fail: Option<Box<dyn Expr>>,
    pub position: PositionRange
}

impl IfExpr {
    pub fn new(condition: Box<dyn Expr>, success: Box<dyn Expr>, fail: Option<Box<dyn Expr>>) -> Box<dyn Expr> {
        let mut position = PositionRange::concat(condition.get_position(), success.get_position());
        
        if let Some(fail) = &fail {
            position = PositionRange::concat(&position, fail.get_position());
        }

        Box::new(IfExpr {
            condition,
            success,
            fail,
            position
        })
    }
}

impl_expr!(IfExpr, visit_if);

#[derive(Debug)]
pub struct DeclarationExpr {
    pub id: i32,
    pub identifier: String,
    pub declaration_type: ParsedType,
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl Eq for DeclarationExpr {}

impl PartialEq for DeclarationExpr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for DeclarationExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


impl DeclarationExpr {
    pub fn new(id: i32, identifier: String, declaration_type: ParsedType, expr: Box<dyn Expr>) -> Box<dyn Expr> {
        Box::new(DeclarationExpr {
            id,
            identifier,
            declaration_type,
            expr,
            position: PositionRange::new(Position::new(0, 0))
        })
    }
}

impl_expr!(DeclarationExpr, visit_declaration);

#[derive(Debug)]
pub struct AssignmentExpr {
    pub asignee: Rc<VarExpr>,
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

//Consider splitting assignment and declaration expressions
impl AssignmentExpr {
    pub fn new(asignee: VarExpr, expr: Box<dyn Expr>) -> Box<dyn Expr> {
        let position = PositionRange::concat(asignee.get_position(), expr.get_position());

        Box::new(AssignmentExpr {
            asignee: Rc::new(asignee),
            expr,
            position
        })
    }    
}

impl_expr!(AssignmentExpr, visit_assignment);

#[derive(Debug)]
pub struct BlockExpr {
    pub exprs: Vec<Box<dyn Expr>>,
    pub position: PositionRange
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn Expr>>) -> Box<dyn Expr> {
        let position = exprs.iter().fold(PositionRange::new(Position::new(0, 0)), |acc, expr| {
            PositionRange::concat(&acc, expr.get_position())
        });

        Box::new(BlockExpr {
            exprs,
            position
        })
    }    
}

impl_expr!(BlockExpr, visit_block);

#[derive(Debug)]
pub struct PrintExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl PrintExpr {
    pub fn new(expr: Box<dyn Expr>, print_token_position: &PositionRange) -> Box<dyn Expr> {
        let position = PositionRange::concat(&expr.get_position(), print_token_position);

        Box::new(PrintExpr {
            expr,
            position
        })
    }
}

impl_expr!(PrintExpr, visit_print);

#[derive(Debug)]
pub struct RandExpr {
    pub min: Box<dyn Expr>,
    pub max: Box<dyn Expr>,
    pub position: PositionRange
}

impl RandExpr {
    pub fn new(min: Box<dyn Expr>, max: Box<dyn Expr>, rand_token_position: &PositionRange) -> Box<dyn Expr> {
        let position = PositionRange::concat(&min.get_position(), &rand_token_position);

        Box::new(RandExpr {
            min,
            max,
            position
        })
    }
}

impl_expr!(RandExpr, visit_rand);

#[derive(Debug)]
pub struct LoopExpr {
    pub initial: Option<Box<dyn Expr>>,
    pub condition: Option<Box<dyn Expr>>,
    pub increment: Option<Box<dyn Expr>>,
    pub body: Box<dyn Expr>,
    pub position: PositionRange
}

impl LoopExpr {
    pub fn new(body: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(LoopExpr {
            initial: None, 
            condition: None, 
            increment: None, 
            body,
            position
        })
    }
    
    pub fn new_while(condition: Box<dyn Expr>, body: Box<dyn Expr>) -> Box<dyn Expr> {
        let position = PositionRange::concat(&condition.get_position(), &body.get_position());

        Box::new(LoopExpr {
            initial: None, 
            condition: Some(condition), 
            increment: None, 
            body,
            position
        })
    }

    pub fn new_for(initial: Box<dyn Expr>, condition: Box<dyn Expr>, increment: Box<dyn Expr>, body: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(LoopExpr {
            initial: Some(initial), 
            condition: Some(condition), 
            increment: Some(increment), 
            body,
            position
        })
    }
}

impl_expr!(LoopExpr, visit_loop);

#[derive(Debug)]
pub struct BreakExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl BreakExpr {
    pub fn new(expr: Box<dyn Expr>) -> Box<dyn Expr> {
        Box::new(BreakExpr {
            expr,
            position: PositionRange::new(Position::new(0, 0))
        })
    }
}

impl_expr!(BreakExpr, visit_break);

#[derive(Debug)]
pub struct InputExpr {
    pub prompt: Box<dyn Expr>,
    pub return_type: ParsedType,
    pub position: PositionRange
}

impl InputExpr {
    pub fn new(prompt: Box<dyn Expr>, return_type: ParsedType) -> Box<dyn Expr> {
        Box::new(InputExpr {
            prompt,
            return_type,
            position: PositionRange::new(Position::new(0, 0))
        })
    }    
}

impl_expr!(InputExpr, visit_input);

#[derive(Debug)]
pub struct CallExpr {
    pub function: VarExpr,
    pub args: Vec<Box<dyn Expr>>,
    pub position: PositionRange
}

impl CallExpr {
    pub fn new(function: VarExpr, args: Vec<Box<dyn Expr>>) -> Box<dyn Expr> {
        let position = PositionRange::new(Position::new(0, 0));

        Box::new(CallExpr {
            function,
            args,
            position
        })
    }
}

impl_expr!(CallExpr, visit_call);

#[derive(Debug, Clone)]
pub struct StructInitializerExpr {
    pub type_name: Rc<String>,
    pub member_inits: Rc<HashMap<String, Box<dyn Expr>>>,
    pub position: PositionRange
}

impl StructInitializerExpr {
    pub fn new(type_name: String, member_inits: HashMap<String, Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(StructInitializerExpr {
            type_name: Rc::new(type_name),
            member_inits: Rc::new(member_inits),
            position
        })
    }
}

impl_expr!(StructInitializerExpr, visit_struct_initializer);

#[derive(Debug)]
pub struct GetAddressExpr {
    pub var_expr: VarExpr,
    pub position: PositionRange
}

impl GetAddressExpr {
    pub fn new(var_expr: VarExpr, position: PositionRange) -> Box<dyn Expr> {
        Box::new(GetAddressExpr {
            var_expr,
            position
        })
    }
}

impl_expr!(GetAddressExpr, visit_get_address);

#[derive(Debug)]
pub struct StaticArrayExpr {
    pub len: usize,
    pub declaration_type: ParsedType,
    pub position: PositionRange
}

impl StaticArrayExpr {
    pub fn new(len: usize, declaration_type: ParsedType) -> Box<dyn Expr> {
        Box::new(StaticArrayExpr {
            len, declaration_type,
            position: PositionRange::new(Position::new(0, 0))
        })
    }
}

impl_expr!(StaticArrayExpr, visit_static_array);

#[derive(Debug)]
pub struct PutCharExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl PutCharExpr {
    pub fn new(expr: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(PutCharExpr {
            expr,
            position
        })
    }
}

impl_expr!(PutCharExpr, visit_put_char);

#[derive(Debug)]
pub struct GetCharExpr {
    pub position: PositionRange
}

impl GetCharExpr {
    pub fn new(position: PositionRange) -> Box<dyn Expr> {
        Box::new(GetCharExpr {
            position
        })
    }
}

impl_expr!(GetCharExpr, visit_get_char);