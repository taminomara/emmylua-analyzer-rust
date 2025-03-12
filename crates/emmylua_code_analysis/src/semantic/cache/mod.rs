use std::{collections::HashMap, sync::Arc};

use emmylua_parser::LuaSyntaxId;

use crate::{db_index::LuaType, FileId, LuaFunctionType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CacheKey {
    Expr(LuaSyntaxId),
    Call(LuaSyntaxId, Option<usize>),
}

#[derive(Debug)]
pub enum CacheEntry {
    ReadyCache,
    ExprCache(LuaType),
    CallCache(Arc<LuaFunctionType>),
}

#[derive(Debug)]
pub struct LuaInferCache {
    file_id: FileId,
    cache: HashMap<CacheKey, CacheEntry>,
}

impl LuaInferCache {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            cache: HashMap::new(),
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    // 表达式缓存相关方法
    pub fn ready_cache(&mut self, key: &CacheKey) {
        self.cache.insert(*key, CacheEntry::ReadyCache);
    }

    pub fn add_cache(&mut self, key: &CacheKey, value: CacheEntry) {
        self.cache.insert(*key, value);
    }

    pub fn get(&self, key: &CacheKey) -> Option<&CacheEntry> {
        self.cache.get(key)
    }

    pub fn remove(&mut self, key: &CacheKey) {
        self.cache.remove(key);
    }
}
