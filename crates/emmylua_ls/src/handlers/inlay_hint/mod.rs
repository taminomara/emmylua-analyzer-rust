mod build_inlay_hint;

use build_inlay_hint::build_inlay_hints;
use lsp_types::{InlayHint, InlayHintParams};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_inlay_hint_handler(
    context: ServerContextSnapshot,
    params: InlayHintParams,
    _: CancellationToken,
) -> Option<Vec<InlayHint>> {
    let uri = params.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    build_inlay_hints(&mut semantic_model)
}

#[allow(unused_variables)]
pub async fn on_resolve_inlay_hint(
    context: ServerContextSnapshot,
    inlay_hint: InlayHint,
    _: CancellationToken,
) -> InlayHint {
    inlay_hint
}
