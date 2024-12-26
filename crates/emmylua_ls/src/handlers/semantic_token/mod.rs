use lsp_types::{SemanticTokensParams, SemanticTokensResult};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_semantic_token_handler(
    context: ServerContextSnapshot,
    params: SemanticTokensParams,
    _: CancellationToken,
) -> Option<SemanticTokensResult> {
 
 
    None
}