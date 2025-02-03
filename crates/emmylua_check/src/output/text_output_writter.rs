use std::{fs::File, io::Write};

use emmylua_code_analysis::{DbIndex, FileId};
use lsp_types::Diagnostic;

use crate::cmd_args::OutputDestination;

use super::OutputWritter;

#[derive(Debug)]
pub struct TextOutputWritter {
    output: Option<File>,
}

impl TextOutputWritter {
    pub fn new(output: OutputDestination) -> Self {
        let output = match output {
            OutputDestination::Stdout => None,
            OutputDestination::File(path) => {
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).unwrap();
                    }
                }

                Some(std::fs::File::create(path).unwrap())
            }
        };

        TextOutputWritter { output }
    }
}

impl OutputWritter for TextOutputWritter {
    fn write(&mut self, db: &DbIndex, file_id: FileId, diagnostics: Vec<Diagnostic>) {
        let file_path = db.get_vfs().get_file_path(&file_id).unwrap();
        let file_path = file_path.to_str().unwrap();
        let mut out_string = String::new();
        out_string.push_str(format!("file: {} ", file_path).as_str());
        let mut error_count = 0;
        let mut warning_count = 0;
        for diagnostic in &diagnostics {
            match diagnostic.severity {
                Some(severity) => match severity {
                    lsp_types::DiagnosticSeverity::ERROR => {
                        error_count += 1;
                    }
                    lsp_types::DiagnosticSeverity::WARNING
                    | lsp_types::DiagnosticSeverity::HINT
                    | lsp_types::DiagnosticSeverity::INFORMATION => {
                        warning_count += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if error_count > 0 {
            out_string.push_str(format!("{} error", error_count).as_str());
        }
        if warning_count > 0 {
            out_string.push_str(format!("{} warning", warning_count).as_str());
        }

        out_string.push('\n');
        for diagnostic in diagnostics {
            out_string.push_str(format!("line: {} \n", diagnostic.range.start.line).as_str());
            out_string
                .push_str(format!("column: {} \n", diagnostic.range.start.character).as_str());
            if let Some(serverity) = diagnostic.severity {
                out_string.push_str(format!("severity: {:?} \n", serverity).as_str());
            }
            if let Some(code) = diagnostic.code {
                match code {
                    lsp_types::NumberOrString::Number(code) => {
                        out_string.push_str(format!("code: {} \n", code).as_str());
                    }
                    lsp_types::NumberOrString::String(code) => {
                        out_string.push_str(format!("code: {} \n", code).as_str());
                    }
                }
            }
            out_string.push_str(format!("message: {} \n", diagnostic.message).as_str());
            out_string.push('\n');
        }

        if let Some(output) = self.output.as_mut() {
            output.write_all(out_string.as_bytes()).unwrap();
        } else {
            println!("{}", out_string);
        }
    }

    fn finish(&mut self) {}
}
