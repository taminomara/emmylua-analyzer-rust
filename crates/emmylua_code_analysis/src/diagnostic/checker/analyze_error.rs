use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[
    DiagnosticCode::TypeNotFound,
    DiagnosticCode::AnnotationUsageError,
];

pub fn check(context: &mut DiagnosticContext, _: &SemanticModel) -> Option<()> {
    let db = context.get_db();
    let file_id = context.get_file_id();
    let diagnostic_index = db.get_diagnostic_index();
    let errors: Vec<_> = diagnostic_index
        .get_diagnostics(&file_id)?
        .iter()
        .map(|e| e.clone())
        .collect();
    for error in errors {
        context.add_diagnostic(error.kind, error.range, error.message, None);
    }

    Some(())
}
