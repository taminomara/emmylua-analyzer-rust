use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcRuntime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<EmmyrcLuaVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_like_function: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_versions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_pattern: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum EmmyrcLuaVersion {
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
