use std::collections::HashMap;

use emmylua_code_analysis::{
    LuaCompilation, LuaDeclId, LuaMemberId, LuaMemberKey, LuaSemanticDeclId, LuaTypeDeclId,
    SemanticDeclLevel, SemanticModel,
};
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaNameToken, LuaStringToken, LuaSyntaxNode, LuaSyntaxToken,
};
use lsp_types::Location;

pub fn search_references(
    semantic_model: &SemanticModel,
    compilation: &LuaCompilation,
    token: LuaSyntaxToken,
) -> Option<Vec<Location>> {
    let mut result = Vec::new();
    if let Some(semantic_decl) =
        semantic_model.find_decl(token.clone().into(), SemanticDeclLevel::default())
    {
        match semantic_decl {
            LuaSemanticDeclId::LuaDecl(decl_id) => {
                search_decl_references(semantic_model, decl_id, &mut result);
            }
            LuaSemanticDeclId::Member(member_id) => {
                search_member_references(semantic_model, compilation, member_id, &mut result);
            }
            LuaSemanticDeclId::TypeDecl(type_decl_id) => {
                search_type_decl_references(semantic_model, type_decl_id, &mut result);
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
    semantic_model: &SemanticModel,
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
            let document = semantic_model.get_document_by_file_id(in_filed_syntax_id.file_id)?;
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

    let property_owner = LuaSemanticDeclId::Member(member_id);
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
        if semantic_model.is_reference_to(
            node.clone(),
            property_owner.clone(),
            SemanticDeclLevel::default(),
        ) {
            let document = semantic_model.get_document();
            let range = in_filed_syntax_id.value.get_range();
            let location = document.to_lsp_location(range)?;
            result.push(location);
            search_member_secondary_references(semantic_model, node, result);
        }
    }

    Some(())
}

fn search_member_secondary_references(
    semantic_model: &SemanticModel,
    node: LuaSyntaxNode,
    result: &mut Vec<Location>,
) -> Option<()> {
    let position = node.text_range().start();
    let parent = LuaAst::cast(node.parent()?)?;
    match parent {
        LuaAst::LuaAssignStat(assign_stat) => {
            let (vars, values) = assign_stat.get_var_and_expr_list();
            let idx = values
                .iter()
                .position(|value| value.get_position() == position)?;
            let var = vars.get(idx)?;
            let decl_id = LuaDeclId::new(semantic_model.get_file_id(), var.get_position());
            search_decl_references(semantic_model, decl_id, result);
            let document = semantic_model.get_document();
            let range = document.to_lsp_location(var.get_range())?;
            result.push(range);
        }
        LuaAst::LuaLocalStat(local_stat) => {
            let local_names = local_stat.get_local_name_list().collect::<Vec<_>>();
            let mut values = local_stat.get_value_exprs();
            let idx = values.position(|value| value.get_position() == position)?;
            let name = local_names.get(idx)?;
            let decl_id = LuaDeclId::new(semantic_model.get_file_id(), name.get_position());
            search_decl_references(semantic_model, decl_id, result);
            let document = semantic_model.get_document();
            let range = document.to_lsp_location(name.get_range())?;
            result.push(range);
        }
        _ => {}
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

fn search_type_decl_references(
    semantic_model: &SemanticModel,
    type_decl_id: LuaTypeDeclId,
    result: &mut Vec<Location>,
) -> Option<()> {
    let refs = semantic_model
        .get_db()
        .get_reference_index()
        .get_type_references(&type_decl_id)?;
    let mut document_cache = HashMap::new();
    for in_filed_reference_range in refs {
        let document = if let Some(document) = document_cache.get(&in_filed_reference_range.file_id)
        {
            document
        } else {
            let document =
                semantic_model.get_document_by_file_id(in_filed_reference_range.file_id)?;
            document_cache.insert(in_filed_reference_range.file_id, document);
            document_cache.get(&in_filed_reference_range.file_id)?
        };
        let location = document.to_lsp_location(in_filed_reference_range.value)?;
        result.push(location);
    }

    Some(())
}
