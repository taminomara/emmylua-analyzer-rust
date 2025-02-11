use emmylua_codestyle::check_code_style;
use rowan::TextRange;

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::CodeStyleCheck];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let document = semantic_model.get_document();
    let file_path = document.get_file_path();
    let text = document.get_text();
    let result = check_code_style(&file_path.to_string_lossy().to_string(), text);
    for diagnostic in result {
        let start = document.get_offset(
            diagnostic.start_line as usize,
            diagnostic.start_col as usize,
        )?;
        let end = document.get_offset(diagnostic.end_line as usize, diagnostic.end_col as usize)?;
        let text_range = TextRange::new(start, end);
        context.add_diagnostic(
            DiagnosticCode::CodeStyleCheck,
            text_range,
            diagnostic.message,
            None,
        );
    }

    Some(())
}
