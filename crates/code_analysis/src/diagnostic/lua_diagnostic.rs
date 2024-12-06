use std::{collections::HashSet, sync::Arc};

pub use super::checker::{DiagnosticContext, LuaChecker};
use super::DiagnosticCode;
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
            checkers: Vec::new(),
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
                if !enable {
                    return false;
                }
            }

            if let Some(disable) = &diagnostic.disable {
                if disable.contains(&code) {
                    return false;
                }
            }
        }
    }

    pub async fn diagnose_file(
        &self,
        compilation: &LuaCompilation,
        file_id: FileId,
        cancel_token: CancellationToken,
    ) -> Option<Vec<Diagnostic>> {
        let model = compilation.get_semantic_model(file_id)?;
        let mut context = DiagnosticContext::new(model);
        for checker in &self.checkers {
            if cancel_token.is_cancelled() {
                return None;
            }

            if self.can_run(&context, checker) {
                checker.check(&mut context);
            }
        }

        Some(context.get_diagnostics())
    }

    fn can_run(&self, context: &DiagnosticContext, checker: &Box<dyn LuaChecker>) -> bool {
        let code = checker.get_code();

        false
    }
}
