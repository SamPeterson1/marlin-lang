use std::{fmt, hash::Hasher, rc::Rc};

use crate::{impl_expr, token::PositionRange};

use super::Expr;

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

#[derive(Clone)]
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
}

impl_expr!(VarExpr, visit_var);