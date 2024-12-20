use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcRuntime {
    #[serde(default)]
    pub version: EmmyrcLuaVersion,
    #[serde(default)]
    pub require_like_function: Vec<String>,
    #[serde(default)]
    pub framework_versions: Vec<String>,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub require_pattern: Vec<String>,
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

impl Default for EmmyrcLuaVersion {
    fn default() -> Self {
        EmmyrcLuaVersion::LuaLatest
    }
}

