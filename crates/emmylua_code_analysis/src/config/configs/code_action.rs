use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcCodeAction {
    /// Whether to insert space after '---'
    #[serde(default = "default_false")]
    pub insert_space: bool,
}

impl Default for EmmyrcCodeAction {
    fn default() -> Self {
        Self {
            insert_space: default_false(),
        }
    }
}

fn default_false() -> bool {
    false
}
