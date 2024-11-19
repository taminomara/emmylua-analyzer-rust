mod infer;

use emmylua_parser::LuaExpr;
use infer::InferResult;

use crate::{db_index::{DbIndex, LuaType}, FileId};

#[derive(Debug)]
pub struct SemanticModel<'a> {
    file_id: FileId,
    db: &'a DbIndex
}

impl<'a> SemanticModel<'a> {
    pub fn new(file_id: FileId, db:&'a DbIndex) -> Self {
        Self {
            file_id,
            db
        }
    }

    pub fn infer_expr(&self, expr: LuaExpr) -> InferResult {
        infer::infer_expr(self.db, expr)
    }
}