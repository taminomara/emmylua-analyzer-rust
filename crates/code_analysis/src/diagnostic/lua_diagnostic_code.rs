use std::{fmt, str::FromStr};

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
    // ... other variants
}

impl DiagnosticCode {
    pub fn get_name(&self) -> &str {
        match self {
            DiagnosticCode::None => "none",
            DiagnosticCode::SyntaxError => "syntax-error",
            DiagnosticCode::TypeNotFound => "type-not-found",
            DiagnosticCode::MissingReturn => "missing-return",
            DiagnosticCode::TypeNotMatch => "type-not-match",
            DiagnosticCode::MissingParameter => "missing-parameter",
            DiagnosticCode::InjectFieldFail => "inject-field-fail",
            DiagnosticCode::UnreachableCode => "unreachable-code",
            DiagnosticCode::Unused => "unused",
            DiagnosticCode::UndefinedGlobal => "undefined-global",
            DiagnosticCode::NeedImport => "need-import",
            DiagnosticCode::Deprecated => "deprecated",
            DiagnosticCode::AccessPrivateMember => "access-private-member",
            DiagnosticCode::AccessProtectedMember => "access-protected-member",
            DiagnosticCode::AccessPackageMember => "access-package-member",
            DiagnosticCode::NoDiscard => "no-discard",
            DiagnosticCode::DisableGlobalDefine => "disable-global-define",
            DiagnosticCode::UndefinedField => "undefined-field",
            DiagnosticCode::LocalConstReassign => "local-const-reassign",
            DiagnosticCode::DuplicateType => "duplicate-type",
            // ... handle other variants
        }
    }
}

// Implement FromStr for DiagnosticCode
impl FromStr for DiagnosticCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "syntax-error" => Ok(DiagnosticCode::SyntaxError),
            "type-not-found" => Ok(DiagnosticCode::TypeNotFound),
            "missing-return" => Ok(DiagnosticCode::MissingReturn),
            "type-not-match" => Ok(DiagnosticCode::TypeNotMatch),
            "missing-parameter" => Ok(DiagnosticCode::MissingParameter),
            "inject-field-fail" => Ok(DiagnosticCode::InjectFieldFail),
            "unreachable-code" => Ok(DiagnosticCode::UnreachableCode),
            "unused" => Ok(DiagnosticCode::Unused),
            "undefined-global" => Ok(DiagnosticCode::UndefinedGlobal),
            "need-import" => Ok(DiagnosticCode::NeedImport),
            "deprecated" => Ok(DiagnosticCode::Deprecated),
            "access-private-member" => Ok(DiagnosticCode::AccessPrivateMember),
            "access-protected-member" => Ok(DiagnosticCode::AccessProtectedMember),
            "access-package-member" => Ok(DiagnosticCode::AccessPackageMember),
            "no-discard" => Ok(DiagnosticCode::NoDiscard),
            "disable-global-define" => Ok(DiagnosticCode::DisableGlobalDefine),
            "undefined-field" => Ok(DiagnosticCode::UndefinedField),
            "local-const-reassign" => Ok(DiagnosticCode::LocalConstReassign),
            "duplicate-type" => Ok(DiagnosticCode::DuplicateType),
            // ... handle other variants
            _ => Ok(DiagnosticCode::None),
        }
    }
}

// Implement Display for DiagnosticCode
impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
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
