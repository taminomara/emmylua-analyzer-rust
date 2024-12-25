mod build_signature_helper;

use lsp_types::{SignatureHelp, SignatureHelpParams};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_signature_helper_handler(
    context: ServerContextSnapshot,
    params: SignatureHelpParams,
    _: CancellationToken,
) -> Option<SignatureHelp> {
    let uri = params.text_document_position_params.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    // let signature_help = semantic_model.get_signature_help(params.text_document_position_params)?;
    // Some(signature_help)
    None
}
