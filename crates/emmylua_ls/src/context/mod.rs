mod client;
mod snapshot;

use client::ClientProxy;
use code_analysis::EmmyLuaAnalysis;
use lsp_server::{Connection, ErrorCode, Message, RequestId, Response};
pub use snapshot::ServerContextSnapshot;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

pub struct ServerContext {
    conn: Connection,
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client: Arc<ClientProxy>,
    cancllations: Arc<Mutex<HashMap<RequestId, CancellationToken>>>,
}

impl ServerContext {
    pub fn new(conn: Connection) -> Self {
        let client = ClientProxy::new(Connection {
            sender: conn.sender.clone(),
            receiver: conn.receiver.clone(),
        });

        let mut analysis = EmmyLuaAnalysis::new();
        analysis.init_std_lib();
        ServerContext {
            conn,
            analysis: Arc::new(RwLock::new(analysis)),
            client: Arc::new(client),
            cancllations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn snapshot(&self) -> ServerContextSnapshot {
        ServerContextSnapshot {
            analysis: self.analysis.clone(),
            client: self.client.clone(),
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

    pub async fn send_response(&self, response: Response) {
        self.client.on_response(response).await;
    }
}
