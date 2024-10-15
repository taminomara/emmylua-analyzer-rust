use lua_diagnostic_code::DiagnosticCode;
use lua_diagnostic_severity::DiagnosticSeverity;

mod lua_diagnostic;
mod lua_diagnostic_code;
mod lua_diagnostic_severity;

fn get_default_severity(code: DiagnosticCode) -> DiagnosticSeverity {
    match code {
        DiagnosticCode::SyntaxError => DiagnosticSeverity::Error,
        DiagnosticCode::TypeNotFound => DiagnosticSeverity::Warning,
        DiagnosticCode::MissingReturn => DiagnosticSeverity::Warning,
        DiagnosticCode::TypeNotMatch => DiagnosticSeverity::Warning,
        DiagnosticCode::MissingParameter => DiagnosticSeverity::Warning,
        DiagnosticCode::InjectFieldFail => DiagnosticSeverity::Error,
        DiagnosticCode::UnreachableCode => DiagnosticSeverity::Hint,
        DiagnosticCode::Unused => DiagnosticSeverity::Hint,
        DiagnosticCode::UndefinedGlobal => DiagnosticSeverity::Error,
        DiagnosticCode::NeedImport => DiagnosticSeverity::Warning,
        DiagnosticCode::Deprecated => DiagnosticSeverity::Hint,
        DiagnosticCode::AccessPrivateMember => DiagnosticSeverity::Warning,
        DiagnosticCode::AccessPackageMember => DiagnosticSeverity::Warning,
        DiagnosticCode::AccessProtectedMember => DiagnosticSeverity::Warning,
        DiagnosticCode::NoDiscard => DiagnosticSeverity::Warning,
        DiagnosticCode::DisableGlobalDefine => DiagnosticSeverity::Error,
        DiagnosticCode::UndefinedField => DiagnosticSeverity::Warning,
        DiagnosticCode::LocalConstReassign => DiagnosticSeverity::Error,
        _ => DiagnosticSeverity::Warning,
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
