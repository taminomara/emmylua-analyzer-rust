mod emmyrc;

use std::collections::HashSet;

use emmylua_parser::{LuaLanguageLevel, ParserConfig};
pub use emmyrc::Emmyrc;
use emmyrc::LuaVersion;
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
        let level = match &self.runtime {
            Some(runtime) => runtime.version.as_ref().unwrap_or(&LuaVersion::Lua54),
            None => &LuaVersion::Lua54,
        };

        let lua_language_level = match level {
            LuaVersion::Lua51 => LuaLanguageLevel::Lua51,
            LuaVersion::Lua52 => LuaLanguageLevel::Lua52,
            LuaVersion::Lua53 => LuaLanguageLevel::Lua53,
            LuaVersion::Lua54 => LuaLanguageLevel::Lua54,
            LuaVersion::LuaJIT => LuaLanguageLevel::LuaJIT,
            LuaVersion::LuaLatest => LuaLanguageLevel::Lua54,
        };

        ParserConfig::new(lua_language_level, Some(node_cache))
    }
}