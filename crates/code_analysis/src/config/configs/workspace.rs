use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcWorkspace {
    pub ignore_dir: Option<Vec<String>>,
    pub ignore_globs: Option<Vec<String>>,
    pub library: Option<Vec<String>>,
    pub workspace_roots: Option<Vec<String>>,
    // unused
    pub preload_file_size: Option<i32>,
    pub encoding: Option<String>,
}
