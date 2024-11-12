mod diagnostic_action;
mod analyze_error;

use std::collections::HashMap;

pub use analyze_error::AnalyzeError;
pub use diagnostic_action::DiagnosticAction;

use crate::FileId;

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct DiagnosticIndex {
    diagnostic_actions: HashMap<FileId, Vec<DiagnosticAction>>,
    diagnostics: HashMap<FileId, Vec<AnalyzeError>>
}

impl DiagnosticIndex {
    pub fn new() -> Self {
        Self {
            diagnostic_actions: HashMap::new(),
            diagnostics: HashMap::new()
        }
    }

    pub fn add_diagnostic_action(&mut self, file_id: FileId, diagnostic: DiagnosticAction) {
        self.diagnostic_actions.entry(file_id).or_default().push(diagnostic);
    }

    pub fn get_diagnostics_actions(&self, file_id: FileId) -> Option<&Vec<DiagnosticAction>> {
        self.diagnostic_actions.get(&file_id)
    }

    pub fn add_diagnostic(&mut self, file_id: FileId, diagnostic: AnalyzeError) {
        self.diagnostics.entry(file_id).or_default().push(diagnostic);
    }

    pub fn get_diagnostics(&self, file_id: FileId) -> Option<&Vec<AnalyzeError>> {
        self.diagnostics.get(&file_id)
    }
}

impl LuaIndex for DiagnosticIndex {
    fn remove(&mut self, file_id: FileId) {
        self.diagnostic_actions.remove(&file_id);
    }
}
