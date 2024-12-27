use lsp_types::{
    ClientCapabilities, CodeActionParams, CodeActionProviderCapability, CodeActionResponse,
    ServerCapabilities,
};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

#[allow(unused_variables)]
pub async fn on_code_action_handler(
    context: ServerContextSnapshot,
    params: CodeActionParams,
    _: CancellationToken,
) -> Option<CodeActionResponse> {
    None
}

pub fn register_capabilities(
    server_capabilities: &mut ServerCapabilities,
    _: &ClientCapabilities,
) -> Option<()> {
    server_capabilities.code_action_provider = Some(CodeActionProviderCapability::Simple(true));
    Some(())
}
