use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::DiagnosticCode;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Setting {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    pub completion: Option<Completion>,
    pub signature: Option<Signature>,
    pub diagnostics: Option<Diagnostics>,
    pub hint: Option<Hint>,
    pub runtime: Option<Runtime>,
    pub workspace: Option<Workspace>,
    pub resource: Option<Resource>,
    pub code_lens: Option<CodeLens>,
    pub strict: Option<Strict>,
    pub semantic_tokens: Option<SemanticTokens>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Completion {
    pub auto_require: Option<bool>,
    pub auto_require_function: Option<String>,
    pub auto_require_naming_convention: Option<FilenameConvention>,
    pub call_snippet: Option<bool>,
    pub postfix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Diagnostics {
    pub disable: Option<Vec<DiagnosticCode>>,
    pub enable: Option<bool>,
    pub globals: Option<Vec<String>>,
    pub globals_regex: Option<Vec<String>>,
    pub severity: Option<HashMap<DiagnosticCode, DiagnosticSeveritySetting>>,
    pub enables: Option<Vec<DiagnosticCode>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Hint {
    pub param_hint: Option<bool>,
    pub index_hint: Option<bool>,
    pub local_hint: Option<bool>,
    pub override_hint: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Runtime {
    pub version: Option<LuaVersion>,
    pub require_like_function: Option<Vec<String>>,
    pub framework_versions: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
    pub require_pattern: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum LuaVersion {
    #[serde(rename = "Lua5.1")]
    Lua51,
    #[serde(rename = "LuaJIT")]
    LuaJIT,
    #[serde(rename = "Lua5.2")]
    Lua52,
    #[serde(rename = "Lua5.3")]
    Lua53,
    #[serde(rename = "Lua5.4")]
    Lua54,
    #[serde(rename = "LuaLatest")]
    LuaLatest,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Workspace {
    pub ignore_dir: Option<Vec<String>>,
    pub ignore_globs: Option<Vec<String>>,
    pub library: Option<Vec<String>>,
    pub workspace_roots: Option<Vec<String>>,
    pub preload_file_size: Option<i32>,
    pub encoding: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Resource {
    pub paths: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CodeLens {
    pub enable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Strict {
    pub require_path: Option<bool>,
    pub type_call: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Signature {
    pub detail_signature_helper: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct SemanticTokens {
    pub enable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FilenameConvention {
    Keep,
    SnakeCase,
    PascalCase,
    CamelCase,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum DiagnosticSeveritySetting {
    Error,
    Warning,
    Information,
    Hint,
}

impl From<DiagnosticSeveritySetting> for DiagnosticSeverity {
    fn from(severity: DiagnosticSeveritySetting) -> Self {
        match severity {
            DiagnosticSeveritySetting::Error => DiagnosticSeverity::ERROR,
            DiagnosticSeveritySetting::Warning => DiagnosticSeverity::WARNING,
            DiagnosticSeveritySetting::Information => DiagnosticSeverity::INFORMATION,
            DiagnosticSeveritySetting::Hint => DiagnosticSeverity::HINT,
        }
    }
}
