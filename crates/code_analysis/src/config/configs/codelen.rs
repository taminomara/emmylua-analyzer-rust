use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcCodeLen {
    pub enable: Option<bool>,
}
