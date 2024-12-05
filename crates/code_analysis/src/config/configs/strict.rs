use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcStrict {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_call: Option<bool>,
}
