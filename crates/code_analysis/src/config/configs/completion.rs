use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_require: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_require_function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_require_naming_convention: Option<EmmyrcFilenameConvention>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_snippet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
