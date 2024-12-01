mod emmyrc;

use std::collections::HashSet;

use emmylua_parser::ParserConfig;
pub use emmyrc::Emmyrc;
use rowan::NodeCache;

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

    pub fn get_parse_config<'cache>(&self, node_cache: &'cache mut NodeCache) -> ParserConfig<'cache> {
        ParserConfig::new(emmylua_parser::LuaLanguageLevel::Lua54, Some(node_cache))
    }
}