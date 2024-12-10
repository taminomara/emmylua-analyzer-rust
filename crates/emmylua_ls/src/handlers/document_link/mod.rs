mod build_link;

use build_link::build_links;
use emmylua_parser::LuaAstNode;
use lsp_types::{DocumentLink, DocumentLinkParams};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_document_link_handler(
    context: ServerContextSnapshot,
    params: DocumentLinkParams,
    _: CancellationToken,
) -> Option<Vec<DocumentLink>> {
    let uri = params.text_document.uri;
    let config_manager = context.config_manager.read().await;
    let workspace_folders = config_manager.workspace_folders.clone();
    let _ = config_manager;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let document = semantic_model.get_document();
    let db = semantic_model.get_db();
    let emmyrc = analysis.get_emmyrc();

    build_links(&db, root.syntax().clone(), &document, &emmyrc, workspace_folders)
}

#[allow(unused_variables)]
pub async fn on_document_link_resolve_handler(
    _: ServerContextSnapshot,
    params: DocumentLink,
    _: CancellationToken,
) -> DocumentLink {
    params
}
