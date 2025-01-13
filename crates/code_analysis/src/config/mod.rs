mod config_loader;
mod configs;
mod flatten_config;

use std::{collections::HashSet, path::PathBuf};

use crate::{semantic::LuaInferConfig, FileId};
pub use config_loader::load_configs;
pub use configs::EmmyrcFilenameConvention;
pub use configs::EmmyrcLuaVersion;
use configs::{
    EmmyrcCodeLen, EmmyrcCompletion, EmmyrcDiagnostic, EmmyrcHover, EmmyrcInlayHint,
    EmmyrcReference, EmmyrcResource, EmmyrcRuntime, EmmyrcSemanticToken, EmmyrcSignature,
    EmmyrcStrict, EmmyrcWorkspace,
};
use emmylua_parser::{LuaLanguageLevel, ParserConfig};
use regex::Regex;
use rowan::NodeCache;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Emmyrc {
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default)]
    pub completion: EmmyrcCompletion,
    #[serde(default)]
    pub diagnostics: EmmyrcDiagnostic,
    #[serde(default)]
    pub signature: EmmyrcSignature,
    #[serde(default)]
    pub hint: EmmyrcInlayHint,
    #[serde(default)]
    pub runtime: EmmyrcRuntime,
    #[serde(default)]
    pub workspace: EmmyrcWorkspace,
    #[serde(default)]
    pub resource: EmmyrcResource,
    #[serde(default)]
    pub code_lens: EmmyrcCodeLen,
    #[serde(default)]
    pub strict: EmmyrcStrict,
    #[serde(default)]
    pub semantic_tokens: EmmyrcSemanticToken,
    #[serde(default)]
    pub references: EmmyrcReference,
    #[serde(default)]
    pub hover: EmmyrcHover,
}

impl Emmyrc {
    pub fn get_infer_config(&self, file_id: FileId) -> LuaInferConfig {
        let require_map: HashSet<String> =
            self.runtime.require_like_function.iter().cloned().collect();

        LuaInferConfig::new(file_id, require_map)
    }

    pub fn get_parse_config<'cache>(
        &self,
        node_cache: &'cache mut NodeCache,
    ) -> ParserConfig<'cache> {
        let level = &self.runtime.version;

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

    pub fn pre_process_emmyrc(&mut self, workspace_root: &PathBuf) {
        let new_workspace_roots = self
            .workspace
            .workspace_roots
            .iter()
            .map(|root| pre_process_path(root, workspace_root))
            .collect::<Vec<String>>();
        self.workspace.workspace_roots = new_workspace_roots;

        let new_ignore_dir = self
            .workspace
            .ignore_dir
            .iter()
            .map(|dir| pre_process_path(dir, workspace_root))
            .collect::<Vec<String>>();
        self.workspace.ignore_dir = new_ignore_dir;

        let new_paths = self
            .resource
            .paths
            .iter()
            .map(|path| pre_process_path(path, workspace_root))
            .collect::<Vec<String>>();
        self.resource.paths = new_paths;
    }
}

fn pre_process_path(path: &str, workspace: &PathBuf) -> String {
    let mut path = path.to_string();

    if path.starts_with('~') {
        let home_dir = dirs::home_dir().unwrap();
        path = home_dir.join(&path[1..]).to_string_lossy().to_string();
    } else if path.starts_with("./") {
        path = workspace.join(&path[2..]).to_string_lossy().to_string();
    } else if path.starts_with('/') {
        path = workspace
            .join(path.trim_start_matches('/'))
            .to_string_lossy()
            .to_string();
    } else if PathBuf::from(&path).is_absolute() {
        path = path.to_string();
    } else {
        path = workspace.join(&path).to_string_lossy().to_string();
    }

    path = path.replace("$", "");
    path = replace_placeholders(&path, workspace.to_str().unwrap());
    path
}

fn replace_placeholders(input: &str, workspace_folder: &str) -> String {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        let key = &caps[1];
        if key == "workspaceFolder" {
            workspace_folder.to_string()
        } else if let Some(env_name) = key.strip_prefix("env:") {
            std::env::var(env_name).unwrap_or_default()
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}
