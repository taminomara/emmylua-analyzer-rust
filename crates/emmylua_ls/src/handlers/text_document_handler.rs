use lsp_types::DidOpenTextDocumentParams;

use crate::context::ServerContextSnapshot;

pub fn on_did_open_text_document(
    _: ServerContextSnapshot,
    params: DidOpenTextDocumentParams,
) {
    eprintln!("on_did_open_text_document {:?}", params.text_document.uri);
}
