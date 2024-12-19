use std::{collections::HashMap, sync::Arc, time::Duration};

use code_analysis::{EmmyLuaAnalysis, FileId};
use log::{debug, info};
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use super::{status_bar::VsCodeStatusBar, ClientProxy};

pub struct FileDiagnostic {
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client: Arc<ClientProxy>,
    #[allow(unused)]
    status_bar: Arc<VsCodeStatusBar>,
    diagnostic_tokens: Arc<Mutex<HashMap<FileId, CancellationToken>>>,
}

impl FileDiagnostic {
    pub fn new(
        analysis: Arc<RwLock<EmmyLuaAnalysis>>,
        client: Arc<ClientProxy>,
        status_bar: Arc<VsCodeStatusBar>,
    ) -> Self {
        Self {
            analysis,
            client,
            status_bar,
            diagnostic_tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[allow(unused)]
    pub async fn add_diagnostic_task(&self, file_id: FileId) {
        let mut tokens = self.diagnostic_tokens.lock().await;

        if let Some(token) = tokens.get(&file_id) {
            token.cancel();
            debug!("cancel diagnostic: {:?}", file_id);
        }

        // create new token
        let cancel_token = CancellationToken::new();
        tokens.insert(file_id.clone(), cancel_token.clone());
        drop(tokens); // free the lock

        let analysis = self.analysis.clone();
        let client = self.client.clone();
        let diagnostic_tokens = self.diagnostic_tokens.clone();
        let file_id_clone = file_id.clone();

        // Spawn a new task to perform diagnostic
        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(500)) => {
                    let analysis = analysis.read().await;
                    if let Some(uri) = analysis.get_uri(file_id_clone) {
                        let diagnostics = analysis.diagnose_file(file_id_clone, cancel_token).await;
                        if let Some(diagnostics) = diagnostics {
                            let diagnostic_param = lsp_types::PublishDiagnosticsParams {
                                uri,
                                diagnostics,
                                version: None,
                            };
                            client.publish_diagnostics(diagnostic_param);
                        }
                    } else {
                        info!("file not found: {:?}", file_id_clone);
                    }
                    // After completion, remove from HashMap
                    let mut tokens = diagnostic_tokens.lock().await;
                    tokens.remove(&file_id_clone);
                }
                _ = cancel_token.cancelled() => {
                    debug!("cancel diagnostic: {:?}", file_id_clone);
                }
            }
        });
    }

    // todo add message show
    pub async fn add_files_diagnostic_task(&self, file_ids: Vec<FileId>) {
        for file_id in file_ids {
            self.add_diagnostic_task(file_id).await;
        }
    }

    #[allow(unused)]
    pub async fn cancel_all(&self) {
        let mut tokens = self.diagnostic_tokens.lock().await;
        for (_, token) in tokens.iter() {
            token.cancel();
        }
        tokens.clear();
    }
}
