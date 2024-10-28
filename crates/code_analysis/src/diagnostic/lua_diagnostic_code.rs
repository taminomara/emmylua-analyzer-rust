use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticCode {
    None,
    SyntaxError,
    TypeNotFound,
    MissingReturn,
    TypeNotMatch,
    MissingParameter,
    InjectFieldFail,
    UnreachableCode,
    Unused,
    UndefinedGlobal,
    NeedImport,
    Deprecated,
    AccessPrivateMember,
    AccessProtectedMember,
    AccessPackageMember,
    NoDiscard,
    DisableGlobalDefine,
    UndefinedField,
    LocalConstReassign,
    DuplicateType,
}

impl DiagnosticCode {
    pub fn get_name(&self) -> String {
        serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }

    pub fn get_code(name: &str) -> DiagnosticCode {
        match serde_json::from_str(&format!("\"{}\"", name)) {
            Ok(code) => code,
            Err(_) => DiagnosticCode::None,
        }
    }
}
