use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;


pub async fn on_hover(
    context: ServerContextSnapshot,
    params: HoverParams,
    cancel_token: CancellationToken
) -> Option<Hover> {
    // let uri = params.text_document_position_params.text_document.uri;
    // let position = params.text_document_position_params.position;
    // let analysis = context.analysis.read().await;
    // let file_id = analysis.get_file_id(&uri)?;
    // let hover = analysis.hover(file_id, position)?;
    // Some(hover)
    let hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: "Hello, World!".to_string()
        }),
        range: None
    };
    
    Some(hover)
}