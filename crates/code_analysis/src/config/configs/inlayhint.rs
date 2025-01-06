use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcInlayHint {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_true")]
    pub param_hint: bool,
    #[serde(default = "default_true")]
    pub index_hint: bool,
    #[serde(default = "default_true")]
    pub local_hint: bool,
    #[serde(default = "default_true")]
    pub override_hint: bool,
}

impl Default for EmmyrcInlayHint {
    fn default() -> Self {
        Self {
            enable: default_true(),
            param_hint: default_true(),
            index_hint: default_true(),
            local_hint: default_true(),
            override_hint: default_true(),
        }
    }
}

fn default_true() -> bool {
    true
}
