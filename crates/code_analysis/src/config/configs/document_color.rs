use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcDocumentColor {
    /// Whether to enable document color.
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for EmmyrcDocumentColor {
    fn default() -> Self {
        Self {
            enable: default_true(),
        }
    }
}

fn default_true() -> bool {
    true
}
