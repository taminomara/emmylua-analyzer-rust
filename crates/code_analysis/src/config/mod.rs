mod emmyrc;

use std::collections::HashSet;

pub use emmyrc::Emmyrc;

use crate::{semantic::LuaInferConfig, FileId};

impl Emmyrc {
    pub fn get_infer_config(&self, file_id: FileId) -> LuaInferConfig {
        let mut require_map: HashSet<String> = HashSet::new();
        if let Some(runtime) = &self.runtime {
            if let Some(require_like_func) = &runtime.require_like_function {
                for func in require_like_func {
                    require_map.insert(func.clone());
                }
            }
        }
        
        LuaInferConfig::new(file_id, require_map)
    }
}