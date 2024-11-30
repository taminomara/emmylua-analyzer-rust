use std::{error::Error, future::Future};

use lsp_server::Notification;
use lsp_types::{
    notification::{Cancel, DidOpenTextDocument, Notification as lsp_notification},
    CancelParams, NumberOrString,
};
use serde::de::DeserializeOwned;

use crate::context::{ServerContext, ServerContextSnapshot};

use super::text_document::on_did_open_text_document;

pub async fn on_notification_handler(
    notification: Notification,
    server_context: &mut ServerContext,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    NotificationDispatcher::new(notification, server_context)
        .on_cancel()
        .await
        .on_async::<DidOpenTextDocument, _, _>(on_did_open_text_document)
        .finish();
    // .on(handler)
    Ok(())
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

    pub fn on_async<R, F, Fut>(&mut self, handler: F) -> &mut Self
    where
        R: lsp_types::notification::Notification + 'static,
        R::Params: DeserializeOwned + Send + std::fmt::Debug + 'static,
        F: Fn(ServerContextSnapshot, R::Params) -> Fut + Send + 'static,
        Fut: Future<Output = Option<()>> + Send + 'static,
    {
        let notification = match &self.notification {
            Some(req) if req.method == R::METHOD => self.notification.take().unwrap(),
            _ => return self,
        };

        if R::METHOD == notification.method {
            let snapshot = self.context.snapshot();
            let m = notification.extract(R::METHOD);
            tokio::spawn(async move {
                handler(snapshot, m.unwrap()).await;
            });
        }
        self
    }

    #[allow(dead_code)]
    pub fn on_sync<R>(
        &mut self,
        handler: fn(ServerContextSnapshot, R::Params) -> Option<()>,
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

    pub async fn on_cancel(&mut self) -> &mut Self {
        let notification = match &self.notification {
            Some(req) if req.method == Cancel::METHOD => self.notification.take().unwrap(),
            _ => return self,
        };

        if Cancel::METHOD == notification.method {
            let m: Result<CancelParams, _> = notification.extract(Cancel::METHOD);
            let req_id = match m.unwrap().id {
                NumberOrString::Number(i) => i.into(),
                NumberOrString::String(s) => s.into(),
            };

            self.context.cancel(req_id).await;
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
