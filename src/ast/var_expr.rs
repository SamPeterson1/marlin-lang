use serde::Serialize;

use crate::ast::{AstId, Parsed, Phase};
use crate::ast::path::Path;
use crate::{impl_ast_node, new_ast_id};
use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize)]
pub struct VarExpr<P: Phase = Parsed> {
    pub path: Path,
    position: PositionRange,
    id: AstId,

    #[serde(skip)]
    _phase: std::marker::PhantomData<P>,
}

impl VarExpr {
    pub fn new(path: Located<Path>) -> Self {
        let (path, position) = path.into_parts();

        Self {
            path,
            position,
            id: new_ast_id!(),
            _phase: std::marker::PhantomData,
        }
    }
}

impl_ast_node!(VarExpr, visit_var);