mod cancel_token;
mod snapshot;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

pub use cancel_token::CancelToken;
use code_analysis::EmmyLuaAnalysis;
use lsp_server::{Connection, ErrorCode, Message, RequestId, Response};
use lsp_types::InitializeParams;
pub use snapshot::ServerContextSnapshot;
use threadpool::ThreadPool;

pub struct ServerContext {
    thread_pool: ThreadPool,
    conn: Connection,
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    cancllations: Arc<Mutex<HashMap<RequestId, CancelToken>>>,
}

impl ServerContext {
    pub fn new(_: InitializeParams, conn: Connection) -> Self {
        ServerContext {
            thread_pool: ThreadPool::default(),
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

    pub fn task<F>(&mut self, req_id: RequestId, exec: F)
    where
        F: FnOnce(CancelToken) -> Option<Response> + Send + 'static,
    {
        let cancel_token = CancelToken::new();

        // Register cancellation token
        {
            let mut cancellations = self.cancllations.lock().unwrap();
            cancellations.insert(req_id.clone(), cancel_token.clone());
        }

        let sender = self.conn.sender.clone();
        let cancellations = self.cancllations.clone();
        self.thread_pool.execute(move || {
            let mut res = exec(cancel_token.clone());
            if cancel_token.is_canceled() || res.is_none() {
                res = Some(Response::new_err(
                    req_id.clone(),
                    ErrorCode::RequestCanceled as i32,
                    "canccel".to_string(),
                ));
            }
            let _ = sender.send(Message::Response(res.unwrap()));

            // Remove cancellation token
            {
                let mut cancellations = cancellations.lock().unwrap();
                cancellations.remove(&req_id);
            }
        });
    }

    pub fn cancel(&mut self, req_id: RequestId) {
        let cancellations = self.cancllations.lock().unwrap();
        if let Some(cancel_token) = cancellations.get(&req_id) {
            cancel_token.cancel();
        }
    }

    pub fn run<F>(&mut self, exec: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.thread_pool.execute(move || {
            exec();
        });
    }
}
