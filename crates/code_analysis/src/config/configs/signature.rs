use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcSignature {
    /// Whether to enable signature help.
    #[serde(default = "default_true")]
    pub detail_signature_helper: bool,
}

fn default_true() -> bool {
    true
}

