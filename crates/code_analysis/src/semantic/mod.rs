mod infer;
mod member;

use std::collections::{HashMap, HashSet};

use emmylua_parser::{LuaExpr, LuaSyntaxId};
use infer::InferResult;
pub use infer::LuaInferConfig;

use crate::db_index::LuaTypeDeclId;
#[allow(unused_imports)]
use crate::{
    db_index::{DbIndex, LuaType},
    FileId,
};

#[derive(Debug)]
pub struct SemanticModel<'a> {
    file_id: FileId,
    db: &'a DbIndex,
    infer_config: LuaInferConfig,
    expr_type_caches: HashMap<LuaSyntaxId, LuaType>
}

impl<'a> SemanticModel<'a> {
    pub fn new(file_id: FileId, db: &'a DbIndex, infer_config: LuaInferConfig) -> Self {
        Self {
            file_id,
            db,
            infer_config,
            expr_type_caches: HashMap::new()
        }
    }

    pub fn infer_expr(&mut self, expr: LuaExpr) -> InferResult {
        infer::infer_expr(self.db, &mut self.infer_config, expr)
    }
}

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
