mod configs;


use std::collections::HashSet;

use configs::{EmmyrcCodeLen, EmmyrcCompletion, EmmyrcDiagnostics, EmmyrcInlayHint, EmmyrcLuaVersion, EmmyrcResource, EmmyrcRuntime, EmmyrcSemanticToken, EmmyrcSignature, EmmyrcStrict, EmmyrcWorkspace};
use emmylua_parser::{LuaLanguageLevel, ParserConfig};
use rowan::NodeCache;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{semantic::LuaInferConfig, FileId};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct Emmyrc {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    pub completion: Option<EmmyrcCompletion>,
    pub diagnostics: Option<EmmyrcDiagnostics>,
    pub signature: Option<EmmyrcSignature>,
    pub hint: Option<EmmyrcInlayHint>,
    pub runtime: Option<EmmyrcRuntime>,
    pub workspace: Option<EmmyrcWorkspace>,
    pub resource: Option<EmmyrcResource>,
    pub code_lens: Option<EmmyrcCodeLen>,
    pub strict: Option<EmmyrcStrict>,
    pub semantic_tokens: Option<EmmyrcSemanticToken>,
}

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

    pub fn get_parse_config<'cache>(
        &self,
        node_cache: &'cache mut NodeCache,
    ) -> ParserConfig<'cache> {
        let level = match &self.runtime {
            Some(runtime) => runtime.version.as_ref().unwrap_or(&EmmyrcLuaVersion::Lua54),
            None => &EmmyrcLuaVersion::Lua54,
        };

        let lua_language_level = match level {
            EmmyrcLuaVersion::Lua51 => LuaLanguageLevel::Lua51,
            EmmyrcLuaVersion::Lua52 => LuaLanguageLevel::Lua52,
            EmmyrcLuaVersion::Lua53 => LuaLanguageLevel::Lua53,
            EmmyrcLuaVersion::Lua54 => LuaLanguageLevel::Lua54,
            EmmyrcLuaVersion::LuaJIT => LuaLanguageLevel::LuaJIT,
            EmmyrcLuaVersion::LuaLatest => LuaLanguageLevel::Lua54,
        };

        ParserConfig::new(lua_language_level, Some(node_cache))
    }

}
