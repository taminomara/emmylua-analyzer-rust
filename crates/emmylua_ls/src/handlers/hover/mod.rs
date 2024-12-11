mod keyword_hover;

use emmylua_parser::{LuaAstNode, LuaExpr};
use keyword_hover::{hover_keyword, is_keyword};
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

#[allow(unused_variables)]
pub async fn on_hover(
    context: ServerContextSnapshot,
    params: HoverParams,
    cancel_token: CancellationToken,
) -> Option<Hover> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let document = semantic_model.get_document();
    let root = semantic_model.get_root();
    let position_offset =
        document.get_offset(position.line as usize, position.character as usize)?;
    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, right) => right,
        TokenAtOffset::None => {
            return None;
        }
    };

    if is_keyword(token.clone()) {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: hover_keyword(token.clone()),
            }),
            range: document.to_lsp_range(token.text_range()),
        });
    }

    let node = LuaExpr::cast(token.parent()?)?;
    let expr_type = semantic_model.infer_expr(node)?;

    // TODO: add detail hover
    // Some(hover)
    let hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: format!("{:?}", expr_type),
        }),
        range: None,
    };

    Some(hover)
}
