use std::{collections::HashMap, sync::Arc};

use crate::{semantic::LuaInferCache, Emmyrc, FileId};

#[derive(Debug)]
pub struct InferManager {
    emmyrc: Arc<Emmyrc>,
    infer_map: HashMap<FileId, LuaInferCache>,
}

impl InferManager {
    pub fn new(emmyrc: Arc<Emmyrc>) -> Self {
        InferManager {
            emmyrc,
            infer_map: HashMap::new(),
        }
    }

    pub fn get_infer_config(&mut self, file_id: FileId) -> &mut LuaInferCache {
        self.infer_map
            .entry(file_id)
            .or_insert_with(|| self.emmyrc.get_infer_config(file_id))
    }
}
