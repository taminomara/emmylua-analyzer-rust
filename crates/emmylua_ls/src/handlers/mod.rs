mod text_document_handler;

use std::error::Error;

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    notification::DidOpenTextDocument, HoverProviderCapability, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind
};
use serde::{de::DeserializeOwned, Serialize};
use text_document_handler::on_did_open_text_document;

use crate::context::{ServerContext, ServerContextSnapshot};

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            lsp_types::TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    }
}

pub fn on_req_handler(
    req: Request,
    server_context: &mut ServerContext,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    RequestDispatcher::new(req, server_context)
        .finish();
    // .on(handler)
    Ok(())
}

pub fn on_notification_handler(
    notification: Notification,
    server_context: &mut ServerContext,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    NotificationDispatcher::new(notification, server_context)
        .on_sync::<DidOpenTextDocument>(on_did_open_text_document)
        .finish();
    // .on(handler)
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

    pub fn on_sync<R>(
        &mut self,
        handler: fn(snapshot: ServerContextSnapshot, R::Params) -> R::Result,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + Send + std::fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        let req = match &self.req {
            Some(req) if req.method == R::METHOD => self.req.take().unwrap(),
            _ => return self,
        };

        if R::METHOD == req.method {
            // eprintln!("request: {}, params: {}", &req.method, &req.params.to_string());
            let snapshot = self.context.snapshot();
            let id = req.id.clone();
            let m: Result<(RequestId, R::Params), _> = req.extract(R::METHOD);
            self.context.task(move || {
                let result = handler(snapshot, m.unwrap().1);
                Response::new_ok(id, result)
            });
        }
        self
    }

    pub fn finish(&mut self) {
        if let Some(req) = &self.req {
            eprintln!("handler not found for request. [{}]", req.method)
        }
    }
}

pub struct NotificationDispatcher<'a> {
    notification: Option<Notification>,
    context: &'a mut ServerContext,
}

impl<'a> NotificationDispatcher<'a> {
    pub fn new(
        notification: Notification,
        context: &'a mut ServerContext,
    ) -> NotificationDispatcher {
        NotificationDispatcher {
            notification: Some(notification),
            context,
        }
    }

    pub fn on<R>(&mut self, handler: fn(snapshot: ServerContextSnapshot, R::Params)) -> &mut Self
    where
        R: lsp_types::notification::Notification + 'static,
        R::Params: DeserializeOwned + Send + std::fmt::Debug + 'static,
    {
        let notification = match &self.notification {
            Some(req) if req.method == R::METHOD => self.notification.take().unwrap(),
            _ => return self,
        };

        if R::METHOD == notification.method {
            let snapshot = self.context.snapshot();
            let m = notification.extract(R::METHOD);
            self.context.run(move || {
                handler(snapshot, m.unwrap());
            });
        }
        self
    }

    pub fn on_sync<R>(
        &mut self,
        handler: fn(snapshot: ServerContextSnapshot, R::Params),
    ) -> &mut Self
    where
        R: lsp_types::notification::Notification + 'static,
        R::Params: DeserializeOwned + Send + std::fmt::Debug + 'static,
    {
        let notification = match &self.notification {
            Some(req) if req.method == R::METHOD => self.notification.take().unwrap(),
            _ => return self,
        };

        if R::METHOD == notification.method {
            let snapshot = self.context.snapshot();
            let m = notification.extract(R::METHOD);
            handler(snapshot, m.unwrap());
        }
        self
    }

    pub fn finish(&mut self) {
        if let Some(notification) = &self.notification {
            eprintln!(
                "handler not found for notification. [{}]",
                notification.method
            )
        }
    }
}
