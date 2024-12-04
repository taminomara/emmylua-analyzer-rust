use std::time::Duration;

use code_analysis::FileId;
use log::info;
use lsp_types::DidOpenTextDocumentParams;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_did_open_text_document(
    _: ServerContextSnapshot,
    params: DidOpenTextDocumentParams,
) -> Option<()> {
    info!("on_did_open_text_document {:?}", params.text_document.uri);

    Some(())
}


async fn on_document_diagnostic(
    file_id: FileId,
    cancel_token: CancellationToken,
) -> Option<()> {

    select! {
        _ = tokio::time::sleep(Duration::from_secs(1)) => {}
        _ = cancel_token.cancelled() => {
            return None;
        }
    }

    Some(())
}