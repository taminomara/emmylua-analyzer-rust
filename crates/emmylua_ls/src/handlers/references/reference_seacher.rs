use std::collections::HashMap;

use emmylua_code_analysis::{
    LuaCompilation, LuaDeclId, LuaMemberId, LuaMemberKey, LuaPropertyOwnerId, SemanticModel,
};
use emmylua_parser::{LuaAstNode, LuaAstToken, LuaNameToken, LuaStringToken, LuaSyntaxToken};
use lsp_types::Location;

pub fn search_references(
    semantic_model: &mut SemanticModel,
    compilation: &LuaCompilation,
    token: LuaSyntaxToken,
) -> Option<Vec<Location>> {
    let mut result = Vec::new();
    if let Some(property_owner) = semantic_model.get_property_owner_id(token.clone().into()) {
        match property_owner {
            LuaPropertyOwnerId::LuaDecl(decl_id) => {
                search_decl_references(semantic_model, decl_id, &mut result);
            }
            LuaPropertyOwnerId::Member(member_id) => {
                search_member_references(semantic_model, compilation, member_id, &mut result);
            }
            _ => {}
        }
    } else if let Some(token) = LuaStringToken::cast(token.clone()) {
        search_string_references(semantic_model, token, &mut result);
    } else if semantic_model.get_emmyrc().references.fuzzy_search {
        fuzzy_search_references(compilation, token, &mut result);
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
        let decl_refs = semantic_model
            .get_db()
            .get_reference_index()
            .get_decl_references(&decl_id.file_id, &decl_id)?;
        let document = semantic_model.get_document();
        for decl_ref in decl_refs {
            let location = document.to_lsp_location(decl_ref.range.clone())?;
            result.push(location);
        }

        return Some(());
    } else {
        let name = decl.get_name();
        let global_references = semantic_model
            .get_db()
            .get_reference_index()
            .get_global_references(name)?;
        for in_filed_syntax_id in global_references {
            let document =
                semantic_model.get_document_by_file_id(in_filed_syntax_id.file_id)?;
            let location = document.to_lsp_location(in_filed_syntax_id.value.get_range())?;
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

fn fuzzy_search_references(
    compilation: &LuaCompilation,
    token: LuaSyntaxToken,
    result: &mut Vec<Location>,
) -> Option<()> {
    let name = LuaNameToken::cast(token)?;
    let name_text = name.get_name_text();
    let fuzzy_references = compilation
        .get_db()
        .get_reference_index()
        .get_index_references(&LuaMemberKey::Name(name_text.to_string().into()))?;

    let mut semantic_cache = HashMap::new();
    for in_filed_syntax_id in fuzzy_references {
        let semantic_model =
            if let Some(semantic_model) = semantic_cache.get_mut(&in_filed_syntax_id.file_id) {
                semantic_model
            } else {
                let semantic_model = compilation.get_semantic_model(in_filed_syntax_id.file_id)?;
                semantic_cache.insert(in_filed_syntax_id.file_id, semantic_model);
                semantic_cache.get_mut(&in_filed_syntax_id.file_id)?
            };

        let document = semantic_model.get_document();
        let range = in_filed_syntax_id.value.get_range();
        let location = document.to_lsp_location(range)?;
        result.push(location);
    }

    Some(())
}
