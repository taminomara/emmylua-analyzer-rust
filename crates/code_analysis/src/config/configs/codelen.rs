use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcCodeLen {
    /// Whether to enable code lens.
    #[serde(default = "default_true")]
    pub enable: bool,
}

fn default_true() -> bool {
    true
}