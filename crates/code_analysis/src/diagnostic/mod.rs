mod lua_diagnostic;
mod lua_diagnostic_code;

use lsp_types::DiagnosticSeverity;
pub use lua_diagnostic_code::DiagnosticCode;

fn get_default_severity(code: DiagnosticCode) -> DiagnosticSeverity {
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

fn is_code_default_enable(code: &DiagnosticCode) -> bool {
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
