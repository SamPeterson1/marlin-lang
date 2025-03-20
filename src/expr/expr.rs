use std::{collections::HashMap, fmt, hash::Hasher, rc::Rc};

use crate::{environment::{Literal, ParsedType, ResolvedType}, operator::{self, BinaryOperator, UnaryOperator}, token::{Position, PositionRange, Token, TokenType}};

pub trait ExprVisitable<T> {
    fn accept_visitor(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}

pub trait Expr: ExprVisitable<ResolvedType> + ExprVisitable<()> + std::fmt::Debug + fmt::Display {
    fn get_position(&self) -> &PositionRange;
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
pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: Box<dyn BinaryOperator>,
    pub position: PositionRange
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Binary\", \"left\": {}, \"right\": {}, \"operator\": \"{}\", \"position\": \"{}\"}}", self.left, self.right, self.operator, self.position)
    }
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, right: Box<dyn Expr>, operator_token: TokenType) -> Box<dyn Expr> {
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

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Unary\", \"expr\": {}, \"operator\": \"{}\", \"position\": \"{}\"}}", self.expr, self.operator, self.position)
    }
}

impl UnaryExpr {
    pub fn new(expr: Box<dyn Expr>, operator_token: Token) -> Box<dyn Expr> {
        let operator = operator::as_unary_operator(operator_token.token_type);

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

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Literal\", \"value\": {}, \"parsed_type\": \"{:?}\", \"position\": \"{}\"}}", self.value, self.parsed_type, self.position)
    }
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

impl fmt::Display for MemberAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemberAccess::Direct(name) => write!(f, "{{\"type\": \"Direct\", \"name\": \"{}\"}}", name),
            MemberAccess::Indirect(name) => write!(f, "{{\"type\": \"Indirect\", \"name\": \"{}\"}}", name)
        }
    }
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

impl fmt::Display for VarExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Var\", \"identifier\": \"{}\", \"member_accesses\": [", self.identifier)?;

        for (i, member_access) in self.member_accesses.iter().enumerate() {
            write!(f, "{}", member_access)?;

            if i + 1 < self.member_accesses.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"array_accesses\": [")?;

        for (i, array_access) in self.array_accesses.iter().enumerate() {
            write!(f, "{}", array_access)?;

            if i + 1 < self.array_accesses.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"n_derefs\": {}, \"position\": \"{}\"}}", self.n_derefs, self.position)
    }
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
    pub fn new_unboxed(id: i32, n_derefs: i32, identifier: String, member_accesses: Vec<MemberAccess>, array_accesses: Vec<Box<dyn Expr>>, position: PositionRange) -> VarExpr {
        VarExpr {
            id,
            n_derefs,
            identifier: Rc::new(identifier),
            member_accesses: Rc::new(member_accesses),
            array_accesses: Rc::new(array_accesses),
            position
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

impl fmt::Display for IfExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"If\", \"condition\": {}, \"success\": {},", self.condition, self.success)?;

        if let Some(fail) = &self.fail {
            write!(f, " \"fail\": {},", fail)?;
        }

        write!(f, " \"position\": \"{}\"}}", self.position)
    }
}

impl IfExpr {
    pub fn new(condition: Box<dyn Expr>, success: Box<dyn Expr>, fail: Option<Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
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

impl fmt::Display for DeclarationExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Declaration\", \"identifier\": \"{}\", \"declaration_type\": {}, \"expr\": {}, \"position\": \"{}\"}}", self.identifier, self.declaration_type, self.expr, self.position)
    }
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
    pub fn new(id: i32, identifier: String, declaration_type: ParsedType, expr: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(DeclarationExpr {
            id,
            identifier,
            declaration_type,
            expr,
            position,
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

impl fmt::Display for AssignmentExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Assignment\", \"asignee\": {}, \"expr\": {}, \"position\": \"{}\"}}", self.asignee, self.expr, self.position)
    }
}

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

