use std::sync::Mutex;

use serde::Serialize;

use crate::resolver::ResolvedType;
use crate::{impl_ast_node, impl_typed};
use crate::lexer::token::{Located, PositionRange, Positioned};

static VAR_ID_COUNTER: Mutex<u64> = Mutex::new(0);

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct VarId(u64);

#[derive(Serialize)]
pub struct VarExpr {
    pub identifier: Located<String>,
    pub id: VarId,
    resolved_type: Option<ResolvedType>,
}

impl VarExpr {
    pub fn new(identifier: Located<String>) -> Self {
        let mut id = VAR_ID_COUNTER.lock().unwrap();
        let current_id = *id;
        *id = *id + 1;

        Self {
            identifier,
            id: VarId(current_id),
            resolved_type: None,
        }
    }
}

impl Positioned for VarExpr {
    fn get_position(&self) -> &PositionRange {
        self.identifier.get_position()
    }
}

impl_typed!(VarExpr);
impl_ast_node!(VarExpr, visit_var);