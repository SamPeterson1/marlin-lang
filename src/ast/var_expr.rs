use std::sync::Mutex;

use serde::Serialize;

use crate::{impl_ast_node};
use crate::lexer::token::{Located, PositionRange, Positioned};

static VAR_ID_COUNTER: Mutex<u64> = Mutex::new(0);

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub struct VarId(u64);

#[derive(Serialize)]
pub struct VarExpr {
    pub identifier: Located<String>,
    pub id: VarId,
}

impl VarExpr {
    pub fn new(identifier: Located<String>) -> Self {
        let mut id = VAR_ID_COUNTER.lock().unwrap();
        let current_id = *id;
        *id = *id + 1;

        Self {
            identifier,
            id: VarId(current_id),
        }
    }
}

impl Positioned for VarExpr {
    fn get_position(&self) -> &PositionRange {
        self.identifier.get_position()
    }
}

impl_ast_node!(VarExpr, visit_var);