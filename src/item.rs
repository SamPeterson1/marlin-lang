use std::{collections::HashMap, rc::Rc, fmt};

use crate::{expr::Expr, token::PositionRange, types::parsed_type::ParsedType};

pub trait ItemVisitable<T> {
    fn accept_visitor(&self, visitor: &mut dyn ItemVisitor<T>) -> T;
}

pub trait Item: ItemVisitable<()> + fmt::Display {
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

#[derive(Clone)]
pub struct StructItem {
    pub name: Rc<String>,
    pub members: HashMap<String, ParsedType>,
    pub position: PositionRange,
}

impl fmt::Display for StructItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"struct\", \"name\": \"{}\", \"members\": {{", self.name)?;
        
        for (i, (name, ty)) in self.members.iter().enumerate() {
            write!(f, "\"{}\": {}", name, ty)?;

            if i < self.members.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}}}")
    }
}

impl StructItem {
    pub fn new(name: String, members: HashMap<String, ParsedType>, position: PositionRange) -> StructItem {
        StructItem {
            name: Rc::new(name),
            members,
            position
        }
    }
}

impl_item!(StructItem, visit_struct);

pub struct FunctionItem {
    pub name: Rc<String>,
    pub args: Vec<(String, ParsedType)>,
    pub expr: Box<dyn Expr>,
    pub ret_type: ParsedType,
    pub position: PositionRange,
}

impl fmt::Display for FunctionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"type\": \"function\", \"name\": \"{}\", \"args\": [", self.name)?;

        for (i, (name, ty)) in self.args.iter().enumerate() {
            write!(f, "{{\"name\": \"{}\", \"type\": {}}}", name, ty)?;

            if i < self.args.len() - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, "], \"return_type\": {}, \"expr\": {}}}", self.ret_type, self.expr)
    }
}

impl FunctionItem {
    pub fn new(name: String, args: Vec<(String, ParsedType)>, expr: Box<dyn Expr>, ret_type: ParsedType, position: PositionRange) -> FunctionItem {
        FunctionItem {
            name: Rc::new(name),
            args,
            expr,
            ret_type,
            position
        }
    }
}

impl_item!(FunctionItem, visit_function);