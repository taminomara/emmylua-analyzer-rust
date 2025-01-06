use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
/// Configuration for EmmyLua code completion.
pub struct EmmyrcCompletion {
    /// Whether to enable code completion.
    #[serde(default = "default_true")]
    pub enable: bool,
    /// Whether to automatically require modules.
    #[serde(default = "default_true")]
    pub auto_require: bool,
    /// The function used for auto-requiring modules.
    #[serde(default = "default_require_function")]
    pub auto_require_function: String,
    /// The naming convention for auto-required filenames.
    #[serde(default)]
    pub auto_require_naming_convention: EmmyrcFilenameConvention,
    /// Whether to use call snippets in completions.
    #[serde(default)]
    pub call_snippet: bool,
    /// The postfix trigger used in completions.
    #[serde(default = "default_postfix")]
    pub postfix: String,
}

impl Default for EmmyrcCompletion {
    fn default() -> Self {
        Self {
            enable: default_true(),
            auto_require: default_true(),
            auto_require_function: default_require_function(),
            auto_require_naming_convention: Default::default(),
            call_snippet: false,
            postfix: default_postfix(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_require_function() -> String {
    "require".to_string()
}

fn default_postfix() -> String {
    "@".to_string()
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum EmmyrcFilenameConvention {
    Keep,
    SnakeCase,
    PascalCase,
    CamelCase,
}

impl Default for EmmyrcFilenameConvention {
    fn default() -> Self {
        EmmyrcFilenameConvention::Keep
    }
}

