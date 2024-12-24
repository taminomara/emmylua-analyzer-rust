mod rename_references;

use emmylua_parser::{LuaAstNode, LuaTokenKind};
use lsp_types::{PrepareRenameResponse, RenameParams, TextDocumentPositionParams, WorkspaceEdit};
use rename_references::rename_references;
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_rename_handler(
    context: ServerContextSnapshot,
    params: RenameParams,
    _: CancellationToken,
) -> Option<WorkspaceEdit> {
    let uri = params.text_document_position.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.text_document_position.position;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(left, right) => {
            if left.kind() == LuaTokenKind::TkName.into() {
                left
            } else {
                right
            }
        }
        TokenAtOffset::None => {
            return None;
        }
    };

    rename_references(&mut semantic_model, &analysis.compilation, token, params.new_name)
}

pub async fn on_prepare_rename_handler(
    context: ServerContextSnapshot,
    params: TextDocumentPositionParams,
    _: CancellationToken,
) -> Option<PrepareRenameResponse> {
    let uri = params.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.position;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(left, right) => {
            if left.kind() == LuaTokenKind::TkName.into() {
                left
            } else {
                right
            }
        }
        TokenAtOffset::None => {
            return None;
        }
    };

    if matches!(
        token.kind().into(),
        LuaTokenKind::TkName | LuaTokenKind::TkInt | LuaTokenKind::TkString
    ) {
        Some(PrepareRenameResponse::DefaultBehavior {
            default_behavior: true,
        })
    } else {
        None
    }
}
