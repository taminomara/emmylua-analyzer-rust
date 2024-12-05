use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcWorkspace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_dir: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_globs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_roots: Option<Vec<String>>,
    // unused
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_file_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}
