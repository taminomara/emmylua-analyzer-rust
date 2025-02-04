mod json_output_writter;
mod text_output_writter;

use std::path::PathBuf;

use emmylua_code_analysis::{DbIndex, FileId};
use lsp_types::Diagnostic;
use tokio::sync::mpsc::Receiver;

use crate::cmd_args::{OutputDestination, OutputFormat};

pub async fn output_result(
    total_count: usize,
    db: &DbIndex,
    workspace: PathBuf,
    mut receiver: Receiver<(FileId, Option<Vec<Diagnostic>>)>,
    output_format: OutputFormat,
    output: OutputDestination,
    warnings_as_errors: bool,
) -> i32 {
    let mut writter: Box<dyn OutputWritter> = match output_format {
        OutputFormat::Json => Box::new(json_output_writter::JsonOutputWritter::new(output)),
        OutputFormat::Text => Box::new(text_output_writter::TextOutputWritter::new(workspace)),
    };

    let mut has_error = false;
    let mut count = 0;
    while let Some((file_id, diagnostics)) = receiver.recv().await {
        count += 1;
        if let Some(diagnostics) = diagnostics {
            for diagnostic in &diagnostics {
                if diagnostic.severity == Some(lsp_types::DiagnosticSeverity::ERROR) {
                    has_error = true;
                    break;
                } else if warnings_as_errors
                    && diagnostic.severity == Some(lsp_types::DiagnosticSeverity::WARNING)
                {
                    has_error = true;
                    break;
                }
            }
            writter.write(db, file_id, diagnostics);
        }

        if count == total_count {
            break;
        }
    }

    writter.finish();

    if has_error {
        1
    } else {
        0
    }
}

trait OutputWritter {
    fn write(&mut self, db: &DbIndex, file_id: FileId, diagnostics: Vec<Diagnostic>);

    fn finish(&mut self);
}
