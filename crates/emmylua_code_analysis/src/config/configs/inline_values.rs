use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcInlineValues {
    /// Whether to enable inline values.
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for EmmyrcInlineValues {
    fn default() -> Self {
        Self {
            enable: default_true(),
        }
    }
}

fn default_true() -> bool {
    true
}
