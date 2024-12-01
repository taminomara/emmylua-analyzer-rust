mod infer;
mod instantiate;
mod member;
mod overload_resolve;
mod type_calc;
mod type_compact;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use emmylua_parser::{LuaExpr, LuaSyntaxId};
use infer::InferResult;
pub use infer::LuaInferConfig;

use crate::{db_index::LuaTypeDeclId, Emmyrc, LuaDocument};
#[allow(unused_imports)]
use crate::{
    db_index::{DbIndex, LuaType},
    FileId,
};
pub(crate) use infer::infer_expr;

#[derive(Debug)]
pub struct SemanticModel<'a> {
    file_id: FileId,
    db: &'a DbIndex,
    infer_config: LuaInferConfig,
    expr_type_caches: HashMap<LuaSyntaxId, LuaType>,
    emmyrc: Arc<Emmyrc>,
}

impl<'a> SemanticModel<'a> {
    pub fn new(
        file_id: FileId,
        db: &'a DbIndex,
        infer_config: LuaInferConfig,
        emmyrc: Arc<Emmyrc>,
    ) -> Self {
        Self {
            file_id,
            db,
            infer_config,
            expr_type_caches: HashMap::new(),
            emmyrc,
        }
    }

    pub fn get_document(&self) -> LuaDocument {
        self.db.get_vfs().get_document(&self.file_id).unwrap()
    }

    pub fn infer_expr(&mut self, expr: LuaExpr) -> InferResult {
        infer_expr(self.db, &mut self.infer_config, expr)
    }

    pub fn get_emmyrc(&self) -> &Emmyrc {
        &self.emmyrc
    }
}

/// Guard to prevent infinite recursion
/// Some type may reference itself, so we need to check if we have already infered this type
#[derive(Debug)]
struct InferGuard {
    guard: HashSet<LuaTypeDeclId>,
}

impl InferGuard {
    fn new() -> Self {
        Self {
            guard: HashSet::default(),
        }
    }

    fn check(&mut self, type_id: &LuaTypeDeclId) -> Option<()> {
        if self.guard.contains(type_id) {
            return None;
        }
        self.guard.insert(type_id.clone());
        Some(())
    }
}
