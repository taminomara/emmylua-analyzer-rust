use lsp_types::InitializeParams;

use crate::context::ServerContextSnapshot;

pub async fn initialized_handler(
    context: ServerContextSnapshot,
    params: InitializeParams,
) -> Option<()> {

    Some(())
}