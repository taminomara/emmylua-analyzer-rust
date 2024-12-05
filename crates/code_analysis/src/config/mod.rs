mod config_loader;
mod configs;

use std::collections::HashSet;

use crate::{semantic::LuaInferConfig, FileId};
pub use config_loader::load_configs;
use configs::{
    EmmyrcCodeLen, EmmyrcCompletion, EmmyrcDiagnostic, EmmyrcInlayHint, EmmyrcLuaVersion,
    EmmyrcResource, EmmyrcRuntime, EmmyrcSemanticToken, EmmyrcSignature, EmmyrcStrict,
    EmmyrcWorkspace,
};
use emmylua_parser::{LuaLanguageLevel, ParserConfig};
use rowan::NodeCache;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Emmyrc {
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<EmmyrcCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<EmmyrcDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<EmmyrcSignature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<EmmyrcInlayHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<EmmyrcRuntime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<EmmyrcWorkspace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<EmmyrcResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens: Option<EmmyrcCodeLen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<EmmyrcStrict>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn get_encoding(&self) -> &str {
        if let Some(workspace) = &self.workspace {
            workspace.encoding.as_deref().unwrap_or("utf-8")
        } else {
            "utf-8"
        }
    }
}
