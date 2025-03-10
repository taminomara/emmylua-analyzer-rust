use std::{collections::{HashMap, HashSet}, sync::Arc};

use emmylua_parser::LuaSyntaxId;

use crate::{db_index::LuaType, FileId, LuaFunctionType};

#[derive(Debug)]
pub struct LuaInferCache {
    require_function: HashSet<String>,
    file_id: FileId,
    expr_type_cache: HashMap<LuaSyntaxId, ExprCache>,
    call_expr_resolve_cache: HashMap<(LuaSyntaxId, Option<usize>), CallCache>,
}

#[derive(Debug)]
pub enum ExprCache {
    ReadyCache,
    Cache(LuaType),
}

#[derive(Debug)]
pub enum CallCache {
    ReadyCache,
    Cache(Arc<LuaFunctionType>),
}

impl LuaInferCache {
    pub fn new(file_id: FileId, require_function: HashSet<String>) -> Self {
        Self {
            require_function,
            file_id,
            expr_type_cache: HashMap::new(),
            call_expr_resolve_cache: HashMap::new(),
        }
    }

    pub fn is_require_function(&self, function_name: &str) -> bool {
        if self.require_function.contains(function_name) {
            return true;
        }

        function_name == "require"
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn mark_expr_ready_cache(&mut self, syntax_id: LuaSyntaxId) {
        self.expr_type_cache
            .insert(syntax_id, ExprCache::ReadyCache);
    }

    pub fn cache_expr_type(&mut self, syntax_id: LuaSyntaxId, ty: LuaType) {
        self.expr_type_cache.insert(syntax_id, ExprCache::Cache(ty));
    }

    pub fn get_cache_expr_type(&self, syntax_id: &LuaSyntaxId) -> Option<&ExprCache> {
        self.expr_type_cache.get(syntax_id)
    }

    pub fn clear_expr_cache(&mut self, syntax_id: &LuaSyntaxId) {
        self.expr_type_cache.remove(syntax_id);
    }

    pub fn mark_call_expr_ready_cache(&mut self, key: (LuaSyntaxId, Option<usize>)) {
        self.call_expr_resolve_cache
            .insert(key, CallCache::ReadyCache);
    }

    pub fn cache_call_expr(&mut self, key: (LuaSyntaxId, Option<usize>), ty: Arc<LuaFunctionType>) {
        self.call_expr_resolve_cache.insert(key, CallCache::Cache(ty));
    }

    pub fn get_cache_call_expr(&self, key: &(LuaSyntaxId, Option<usize>)) -> Option<&CallCache> {
        self.call_expr_resolve_cache.get(key)
    }

    pub fn clear_call_expr_cache(&mut self, key: &(LuaSyntaxId, Option<usize>)) {
        self.call_expr_resolve_cache.remove(key);
    }
}
