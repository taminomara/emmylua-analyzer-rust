use std::{collections::HashSet, sync::Arc};

pub use super::checker::DiagnosticContext;
use super::{checker::check_file, DiagnosticCode};
use crate::{Emmyrc, FileId, LuaCompilation};
use lsp_types::Diagnostic;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct LuaDiagnostic {
    enable: bool,
    workspace_enabled: Arc<HashSet<DiagnosticCode>>,
    workspace_disabled: Arc<HashSet<DiagnosticCode>>,
}

impl LuaDiagnostic {
    pub fn new() -> Self {
        Self {
            enable: true,
            workspace_enabled: HashSet::new().into(),
            workspace_disabled: HashSet::new().into(),
        }
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

        if cancel_token.is_cancelled() {
            return None;
        }

        let db = compilation.get_db();
        let mut semantic_model = compilation.get_semantic_model(file_id)?;
        let mut context = DiagnosticContext::new(
            file_id,
            db,
            self.workspace_enabled.clone(),
            self.workspace_disabled.clone(),
        );

        check_file(&mut context, &mut semantic_model);

        Some(context.get_diagnostics())
    }
}