impl fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Block\", \"exprs\": [")?;

        for (i, expr) in self.exprs.iter().enumerate() {
            write!(f, "{}", expr)?;

            if i + 1 < self.exprs.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"position\": \"{}\"}}", self.position)
    }
}

impl BlockExpr {
    pub fn new(exprs: Vec<Box<dyn Expr>>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(BlockExpr {
            exprs,
            position
        })
    }    
}

impl_expr!(BlockExpr, visit_block);

#[derive(Debug)]
pub struct LoopExpr {
    pub initial: Option<Box<dyn Expr>>,
    pub condition: Option<Box<dyn Expr>>,
    pub increment: Option<Box<dyn Expr>>,
    pub body: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for LoopExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Loop\"")?;

        if let Some(initial) = &self.initial {
            write!(f, ", \"initial\": {}", initial)?;
        }

        if let Some(condition) = &self.condition {
            write!(f, ", \"condition\": {}", condition)?;
        }

        if let Some(increment) = &self.increment {
            write!(f, ", \"increment\": {}", increment)?;
        }

        write!(f, ", \"body\": {}, \"position\": \"{}\"}}", self.body, self.position)
    }
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
    
    pub fn new_while(condition: Box<dyn Expr>, body: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
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

impl fmt::Display for BreakExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Break\", \"expr\": {}, \"position\": \"{}\"}}", self.expr, self.position)
    }
}

impl BreakExpr {
    pub fn new(expr: Box<dyn Expr>, position: PositionRange) -> Box<dyn Expr> {
        Box::new(BreakExpr {
            expr,
            position
        })
    }
}

impl_expr!(BreakExpr, visit_break);

#[derive(Debug)]
pub struct CallExpr {
    pub function: String,
    pub args: Vec<Box<dyn Expr>>,
    pub position: PositionRange
}

impl fmt::Display for CallExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"Call\", \"function\": \"{}\", \"args\": [", self.function)?;

        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i + 1 < self.args.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "], \"position\": \"{}\"}}", self.position)
    }
}

impl CallExpr {
    pub fn new(function: String, args: Vec<Box<dyn Expr>>) -> Box<dyn Expr> {
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

impl fmt::Display for StructInitializerExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"StructInitializer\", \"type_name\": \"{}\", \"member_inits\": {{", self.type_name)?;

        for (i, (name, expr)) in self.member_inits.iter().enumerate() {
            write!(f, "\"{}\": {}", name, expr)?;

            if i + 1 < self.member_inits.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}, \"position\": \"{}\"}}", self.position)
    }
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

impl fmt::Display for GetAddressExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"GetAddress\", \"var_expr\": {}, \"position\": \"{}\"}}", self.var_expr, self.position)
    }
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

impl fmt::Display for StaticArrayExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"StaticArray\", \"len\": {}, \"declaration_type\": {}, \"position\": \"{}\"}}", self.len, self.declaration_type, self.position)
    }
}

impl StaticArrayExpr {
    pub fn new(len: usize, declaration_type: ParsedType, position: PositionRange) -> Box<dyn Expr> {
        Box::new(StaticArrayExpr {
            len, declaration_type,
            position
        })
    }
}

impl_expr!(StaticArrayExpr, visit_static_array);

#[derive(Debug)]
pub struct PutCharExpr {
    pub expr: Box<dyn Expr>,
    pub position: PositionRange
}

impl fmt::Display for PutCharExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"PutChar\", \"expr\": {}, \"position\": \"{}\"}}", self.expr, self.position)
    }
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

impl fmt::Display for GetCharExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"GetChar\", \"position\": \"{}\"}}", self.position)
    }
}

impl GetCharExpr {
    pub fn new(position: PositionRange) -> Box<dyn Expr> {
        Box::new(GetCharExpr {
            position
        })
    }
}

impl_expr!(GetCharExpr, visit_get_char);