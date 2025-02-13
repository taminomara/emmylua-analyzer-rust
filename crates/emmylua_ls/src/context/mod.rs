mod client;
mod client_id;
mod config_manager;
mod file_diagnostic;
mod snapshot;
mod status_bar;

pub use client::ClientProxy;
pub use client_id::{get_client_id, ClientId};
use emmylua_code_analysis::EmmyLuaAnalysis;
pub use config_manager::load_emmy_config;
pub use config_manager::ConfigManager;
pub use file_diagnostic::FileDiagnostic;
use lsp_server::{Connection, ErrorCode, Message, RequestId, Response};
pub use snapshot::ServerContextSnapshot;
pub use status_bar::StatusBar;
pub use status_bar::ProgressTask;
use std::{collections::HashMap, future::Future, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

pub struct ServerContext {
    #[allow(unused)]
    conn: Connection,
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client: Arc<ClientProxy>,
    cancllations: Arc<Mutex<HashMap<RequestId, CancellationToken>>>,
    file_diagnostic: Arc<FileDiagnostic>,
    config_manager: Arc<RwLock<ConfigManager>>,
    status_bar: Arc<StatusBar>,
}

impl ServerContext {
    pub fn new(conn: Connection) -> Self {
        let client = Arc::new(ClientProxy::new(Connection {
            sender: conn.sender.clone(),
            receiver: conn.receiver.clone(),
        }));

        let mut analysis = EmmyLuaAnalysis::new();
        analysis.init_std_lib(true);

        let analysis = Arc::new(RwLock::new(analysis));
        let status_bar = Arc::new(StatusBar::new(client.clone()));
        let file_diagnostic = Arc::new(FileDiagnostic::new(
            analysis.clone(),
            client.clone(),
            status_bar.clone(),
        ));
        let config_manager = Arc::new(RwLock::new(ConfigManager::new(
            analysis.clone(),
            client.clone(),
            status_bar.clone(),
        )));

        ServerContext {
            conn,
            analysis,
            client,
            file_diagnostic,
            cancllations: Arc::new(Mutex::new(HashMap::new())),
            config_manager,
            status_bar,
        }
    }

    pub fn snapshot(&self) -> ServerContextSnapshot {
        ServerContextSnapshot {
            analysis: self.analysis.clone(),
            client: self.client.clone(),
            file_diagnostic: self.file_diagnostic.clone(),
            config_manager: self.config_manager.clone(),
            status_bar: self.status_bar.clone(),
        }
    }

    pub fn send(&self, response: Response) {
        let _ = self.conn.sender.send(Message::Response(response));
    }

    pub async fn task<F, Fut>(&self, req_id: RequestId, exec: F)
    where
        F: FnOnce(CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = Option<Response>> + Send + 'static,
    {
        let cancel_token = CancellationToken::new();

        {
            let mut cancellations = self.cancllations.lock().await;
            cancellations.insert(req_id.clone(), cancel_token.clone());
        }

        let sender = self.conn.sender.clone();
        let cancellations = self.cancllations.clone();

        tokio::spawn(async move {
            let res = exec(cancel_token.clone()).await;
            if cancel_token.is_cancelled() {
                let response = Response::new_err(
                    req_id.clone(),
                    ErrorCode::RequestCanceled as i32,
                    "cancel".to_string(),
                );
                let _ = sender.send(Message::Response(response));
            } else if res.is_none() {
                let response = Response::new_err(
                    req_id.clone(),
                    ErrorCode::InternalError as i32,
                    "internal error".to_string(),
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

    pub async fn close(&self) {
        let mut config_manager = self.config_manager.write().await;
        config_manager.watcher = None;
    }

    pub async fn send_response(&self, response: Response) {
        self.client.on_response(response).await;
    }
}
