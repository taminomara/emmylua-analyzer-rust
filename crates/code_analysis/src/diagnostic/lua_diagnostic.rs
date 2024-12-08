use std::{collections::HashSet, sync::Arc};

pub use super::checker::{DiagnosticContext, LuaChecker};
use super::{checker::init_checkers, lua_diagnostic_code::is_code_default_enable, DiagnosticCode};
use crate::{Emmyrc, FileId, LuaCompilation};
use lsp_types::Diagnostic;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct LuaDiagnostic {
    checkers: Vec<Box<dyn LuaChecker>>,
    enable: bool,
    workspace_enabled: HashSet<DiagnosticCode>,
    workspace_disabled: HashSet<DiagnosticCode>,
}

impl LuaDiagnostic {
    pub fn new() -> Self {
        Self {
            checkers: init_checkers(),
            enable: true,
            workspace_enabled: HashSet::new(),
            workspace_disabled: HashSet::new(),
        }
    }

    pub fn add_checker(&mut self, checker: Box<dyn LuaChecker>) {
        self.checkers.push(checker);
    }

    pub fn update_config(&mut self, emmyrc: Arc<Emmyrc>) {
        if let Some(diagnostic) = &emmyrc.diagnostics {
            if let Some(enable) = &diagnostic.enable {
                self.enable = *enable;
            }

            if let Some(disable) = &diagnostic.disable {
                self.workspace_disabled = disable.iter().cloned().collect();
            }

            if let Some(enable) = &diagnostic.enables {
                self.workspace_enabled = enable.iter().cloned().collect();
            }
        }
    }

    pub async fn diagnose_file(
        &self,
        compilation: &LuaCompilation,
        file_id: FileId,
        cancel_token: CancellationToken,
    ) -> Option<Vec<Diagnostic>> {
        if !self.enable {
            return None;
        }

        let model = compilation.get_semantic_model(file_id)?;
        let mut context = DiagnosticContext::new(model);
        for checker in &self.checkers {
            if cancel_token.is_cancelled() {
                return None;
            }

            let code = checker.get_code();
            if self.is_checker_enable_by_code(&context, &code) {
                checker.check(&mut context);
            }
        }

        Some(context.get_diagnostics())
    }

    fn is_checker_enable_by_code(
        &self,
        context: &DiagnosticContext,
        code: &DiagnosticCode,
    ) -> bool {
        let file_id = context.get_file_id();
        let db = context.get_db();
        let diagnostic_index = db.get_diagnostic_index();
        // force enable
        if diagnostic_index.is_file_enabled(&file_id, &code) {
            return true;
        }

        // workspace force disabled
        if self.workspace_disabled.contains(&code) {
            return false;
        }

        let meta_index = db.get_meta_file();
        // ignore meta file diagnostic
        if meta_index.is_meta_file(&file_id) {
            return false;
        }

        // is file disabled this code
        if diagnostic_index.is_file_disabled(&file_id, &code) {
            return false;
        }

        // workspace force enabled
        if self.workspace_enabled.contains(&code) {
            return true;
        }

        // default setting
        is_code_default_enable(&code)
    }
}
