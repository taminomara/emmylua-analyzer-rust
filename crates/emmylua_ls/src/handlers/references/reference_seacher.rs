use std::collections::HashMap;

use code_analysis::{
    LuaCompilation, LuaDeclId, LuaMemberId, LuaMemberKey, LuaPropertyOwnerId, SemanticModel,
};
use emmylua_parser::{LuaAstNode, LuaAstToken, LuaStringToken, LuaSyntaxToken};
use lsp_types::Location;

pub fn search_references(
    semantic_model: &mut SemanticModel,
    compilation: &LuaCompilation,
    token: LuaSyntaxToken,
) -> Option<Vec<Location>> {
    let mut result = Vec::new();
    let semantic_info = semantic_model.get_semantic_info(token.clone().into());
    if let Some(property_owner) = semantic_info?.property_owner {
        match property_owner {
            LuaPropertyOwnerId::LuaDecl(decl_id) => {
                search_decl_references(semantic_model, decl_id, &mut result);
            }
            LuaPropertyOwnerId::Member(member_id) => {
                search_member_references(semantic_model, compilation, member_id, &mut result);
            }
            _ => {}
        }
    } else if let Some(token) = LuaStringToken::cast(token) {
        search_string_references(semantic_model, token, &mut result);
    } else if semantic_model.get_emmyrc().references.fuzzy_search {
        // todo!()
    }

    Some(result)
}

pub fn search_decl_references(
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

pub fn search_member_references(
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

    let property_owner = LuaPropertyOwnerId::Member(member_id);
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
        if semantic_model.is_reference_to(node, property_owner.clone()) {
            let document = semantic_model.get_document();
            let range = in_filed_syntax_id.value.get_range();
            let location = document.to_lsp_location(range)?;
            result.push(location);
        }
    }

    Some(())
}

fn search_string_references(
    semantic_model: &SemanticModel,
    token: LuaStringToken,
    result: &mut Vec<Location>,
) -> Option<()> {
    let string_token_text = token.get_value();
    let string_refs = semantic_model
        .get_db()
        .get_reference_index()
        .get_string_references(&string_token_text);

    for in_filed_reference_range in string_refs {
        let document = semantic_model.get_document_by_file_id(in_filed_reference_range.file_id)?;
        let location = document.to_lsp_location(in_filed_reference_range.value)?;
        result.push(location);
    }

    Some(())
}
