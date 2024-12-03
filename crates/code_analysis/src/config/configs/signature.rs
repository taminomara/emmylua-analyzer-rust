use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcSignature {
    pub detail_signature_helper: Option<bool>,
}