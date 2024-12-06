use std::sync::Arc;

use lsp_types::Diagnostic;
use tokio_util::sync::CancellationToken;

use crate::{Emmyrc, FileId, LuaCompilation};

use super::checker::{check, DiagnosticContext};

#[derive(Debug)]
pub struct LuaDiagnostic {
    emmyrc: Arc<Emmyrc>,
}

impl LuaDiagnostic {
    pub fn new(emmyrc: Arc<Emmyrc>) -> Self {
        Self {
            emmyrc,
            // checker: Vec::new(),
        }
    }

    // pub fn add_checker(&mut self, checker: Box<dyn LuaChecker>) {
    //     self.checker.push(checker);
    // }

    pub async fn diagnose_file(
        &self,
        compilation: &LuaCompilation,
        file_id: FileId,
        cancel_token: CancellationToken,
    ) -> Option<Vec<Diagnostic>>{
        let model = compilation.get_semantic_model(file_id)?;
        let mut context = DiagnosticContext::new(model);       
        check(&mut context, cancel_token);        

        Some(context.get_diagnostics())
    }
}
