use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcSemanticToken {
    /// Whether to enable semantic token.
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for EmmyrcSemanticToken {
    fn default() -> Self {
        Self {
            enable: default_true(),
        }
    }
}

fn default_true() -> bool {
    true
}
