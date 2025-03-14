use std::collections::HashMap;

use crate::{semantic::LuaInferCache, FileId};

#[derive(Debug)]
pub struct InferCacheManager {
    infer_map: HashMap<FileId, LuaInferCache>,
}

impl InferCacheManager {
    pub fn new() -> Self {
        InferCacheManager {
            infer_map: HashMap::new(),
        }
    }

    pub fn get_infer_cache(&mut self, file_id: FileId) -> &mut LuaInferCache {
        self.infer_map.entry(file_id).or_insert_with(|| {
            LuaInferCache::new(
                file_id,
                crate::CacheOptions {
                    analysis_phase: false,
                },
            )
        })
    }
}
