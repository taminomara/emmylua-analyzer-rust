use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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


pub fn get_default_severity(code: DiagnosticCode) -> DiagnosticSeverity {
    match code {
        DiagnosticCode::SyntaxError => DiagnosticSeverity::ERROR,
        DiagnosticCode::TypeNotFound => DiagnosticSeverity::WARNING,
        DiagnosticCode::MissingReturn => DiagnosticSeverity::WARNING,
        DiagnosticCode::TypeNotMatch => DiagnosticSeverity::WARNING,
        DiagnosticCode::MissingParameter => DiagnosticSeverity::WARNING,
        DiagnosticCode::InjectFieldFail => DiagnosticSeverity::ERROR,
        DiagnosticCode::UnreachableCode => DiagnosticSeverity::HINT,
        DiagnosticCode::Unused => DiagnosticSeverity::HINT,
        DiagnosticCode::UndefinedGlobal => DiagnosticSeverity::ERROR,
        DiagnosticCode::NeedImport => DiagnosticSeverity::WARNING,
        DiagnosticCode::Deprecated => DiagnosticSeverity::HINT,
        DiagnosticCode::AccessPrivateMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::AccessPackageMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::AccessProtectedMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::NoDiscard => DiagnosticSeverity::WARNING,
        DiagnosticCode::DisableGlobalDefine => DiagnosticSeverity::ERROR,
        DiagnosticCode::UndefinedField => DiagnosticSeverity::WARNING,
        DiagnosticCode::LocalConstReassign => DiagnosticSeverity::ERROR,
        _ => DiagnosticSeverity::WARNING,
    }
}

pub fn is_code_default_enable(code: &DiagnosticCode) -> bool {
    match code {
        DiagnosticCode::SyntaxError => true,
        DiagnosticCode::TypeNotFound => false,
        DiagnosticCode::MissingReturn => true,
        DiagnosticCode::TypeNotMatch => true,
        DiagnosticCode::MissingParameter => true,
        DiagnosticCode::InjectFieldFail => false,
        DiagnosticCode::UnreachableCode => true,
        DiagnosticCode::Unused => true,
        DiagnosticCode::UndefinedGlobal => true,
        DiagnosticCode::NeedImport => true,
        DiagnosticCode::Deprecated => true,
        DiagnosticCode::AccessPrivateMember => true,
        DiagnosticCode::AccessProtectedMember => true,
        DiagnosticCode::AccessPackageMember => true,
        DiagnosticCode::NoDiscard => true,
        DiagnosticCode::DisableGlobalDefine => false,
        DiagnosticCode::UndefinedField => false,
        DiagnosticCode::LocalConstReassign => true,
        DiagnosticCode::DuplicateType => true,
        _ => false,
    }
}
