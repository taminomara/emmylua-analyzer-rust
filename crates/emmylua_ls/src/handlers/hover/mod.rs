mod keyword_hover;
mod build_hover;
mod hover_humanize;

use build_hover::build_semantic_info_hover;
use code_analysis::humanize_type;
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

    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, right) => right,
        TokenAtOffset::None => {
            return None;
        }
    };

    match token {
        keywords if is_keyword(keywords.clone()) => {
            let document = semantic_model.get_document();
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: hover_keyword(keywords.clone()),
                }),
                range: document.to_lsp_range(keywords.text_range()),
            });
        }
        _ => {
            let semantic_info = semantic_model.get_semantic_info(token.clone().into())?;
            let db = semantic_model.get_db();
            let document = semantic_model.get_document();
            build_semantic_info_hover(db, &document, token, semantic_info)
        }
    }
}
