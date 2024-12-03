use std::collections::HashMap;

use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::DiagnosticCode;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmmyrcDiagnostics {
    pub disable: Option<Vec<DiagnosticCode>>,
    pub enable: Option<bool>,
    pub globals: Option<Vec<String>>,
    pub globals_regex: Option<Vec<String>>,
    pub severity: Option<HashMap<DiagnosticCode, DiagnosticSeveritySetting>>,
    pub enables: Option<Vec<DiagnosticCode>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum DiagnosticSeveritySetting {
    Error,
    Warning,
    Information,
    Hint,
}

impl From<DiagnosticSeveritySetting> for DiagnosticSeverity {
    fn from(severity: DiagnosticSeveritySetting) -> Self {
        match severity {
            DiagnosticSeveritySetting::Error => DiagnosticSeverity::ERROR,
            DiagnosticSeveritySetting::Warning => DiagnosticSeverity::WARNING,
            DiagnosticSeveritySetting::Information => DiagnosticSeverity::INFORMATION,
            DiagnosticSeveritySetting::Hint => DiagnosticSeverity::HINT,
        }
    }
}
