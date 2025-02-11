mod goto_def_definition;
mod goto_doc_see;
mod goto_module_file;

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaDocTagSee, LuaNameToken, LuaStringToken, LuaTokenKind,
};
use goto_def_definition::goto_def_definition;
use goto_doc_see::goto_doc_see;
use goto_module_file::goto_module_file;
use lsp_types::{
    ClientCapabilities, GotoDefinitionParams, GotoDefinitionResponse, OneOf, ServerCapabilities,
};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_goto_definition_handler(
    context: ServerContextSnapshot,
    params: GotoDefinitionParams,
    _: CancellationToken,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.text_document_position_params.position;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    if position_offset > root.syntax().text_range().end() {
        return None;
    }

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

    if let Some(property_owner) = semantic_model.get_property_owner_id(token.clone().into()) {
        return goto_def_definition(&semantic_model, property_owner);
    } else if let Some(string_token) = LuaStringToken::cast(token.clone()) {
        if let Some(module_response) = goto_module_file(&semantic_model, string_token) {
            return Some(module_response);
        }
    } else if let Some(name_token) = LuaNameToken::cast(token.clone()) {
        if let Some(doc_see) = name_token.get_parent::<LuaDocTagSee>() {
            return goto_doc_see(&semantic_model, doc_see, name_token);
        }
    }

    // goto self
    let document = semantic_model.get_document();
    let lsp_location = document.to_lsp_location(token.text_range())?;
    Some(GotoDefinitionResponse::Scalar(lsp_location))
}

pub fn register_capabilities(
    server_capabilities: &mut ServerCapabilities,
    _: &ClientCapabilities,
) -> Option<()> {
    server_capabilities.definition_provider = Some(OneOf::Left(true));
    Some(())
}
