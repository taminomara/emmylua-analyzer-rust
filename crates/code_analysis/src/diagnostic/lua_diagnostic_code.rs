use diagnostic_macro::LuaDiagnosticMacro;
use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, LuaDiagnosticMacro)]
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
    // ... other variants
}

// Update functions to match enum variants
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
        DiagnosticCode::AccessProtectedMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::AccessPackageMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::NoDiscard => DiagnosticSeverity::WARNING,
        DiagnosticCode::DisableGlobalDefine => DiagnosticSeverity::ERROR,
        DiagnosticCode::UndefinedField => DiagnosticSeverity::WARNING,
        DiagnosticCode::LocalConstReassign => DiagnosticSeverity::ERROR,
        DiagnosticCode::DuplicateType => DiagnosticSeverity::WARNING,
        _ => DiagnosticSeverity::WARNING,
    }
}

pub fn is_code_default_enable(code: &DiagnosticCode) -> bool {
    match code {
        DiagnosticCode::TypeNotFound => false,
        DiagnosticCode::InjectFieldFail => false,
        DiagnosticCode::DisableGlobalDefine => false,
        DiagnosticCode::UndefinedField => false,

        // ... handle other variants
        _ => true,
    }
}
