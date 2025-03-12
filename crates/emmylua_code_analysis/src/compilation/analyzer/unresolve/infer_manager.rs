use std::collections::HashMap;

use crate::{semantic::LuaInferCache, FileId};

#[derive(Debug)]
pub struct InferManager {
    infer_map: HashMap<FileId, LuaInferCache>,
}

impl InferManager {
    pub fn new() -> Self {
        InferManager {
            infer_map: HashMap::new(),
        }
    }

    pub fn get_infer_cache(&mut self, file_id: FileId) -> &mut LuaInferCache {
        self.infer_map
            .entry(file_id)
            .or_insert_with(|| LuaInferCache::new(file_id))
    }
}
