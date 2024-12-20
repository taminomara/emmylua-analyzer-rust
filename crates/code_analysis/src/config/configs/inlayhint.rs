use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcInlayHint {
    #[serde(default = "default_true")]
    pub param_hint: bool,
    #[serde(default = "default_true")]
    pub index_hint: bool,
    #[serde(default = "default_true")]
    pub local_hint: bool,
    #[serde(default = "default_true")]
    pub override_hint: bool,
}

fn default_true() -> bool {
    true
}
