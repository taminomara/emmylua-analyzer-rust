use std::{error::Error, future::Future};

use log::error;
use lsp_server::{Request, RequestId, Response};
use lsp_types::request::{
    CodeLensRequest, CodeLensResolve, ColorPresentationRequest, Completion, DocumentColor,
    DocumentHighlightRequest, DocumentLinkRequest, DocumentLinkResolve, DocumentSymbolRequest,
    FoldingRangeRequest, GotoDefinition, HoverRequest, InlayHintRequest, InlayHintResolveRequest,
    PrepareRenameRequest, References, Rename, ResolveCompletionItem, SelectionRangeRequest,
    SemanticTokensFullRequest, SignatureHelpRequest,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::sync::CancellationToken;

use crate::context::{ServerContext, ServerContextSnapshot};

use super::{
    code_lens::{on_code_lens_handler, on_resolve_code_lens_handler},
    completion::{on_completion_handler, on_completion_resolve_handler},
    defination::on_goto_defination_handler,
    document_color::{on_document_color, on_document_color_presentation},
    document_highlight::on_document_highlight_handler,
    document_link::{on_document_link_handler, on_document_link_resolve_handler},
    document_selection_range::on_document_selection_range_handle,
    document_symbol::on_document_symbol,
    emmy_annotator::{on_emmy_annotator_handler, EmmyAnnotatorRequest},
    fold_range::on_folding_range_handler,
    hover::on_hover,
    inlay_hint::{on_inlay_hint_handler, on_resolve_inlay_hint},
    references::on_references_handler,
    rename::{on_prepare_rename_handler, on_rename_handler},
    semantic_token::on_semantic_token_handler,
    signature_helper::on_signature_helper_handler,
};

pub async fn on_req_handler(
    req: Request,
    server_context: &mut ServerContext,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    RequestDispatcher::new(req, server_context)
        .on_parallel::<HoverRequest, _, _>(on_hover)
        .await
        .on_parallel::<DocumentSymbolRequest, _, _>(on_document_symbol)
        .await
        .on_parallel::<FoldingRangeRequest, _, _>(on_folding_range_handler)
        .await
        .on_parallel::<DocumentColor, _, _>(on_document_color)
        .await
        .on_parallel::<ColorPresentationRequest, _, _>(on_document_color_presentation)
        .await
        .on_parallel::<DocumentLinkRequest, _, _>(on_document_link_handler)
        .await
        .on_parallel::<DocumentLinkResolve, _, _>(on_document_link_resolve_handler)
        .await
        .on_parallel::<EmmyAnnotatorRequest, _, _>(on_emmy_annotator_handler)
        .await
        .on_parallel::<SelectionRangeRequest, _, _>(on_document_selection_range_handle)
        .await
        .on_parallel::<Completion, _, _>(on_completion_handler)
        .await
        .on_parallel::<ResolveCompletionItem, _, _>(on_completion_resolve_handler)
        .await
        .on_parallel::<InlayHintRequest, _, _>(on_inlay_hint_handler)
        .await
        .on_parallel::<InlayHintResolveRequest, _, _>(on_resolve_inlay_hint)
        .await
        .on_parallel::<GotoDefinition, _, _>(on_goto_defination_handler)
        .await
        .on_parallel::<References, _, _>(on_references_handler)
        .await
        .on_parallel::<Rename, _, _>(on_rename_handler)
        .await
        .on_parallel::<PrepareRenameRequest, _, _>(on_prepare_rename_handler)
        .await
        .on_parallel::<CodeLensRequest, _, _>(on_code_lens_handler)
        .await
        .on_parallel::<CodeLensResolve, _, _>(on_resolve_code_lens_handler)
        .await
        .on_parallel::<SignatureHelpRequest, _, _>(on_signature_helper_handler)
        .await
        .on_parallel::<DocumentHighlightRequest, _, _>(on_document_highlight_handler)
        .await
        .on_parallel::<SemanticTokensFullRequest, _, _>(on_semantic_token_handler)
        .await
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

    pub async fn on_parallel<R, F, Fut>(&mut self, handler: F) -> &mut Self
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
