use std::collections::HashMap;

use lsp_types::DiagnosticSeverity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::DiagnosticCode;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcDiagnostic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<Vec<DiagnosticCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals_regex: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<HashMap<DiagnosticCode, DiagnosticSeveritySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
