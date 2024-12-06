use crate::DiagnosticCode;

use super::DiagnosticContext;
use tokio_util::sync::CancellationToken;

pub fn check(context: &mut DiagnosticContext, cancel_token: &CancellationToken) -> Option<()> {
    if cancel_token.is_cancelled() {
        return None;
    }

    let semantic_model = &context.semantic_model;
    if let Some(parse_errors) = semantic_model.get_file_parse_error() {
        for parse_error in parse_errors {
            context.add_diagnostic(
                DiagnosticCode::SyntaxError,
                parse_error.1,
                parse_error.0,
                None,
            );
        }
    }

    Some(())
}
