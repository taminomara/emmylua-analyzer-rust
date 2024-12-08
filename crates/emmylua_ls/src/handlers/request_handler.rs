use std::{error::Error, future::Future};

use log::error;
use lsp_server::{Request, RequestId, Response};
use lsp_types::request::HoverRequest;
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::sync::CancellationToken;

use crate::context::{ServerContext, ServerContextSnapshot};

use super::hover::on_hover;

pub async fn on_req_handler(
    req: Request,
    server_context: &mut ServerContext,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    RequestDispatcher::new(req, server_context)
        .on_async::<HoverRequest, _, _>(on_hover).await
        .finish();
    Ok(())
}

pub struct RequestDispatcher<'a> {
    req: Option<Request>,
    context: &'a mut ServerContext,
}

impl<'a> RequestDispatcher<'a> {
    pub fn new(req: Request, context: &'a mut ServerContext) -> Self {
        RequestDispatcher {
            req: Some(req),
            context,
        }
    }

    pub async fn on_async<R, F, Fut>(
        &mut self,
        handler: F
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + Send + std::fmt::Debug + 'static,
        R::Result: Serialize + 'static,
        F: Fn(ServerContextSnapshot, R::Params, CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = R::Result> + Send + 'static,
    {
        let req = match &self.req {
            Some(req) if req.method == R::METHOD => self.req.take().unwrap(),
            _ => return self,
        };

        if R::METHOD == req.method {
            let snapshot = self.context.snapshot();
            let id = req.id.clone();
            let m: Result<(RequestId, R::Params), _> = req.extract(R::METHOD);
            self.context
                .task(id.clone(), |cancel_token| async move {
                    let result = handler(snapshot, m.unwrap().1, cancel_token).await;
                    Some(Response::new_ok(id, result))
                })
                .await;
        }
        self
    }

    pub fn finish(&mut self) {
        if let Some(req) = &self.req {
            error!("handler not found for request. [{}]", req.method);
            let response = Response::new_err(
                req.id.clone(),
                lsp_server::ErrorCode::MethodNotFound as i32,
                "handler not found".to_string(),
            );
            self.context.send(response);
        }
    }
}