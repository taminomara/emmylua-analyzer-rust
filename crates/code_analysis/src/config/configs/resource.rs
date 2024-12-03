use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcResource {
    pub paths: Option<Vec<String>>,
}
