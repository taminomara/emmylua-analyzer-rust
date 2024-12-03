use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcStrict {
    pub require_path: Option<bool>,
    pub type_call: Option<bool>,
}


