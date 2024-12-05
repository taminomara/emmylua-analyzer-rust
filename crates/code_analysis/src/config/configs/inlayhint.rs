use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcInlayHint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_hint: Option<bool>,
}
