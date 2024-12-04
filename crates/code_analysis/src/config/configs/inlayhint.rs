use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcInlayHint {
    pub param_hint: Option<bool>,
    pub index_hint: Option<bool>,
    pub local_hint: Option<bool>,
    pub override_hint: Option<bool>,
}
