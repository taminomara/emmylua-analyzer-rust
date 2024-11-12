use rowan::TextRange;

use crate::DiagnosticCode;

#[derive(Debug)]
pub struct DiagnosticAction {
    range: TextRange,
    kind: DiagnosticActionKind,
    code: DiagnosticCode
}

impl DiagnosticAction {
    pub fn new(range: TextRange, kind: DiagnosticActionKind, code: DiagnosticCode) -> Self {
        Self {
            range,
            kind,
            code
        }
    }
}


#[derive(Debug)]
pub enum DiagnosticActionKind {
    Disable,
    Enable,
}