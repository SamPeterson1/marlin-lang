use std::{collections::HashMap, rc::Rc};

use crate::{environment::ParsedType, token::PositionRange};

use super::{BlockExpr, Expr};

pub trait ItemVisitable<T> {
    fn accept_visitor(&self, visitor: &mut dyn ItemVisitor<T>) -> T;
}

pub trait Item: ItemVisitable<()> + std::fmt::Debug {
    fn get_position(&self) -> &PositionRange;
}

pub trait ItemVisitor<T> {
    fn visit_struct(&mut self, item: &StructItem) -> T;
    fn visit_function(&mut self, item: &FunctionItem) -> T;
}

macro_rules! impl_item {
    ($Name: ident, $VisitFunction: ident) => {
        impl Item for $Name {
            fn get_position(&self) -> &PositionRange {
                &self.position
            }
        }

        impl<T> ItemVisitable<T> for $Name {
            fn accept_visitor(&self, visitor: &mut dyn ItemVisitor<T>) -> T {
                visitor.$VisitFunction(self)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructItem {
    pub name: Rc<String>,
    pub members: HashMap<String, ParsedType>,
    pub position: PositionRange,
}

impl StructItem {
    pub fn new(name: String, members: HashMap<String, ParsedType>, position: PositionRange) -> Box<dyn Item> {
        Box::new(StructItem {
            name: Rc::new(name),
            members,
            position
        })
    }
}

impl_item!(StructItem, visit_struct);

#[derive(Debug)]
pub struct FunctionItem {
    pub name: Rc<String>,
    pub args: HashMap<String, ParsedType>,
    pub expr: Box<dyn Expr>,
    pub ret_type: ParsedType,
    pub position: PositionRange,
}

impl FunctionItem {
    pub fn new(name: String, args: HashMap<String, ParsedType>, expr: Box<dyn Expr>, ret_type: ParsedType, position: PositionRange) -> Box<dyn Item> {
        Box::new(FunctionItem {
            name: Rc::new(name),
            args,
            expr,
            ret_type,
            position
        })
    }
}

impl_item!(FunctionItem, visit_function);