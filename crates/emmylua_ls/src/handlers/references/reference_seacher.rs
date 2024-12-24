use std::collections::HashMap;

use code_analysis::{
    LuaCompilation, LuaDeclId, LuaMemberId, LuaMemberKey, LuaPropertyOwnerId, SemanticModel,
};
use emmylua_parser::{LuaAstNode, LuaSyntaxToken};
use lsp_types::Location;

pub fn search_references(
    semantic_model: &mut SemanticModel,
    compilation: &LuaCompilation,
    token: LuaSyntaxToken,
) -> Option<Vec<Location>> {
    let mut result = Vec::new();
    let semantic_info = semantic_model.get_semantic_info(token.into())?;
    match semantic_info.property_owner? {
        LuaPropertyOwnerId::LuaDecl(decl_id) => {
            search_decl_references(semantic_model, decl_id, &mut result);
        }
        LuaPropertyOwnerId::Member(member_id) => {
            search_member_references(semantic_model, compilation, member_id, &mut result);
        }
        _ => {}
    }

    Some(result)
}

fn search_decl_references(
    semantic_model: &mut SemanticModel,
    decl_id: LuaDeclId,
    result: &mut Vec<Location>,
) -> Option<()> {
    let decl = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)?;
    if decl.is_local() {
        let local_references = semantic_model
            .get_db()
            .get_reference_index()
            .get_local_references(&decl_id.file_id, &decl_id)?;
        let document = semantic_model.get_document();
        for reference_range in local_references {
            let location = document.to_lsp_location(reference_range.clone())?;
            result.push(location);
        }

        return Some(());
    } else {
        let name = decl.get_name();
        let global_references = semantic_model
            .get_db()
            .get_reference_index()
            .get_global_references(&LuaMemberKey::Name(name.to_string().into()))?;
        for in_filed_reference_range in global_references {
            let document =
                semantic_model.get_document_by_file_id(in_filed_reference_range.file_id)?;
            let location = document.to_lsp_location(in_filed_reference_range.value.clone())?;
            result.push(location);
        }
    }

    Some(())
}

fn search_member_references(
    semantic_model: &SemanticModel,
    compilation: &LuaCompilation,
    member_id: LuaMemberId,
    result: &mut Vec<Location>,
) -> Option<()> {
    let member = semantic_model
        .get_db()
        .get_member_index()
        .get_member(&member_id)?;
    let key = member.get_key();
    let index_references = semantic_model
        .get_db()
        .get_reference_index()
        .get_index_references(&key)?;

    let mut semantic_cache = HashMap::new();

    for in_filed_syntax_id in index_references {
        let semantic_model =
            if let Some(semantic_model) = semantic_cache.get_mut(&in_filed_syntax_id.file_id) {
                semantic_model
            } else {
                let semantic_model = compilation.get_semantic_model(in_filed_syntax_id.file_id)?;
                semantic_cache.insert(in_filed_syntax_id.file_id, semantic_model);
                semantic_cache.get_mut(&in_filed_syntax_id.file_id)?
            };
        let root = semantic_model.get_root();
        let node = in_filed_syntax_id.value.to_node_from_root(root.syntax())?;
        let semantic_info = semantic_model.get_semantic_info(node.into())?;
        let property_owner = semantic_info.property_owner?;
        if property_owner == LuaPropertyOwnerId::Member(member_id) {
            let document = semantic_model.get_document();
            let range = in_filed_syntax_id.value.get_range();
            let location = document.to_lsp_location(range)?;
            result.push(location);
        } else if let LuaPropertyOwnerId::Member(ref_member_id) = &property_owner {
            let ref_owner = semantic_model
                .get_db()
                .get_member_index()
                .get_member(ref_member_id)?
                .get_owner();
            let self_onwer = semantic_model
                .get_db()
                .get_member_index()
                .get_member(&member_id)?
                .get_owner();
            if ref_owner == self_onwer {
                let document = semantic_model.get_document();
                let range = in_filed_syntax_id.value.get_range();
                let location = document.to_lsp_location(range)?;
                result.push(location);
            }
        }
    }

    Some(())
}
