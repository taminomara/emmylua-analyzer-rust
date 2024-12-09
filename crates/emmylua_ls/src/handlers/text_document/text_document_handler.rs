use lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams};

use crate::context::ServerContextSnapshot;

pub async fn on_did_open_text_document(
    context: ServerContextSnapshot,
    params: DidOpenTextDocumentParams,
) -> Option<()> {
    let mut analysis = context.analysis.write().await;
    let uri = params.text_document.uri;
    let text = params.text_document.text;
    let file_id = analysis.update_file_by_uri(&uri, Some(text));
    if let Some(file_id) = file_id {
        context.file_diagnostic.add_diagnostic_task(file_id).await;
    }

    Some(())
}

pub async fn on_did_save_text_document(
    context: ServerContextSnapshot,
    params: DidSaveTextDocumentParams,
) -> Option<()> {
    let analysis = context.analysis.read().await;
    let uri = params.text_document.uri;
    let file_id = analysis.get_file_id(&uri);
    if let Some(file_id) = file_id {
        context.file_diagnostic.add_diagnostic_task(file_id).await;
    }

    Some(())
}

pub async fn on_did_change_text_document(
    context: ServerContextSnapshot,
    params: DidChangeTextDocumentParams,
) -> Option<()> {
    let mut analysis = context.analysis.write().await;
    let uri = params.text_document.uri;
    let text = params.content_changes.first()?.text.clone();
    let file_id = analysis.update_file_by_uri(&uri, Some(text));
    drop(analysis);
    if let Some(file_id) = file_id {
        context.file_diagnostic.add_diagnostic_task(file_id).await;
    }

    Some(())
}