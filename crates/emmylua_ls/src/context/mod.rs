mod snapshot;

use std::{collections::HashMap, sync::Arc};
use code_analysis::EmmyLuaAnalysis;
use lsp_server::{Connection, ErrorCode, Message, RequestId, Response};
use lsp_types::InitializeParams;
pub use snapshot::ServerContextSnapshot;
use tokio_util::sync::CancellationToken;
use tokio::sync::{Mutex, RwLock};

pub struct ServerContext {
    conn: Connection,
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    cancllations: Arc<Mutex<HashMap<RequestId, CancellationToken>>>,
}

impl ServerContext {
    pub fn new(conn: Connection) -> Self {
        ServerContext {
            conn,
            analysis: Arc::new(RwLock::new(EmmyLuaAnalysis::new())),
            cancllations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn snapshot(&self) -> ServerContextSnapshot {
        ServerContextSnapshot {
            analysis: self.analysis.clone(),
        }
    }

    pub async fn task<F>(&self, req_id: RequestId, exec: F)
    where
        F: FnOnce(CancellationToken) -> Option<Response> + Send + 'static,
    {
        let cancel_token = CancellationToken::new();

        {
            let mut cancellations = self.cancllations.lock().await;
            cancellations.insert(req_id.clone(), cancel_token.clone());
        }

        let sender = self.conn.sender.clone();
        let cancellations = self.cancllations.clone();

        tokio::spawn(async move {
            let res = exec(cancel_token.clone());
            if cancel_token.is_cancelled() || res.is_none() {
                let response = Response::new_err(
                    req_id.clone(),
                    ErrorCode::RequestCanceled as i32,
                    "cancel".to_string(),
                );
                let _ = sender.send(Message::Response(response));
            } else if let Some(it) = res {
                let _ = sender.send(Message::Response(it));
            }

            let mut cancellations = cancellations.lock().await;
            cancellations.remove(&req_id);
        });
    }

    pub async fn cancel(&self, req_id: RequestId) {
        let cancellations = self.cancllations.lock().await;
        if let Some(cancel_token) = cancellations.get(&req_id) {
            cancel_token.cancel();
        }
    }
}
