use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcCompletion {
    pub auto_require: Option<bool>,
    pub auto_require_function: Option<String>,
    pub auto_require_naming_convention: Option<EmmyrcFilenameConvention>,
    pub call_snippet: Option<bool>,
    pub postfix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum EmmyrcFilenameConvention {
    Keep,
    SnakeCase,
    PascalCase,
    CamelCase,
}