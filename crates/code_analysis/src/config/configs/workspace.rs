use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcWorkspace {
    #[serde(default)]
    pub ignore_dir: Vec<String>,
    #[serde(default)]
    pub ignore_globs: Vec<String>,
    #[serde(default)]
    pub library: Vec<String>,
    #[serde(default)]
    pub workspace_roots: Vec<String>,
    // unused
    #[serde(default)]
    pub preload_file_size: i32,
    #[serde(default = "encoding_default")]
    pub encoding: String,
}

fn encoding_default() -> String {
    "utf-8".to_string()
}
