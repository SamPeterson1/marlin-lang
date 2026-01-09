use std::collections::{HashMap, VecDeque};

use crate::{ast::{AstId, Path}, compiler::{Compiler, SymbolTable}, logger::LogTarget};

mod visitor;
mod witness;

pub struct LocalResolver<'ctx, 'ast> {
    log_target: &'ctx dyn LogTarget,
    compiler: &'ctx Compiler<'ast>,
    symbol_table: &'ctx SymbolTable<'ast>,
    scopes: VecDeque<HashMap<&'ast String, AstId>>,
    unknown_variables: Vec<&'ast Path>
}

impl<'ctx, 'ast> LocalResolver<'ctx, 'ast> {
    pub fn new(log_target: &'ctx dyn LogTarget, compiler: &'ctx Compiler<'ast>, symbol_table: &'ctx SymbolTable<'ast>) -> Self {
        Self {
            log_target,
            compiler,
            symbol_table,
            scopes: VecDeque::new(),
            unknown_variables: Vec::new(),
        }
    }
}
