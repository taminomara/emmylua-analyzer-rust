use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcStrict {
    /// Whether to enable strict mode require path.
    #[serde(default = "default_true")]
    pub require_path: bool,
    #[serde(default)]
    pub type_call: bool,
}

impl Default for EmmyrcStrict {
    fn default() -> Self {
        Self {
            require_path: default_true(),
            type_call: false,
        }
    }
}

fn default_true() -> bool {
    true
}