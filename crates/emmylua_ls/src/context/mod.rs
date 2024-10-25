mod snapshot;

use lsp_server::{Connection, Message, Response};
use lsp_types::InitializeParams;
pub use snapshot::ServerContextSnapshot;
use threadpool::ThreadPool;

pub struct ServerContext {
    // thread pool
    thread_pool: ThreadPool,
    conn: Connection,
}

impl ServerContext {
    pub fn new(_: InitializeParams, conn: Connection) -> Self {
        ServerContext {
            thread_pool: ThreadPool::default(),
            conn,
        }
    }

    pub fn snapshot(&self) -> ServerContextSnapshot {
        ServerContextSnapshot {}
    }

    pub fn task<F>(&mut self, exec: F)
    where
        F: FnOnce() -> Response + Send + 'static,
    {
        let s = self.conn.sender.clone();
        self.thread_pool.execute(move || {
            let res = exec();
            let _ = s.send(Message::Response(res));
        });
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
