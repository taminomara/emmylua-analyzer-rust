use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcStrict {
    /// Whether to enable strict mode require path.
    #[serde(default)]
    pub require_path: bool,
    #[serde(default)]
    pub type_call: bool,
    /// Whether to enable strict mode array indexing.
    #[serde(default = "default_false")]
    pub array_index: bool,
    /// meta define overrides file define
    #[serde(default = "default_true")]
    pub meta_override_file_define: bool,
}

impl Default for EmmyrcStrict {
    fn default() -> Self {
        Self {
            require_path: false,
            type_call: false,
            array_index: false,
            meta_override_file_define: true,
        }
    }
}
