use std::collections::HashSet;

use crate::FileId;

#[derive(Debug)]
pub struct LuaInferConfig {
    pub require_function: HashSet<String>,
    pub file_id: FileId,
}

impl LuaInferConfig {
    pub fn new(file_id: FileId, require_function: HashSet<String>) -> Self {
        Self {
            require_function,
            file_id,
        }
    }

    pub fn is_require_function(&self, function_name: &str) -> bool {
        self.require_function.contains(function_name)
    }
}