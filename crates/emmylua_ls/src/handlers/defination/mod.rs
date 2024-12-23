use code_analysis::LuaPropertyOwnerId;
use emmylua_parser::{LuaAstNode, LuaTokenKind};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_goto_defination_handler(
    context: ServerContextSnapshot,
    params: GotoDefinitionParams,
    _: CancellationToken,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.text_document_position_params.position;
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

    let semantic_info = semantic_model.get_semantic_info(token.clone().into())?;
    match semantic_info.property_owner? {
        LuaPropertyOwnerId::LuaDecl(decl_id) => {
            let decl = semantic_model
                .get_db()
                .get_decl_index()
                .get_decl(&decl_id)?;
            let document = semantic_model.get_document_by_file_id(decl_id.file_id)?;
            let location = document.to_lsp_location(decl.get_range())?;
            return Some(GotoDefinitionResponse::Scalar(location));
        }
        LuaPropertyOwnerId::Member(member_id) => {
            let member = semantic_model
                .get_db()
                .get_member_index()
                .get_member(&member_id)?;
            let document = semantic_model.get_document_by_file_id(member_id.file_id)?;
            let location = document.to_lsp_location(member.get_range())?;
            return Some(GotoDefinitionResponse::Scalar(location));
        }
        LuaPropertyOwnerId::TypeDecl(type_decl_id) => {
            let type_decl = semantic_model
                .get_db()
                .get_type_index()
                .get_type_decl(&type_decl_id)?;
            let mut locations = Vec::new();
            for lua_location in type_decl.get_locations() {
                let document = semantic_model.get_document_by_file_id(lua_location.file_id)?;
                let location = document.to_lsp_location(lua_location.range)?;
                locations.push(location);
            }

            return Some(GotoDefinitionResponse::Array(locations));
        }
        _ => {}
    }

    None
}
