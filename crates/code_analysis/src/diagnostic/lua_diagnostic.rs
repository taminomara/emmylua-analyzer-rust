use std::{collections::HashSet, sync::Arc};

pub use super::checker::{DiagnosticContext, LuaChecker};
use super::{checker::init_checkers, DiagnosticCode};
use crate::{Emmyrc, FileId, LuaCompilation};
use lsp_types::Diagnostic;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct LuaDiagnostic {
    checkers: Vec<Box<dyn LuaChecker>>,
    enable: bool,
    workspace_enabled: Arc<HashSet<DiagnosticCode>>,
    workspace_disabled: Arc<HashSet<DiagnosticCode>>,
}

impl LuaDiagnostic {
    pub fn new() -> Self {
        Self {
            checkers: init_checkers(),
            enable: true,
            workspace_enabled: HashSet::new().into(),
            workspace_disabled: HashSet::new().into(),
        }
    }

    pub fn add_checker(&mut self, checker: Box<dyn LuaChecker>) {
        self.checkers.push(checker);
    }

    pub fn update_config(&mut self, emmyrc: Arc<Emmyrc>) {
        self.enable = emmyrc.diagnostics.enable;
        self.workspace_disabled = Arc::new(emmyrc.diagnostics.disable.iter().cloned().collect());
        self.workspace_enabled = Arc::new(emmyrc.diagnostics.enables.iter().cloned().collect());
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
        let mut context = DiagnosticContext::new(
            model,
            self.workspace_enabled.clone(),
            self.workspace_disabled.clone(),
        );
        for checker in &self.checkers {
            if cancel_token.is_cancelled() {
                return None;
            }

            let codes = checker.support_codes();
            let can_check = codes
                .iter()
                .any(|code| context.is_checker_enable_by_code(code));
            if !can_check {
                continue;
            }

            checker.check(&mut context);
        }

        Some(context.get_diagnostics())
    }
}
