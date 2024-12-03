use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcSemanticToken {
    pub enable: Option<bool>,
}
