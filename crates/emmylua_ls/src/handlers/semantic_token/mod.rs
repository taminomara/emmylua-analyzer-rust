mod build_semantic_tokens;
mod semantic_token_builder;

use crate::context::ServerContextSnapshot;
use build_semantic_tokens::build_semantic_tokens;
use lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};
pub use semantic_token_builder::{SEMANTIC_TOKEN_MODIFIERS, SEMANTIC_TOKEN_TYPES};
use tokio_util::sync::CancellationToken;

pub async fn on_semantic_token_handler(
    context: ServerContextSnapshot,
    params: SemanticTokensParams,
    _: CancellationToken,
) -> Option<SemanticTokensResult> {
    let uri = params.text_document.uri;
    let analysis = context.analysis.read().await;
    let config_manager = context.config_manager.read().await;
    let support_muliline_token = config_manager.semantic_multiline_support;
    let client_id = config_manager.client_config.client_id;
    let _ = config_manager;
    let file_id = analysis.get_file_id(&uri)?;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;

    let result = build_semantic_tokens(&mut semantic_model, support_muliline_token, client_id)?;

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: result,
    }))
}
