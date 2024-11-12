use rowan::TextRange;

use crate::DiagnosticCode;

#[derive(Debug)]
pub struct AnalyzeError {
    pub kind: DiagnosticCode,
    pub message: String,
    pub range: TextRange
}

impl AnalyzeError {
    pub fn new(kind: DiagnosticCode, message: String, range: TextRange) -> Self {
        Self {
            kind,
            message,
            range
        }
    }
}