use std::fmt;

use crate::token::PositionRange;

use super::Expr;

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

crate::impl_expr!(LoopExpr, visit_loop);