use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
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

impl Default for EmmyrcRuntime {
    fn default() -> Self {
        Self {
            version: Default::default(),
            require_like_function: Default::default(),
            framework_versions: Default::default(),
            extensions: Default::default(),
            require_pattern: Default::default(),
        }
    }
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

