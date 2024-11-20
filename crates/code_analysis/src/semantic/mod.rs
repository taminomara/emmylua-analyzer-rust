mod infer;

use emmylua_parser::LuaExpr;
use infer::InferResult;
pub use infer::LuaInferConfig;

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
}

impl<'a> SemanticModel<'a> {
    pub fn new(file_id: FileId, db: &'a DbIndex, infer_config: LuaInferConfig) -> Self {
        Self {
            file_id,
            db,
            infer_config,
        }
    }

    pub fn infer_expr(&self, expr: LuaExpr) -> InferResult {
        infer::infer_expr(self.db, &self.infer_config, expr)
    }
}
