use crate::DiagnosticCode;

use super::{DiagnosticContext, LuaChecker};

#[derive(Debug)]
pub struct SyntaxErrorChecker();

impl LuaChecker for SyntaxErrorChecker {
    fn check(&self, context: &mut DiagnosticContext) -> Option<()> {
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

    fn get_code(&self) -> DiagnosticCode {
        DiagnosticCode::SyntaxError
    }
}
