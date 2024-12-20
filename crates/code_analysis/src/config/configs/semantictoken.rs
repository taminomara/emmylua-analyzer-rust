use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcSemanticToken {
    /// Whether to enable semantic token.
    #[serde(default = "default_true")]
    pub enable: bool,
}

fn default_true() -> bool {
    true
}
