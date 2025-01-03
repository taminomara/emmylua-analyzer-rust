mod syntax_error;
mod type_not_found;
mod duplicate_type;

use lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, NumberOrString};
use rowan::TextRange;
use std::fmt::Debug;

use crate::{db_index::DbIndex, semantic::SemanticModel, FileId};

use super::{lua_diagnostic_code::get_default_severity, DiagnosticCode};

pub trait LuaChecker: Debug + Send + Sync {
    fn check(&self, context: &mut DiagnosticContext) -> Option<()>;

    fn get_code(&self) -> DiagnosticCode;
}

macro_rules! checker {
    ($name:ident) => {
        Box::new($name::Checker())
    };
}

pub fn init_checkers() -> Vec<Box<dyn LuaChecker>> {
    vec![
        checker!(syntax_error),
        checker!(type_not_found),
        checker!(duplicate_type),
    ]
}

pub struct DiagnosticContext<'a> {
    semantic_model: SemanticModel<'a>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> DiagnosticContext<'a> {
    pub fn new(semantic_model: SemanticModel<'a>) -> Self {
        Self {
            semantic_model,
            diagnostics: Vec::new(),
        }
    }

    pub fn get_db(&self) -> &DbIndex {
        &self.semantic_model.get_db()
    }

    pub fn get_file_id(&self) -> FileId {
        self.semantic_model.get_file_id()
    }

    pub fn add_diagnostic(
        &mut self,
        code: DiagnosticCode,
        range: TextRange,
        message: String,
        data: Option<serde_json::Value>,
    ) {
        if !self.should_report_diagnostic(&code, &range) {
            return;
        }

        let diagnostic = Diagnostic {
            message,
            range: self.translate_range(range).unwrap_or(lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
            }),
            severity: self.get_severity(code),
            code: Some(NumberOrString::String(code.get_name().to_string())),
            source: Some("EmmyLua".into()),
            tags: self.get_tags(code),
            data,
            ..Default::default()
        };

        self.diagnostics.push(diagnostic);
    }

    fn should_report_diagnostic(&self, code: &DiagnosticCode, range: &TextRange) -> bool {
        let diagnostic_index = self.get_db().get_diagnostic_index();

        !diagnostic_index.is_file_diagnostic_code_disabled(&self.get_file_id(), code, range)
    }

    fn get_severity(&self, code: DiagnosticCode) -> Option<DiagnosticSeverity> {
        Some(get_default_severity(code))
    }

    fn get_tags(&self, code: DiagnosticCode) -> Option<Vec<DiagnosticTag>> {
        match code {
            DiagnosticCode::Unused | DiagnosticCode::UnreachableCode => {
                Some(vec![DiagnosticTag::UNNECESSARY])
            }
            DiagnosticCode::Deprecated => Some(vec![DiagnosticTag::DEPRECATED]),
            _ => None,
        }
    }

    fn translate_range(&self, range: TextRange) -> Option<lsp_types::Range> {
        let document = self.semantic_model.get_document();
        let (start_line, start_character) = document.get_line_col(range.start())?;
        let (end_line, end_character) = document.get_line_col(range.end())?;

        Some(lsp_types::Range {
            start: lsp_types::Position {
                line: start_line as u32,
                character: start_character as u32,
            },
            end: lsp_types::Position {
                line: end_line as u32,
                character: end_character as u32,
            },
        })
    }

    pub fn get_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}
