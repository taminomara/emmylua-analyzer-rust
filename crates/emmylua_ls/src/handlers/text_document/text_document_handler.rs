use lsp_types::DidOpenTextDocumentParams;

use crate::context::ServerContextSnapshot;

pub async fn on_did_open_text_document(
    _: ServerContextSnapshot,
    params: DidOpenTextDocumentParams,
) -> Option<()> {
    eprintln!("on_did_open_text_document {:?}", params.text_document.uri);

    Some(())
}
