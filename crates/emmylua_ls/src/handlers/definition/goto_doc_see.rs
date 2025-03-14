use emmylua_code_analysis::{LuaMemberKey, LuaPropertyOwnerId, LuaType, SemanticModel};
use emmylua_parser::{LuaAstToken, LuaGeneralToken};
use lsp_types::GotoDefinitionResponse;

pub fn goto_doc_see(
    semantic_model: &SemanticModel,
    content_token: LuaGeneralToken,
) -> Option<GotoDefinitionResponse> {
    let text = content_token.get_text();
    let name_parts = text.split('#').collect::<Vec<_>>();

    match name_parts.len() {
        1 => {
            let name = &name_parts[0];
            return goto_type(semantic_model, &name);
        }
        2 => {
            let type_name = &name_parts[0];
            let member_name = &name_parts[1];
            return goto_type_member(semantic_model, &type_name, &member_name);
        }
        _ => {}
    }

    None
}

fn goto_type(semantic_model: &SemanticModel, type_name: &str) -> Option<GotoDefinitionResponse> {
    let file_id = semantic_model.get_file_id();
    let type_decl = semantic_model
        .get_db()
        .get_type_index()
        .find_type_decl(file_id, type_name)?;
    let locations = type_decl.get_locations();
    let mut result = Vec::new();
    for location in locations {
        let document = semantic_model.get_document_by_file_id(location.file_id)?;
        let lsp_location = document.to_lsp_location(location.range)?;
        result.push(lsp_location);
    }

    Some(GotoDefinitionResponse::Array(result))
}

fn goto_type_member(
    semantic_model: &SemanticModel,
    type_name: &str,
    member_name: &str,
) -> Option<GotoDefinitionResponse> {
    let file_id = semantic_model.get_file_id();
    let type_decl = semantic_model
        .get_db()
        .get_type_index()
        .find_type_decl(file_id, type_name)?;
    let type_id = type_decl.get_id();
    let typ = LuaType::Ref(type_id);
    let member_map = semantic_model.infer_member_map(&typ)?;
    let member_infos = member_map.get(&LuaMemberKey::Name(member_name.to_string().into()))?;

    let mut result = Vec::new();
    for member_info in member_infos {
        if let Some(LuaPropertyOwnerId::Member(member_id)) = &member_info.property_owner_id {
            let file_id = member_id.file_id;
            let member_range = member_id.get_syntax_id().get_range();
            let document = semantic_model.get_document_by_file_id(file_id)?;
            let lsp_location = document.to_lsp_location(member_range)?;
            result.push(lsp_location);
        }
    }

    Some(GotoDefinitionResponse::Array(result))
}
