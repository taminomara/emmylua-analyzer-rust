use std::{collections::HashMap, sync::Arc};

use crate::{semantic::LuaInferConfig, Emmyrc, FileId};

#[derive(Debug)]
pub struct InferManager {
    emmyrc: Arc<Emmyrc>,
    infer_map: HashMap<FileId, LuaInferConfig>,
}

impl InferManager {
    pub fn new(emmyrc: Arc<Emmyrc>) -> Self {
        InferManager {
            emmyrc,
            infer_map: HashMap::new(),
        }
    }

    pub fn get_infer_config(&mut self, file_id: FileId) -> &mut LuaInferConfig {
        self.infer_map
            .entry(file_id)
            .or_insert_with(|| self.emmyrc.get_infer_config(file_id))
    }
}
