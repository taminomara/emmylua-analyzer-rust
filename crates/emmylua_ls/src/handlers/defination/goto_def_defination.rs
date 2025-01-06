use code_analysis::{LuaPropertyOwnerId, SemanticModel};
use lsp_types::GotoDefinitionResponse;

pub fn goto_def_defination(
    semantic_model: &SemanticModel,
    property_owner: LuaPropertyOwnerId,
) -> Option<GotoDefinitionResponse> {
    match property_owner {
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
