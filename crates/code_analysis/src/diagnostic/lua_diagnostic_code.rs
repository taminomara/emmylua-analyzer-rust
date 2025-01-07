use diagnostic_macro::LuaDiagnosticMacro;
use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, LuaDiagnosticMacro,
)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticCode {
    None,
    /// Syntax error
    SyntaxError,
    /// Type not found
    TypeNotFound,
    /// Missing return statement
    MissingReturn,
    /// Type not match
    TypeNotMatch,
    /// Missing parameter
    MissingParameter,
    /// Inject field fail
    InjectFieldFail,
    /// Unreachable code
    UnreachableCode,
    /// Unused
    Unused,
    /// Undefined global
    UndefinedGlobal,
    /// Need import
    NeedImport,
    /// Deprecated
    Deprecated,
    /// Access invisible member
    AccessInvisibleMember,
    /// Discard return value
    DiscardReturns,
    /// Disable global define
    DisableGlobalDefine,
    /// Undefined field
    UndefinedField,
    /// Local const reassign
    LocalConstReassign,
    /// Duplicate type
    DuplicateType,
    /// Redefined local
    RedefinedLocal,
    /// Redefined label
    RedefinedLabel,
    /// Name Style check
    NameStyleCheck,
    /// Code style check
    CodeStyleCheck,
    /// Need check nil
    NeedCheckNil,
    /// Await in sync
    AwaitInSync,
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
        DiagnosticCode::AccessInvisibleMember => DiagnosticSeverity::WARNING,
        DiagnosticCode::DiscardReturns => DiagnosticSeverity::WARNING,
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
