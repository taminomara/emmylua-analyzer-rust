use std::collections::HashMap;

use emmylua_code_analysis::{LuaCompilation, LuaDeclId, SemanticModel};
use emmylua_parser::{LuaAst, LuaAstNode, LuaAstToken, LuaNameToken};
use lsp_types::Uri;

pub fn rename_decl_references(
    semantic_model: &SemanticModel,
    compilation: &LuaCompilation,
    decl_id: LuaDeclId,
    new_name: String,
    result: &mut HashMap<Uri, HashMap<lsp_types::Range, String>>,
) -> Option<()> {
    let decl = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)?;
    if decl.is_local() {
        let local_references = semantic_model
            .get_db()
            .get_reference_index()
            .get_decl_references(&decl_id.file_id, &decl_id);
        let document = semantic_model.get_document();
        let uri = document.get_uri();
        if let Some(decl_refs) = local_references {
            for decl_ref in decl_refs {
                let range = document.to_lsp_range(decl_ref.range.clone())?;
                result
                    .entry(uri.clone())
                    .or_insert_with(HashMap::new)
                    .insert(range, new_name.clone());
            }
        }

        let decl_range = get_decl_name_token_lsp_range(semantic_model, decl_id)?;
        result
            .entry(uri)
            .or_insert_with(HashMap::new)
            .insert(decl_range, new_name.clone());

        return Some(());
    } else {
        let name = decl.get_name();
        let global_references = semantic_model
            .get_db()
            .get_reference_index()
            .get_global_references(name)?;

        let mut semantic_cache = HashMap::new();
        for in_filed_syntax_id in global_references {
            let semantic_model = if let Some(semantic_model) =
                semantic_cache.get_mut(&in_filed_syntax_id.file_id)
            {
                semantic_model
            } else {
                let semantic_model = compilation.get_semantic_model(in_filed_syntax_id.file_id)?;
                semantic_cache.insert(in_filed_syntax_id.file_id, semantic_model);
                semantic_cache.get_mut(&in_filed_syntax_id.file_id)?
            };
            let document = semantic_model.get_document();
            let uri = document.get_uri();
            let range = document.to_lsp_range(in_filed_syntax_id.value.get_range())?;
            result
                .entry(uri)
                .or_insert_with(HashMap::new)
                .insert(range, new_name.clone());
        }
    }

    Some(())
}

fn get_decl_name_token_lsp_range(
    semantic_model: &SemanticModel,
    decl_id: LuaDeclId,
) -> Option<lsp_types::Range> {
    let decl = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)?;
    let document = semantic_model.get_document_by_file_id(decl_id.file_id)?;
    let syntax_id = decl.get_syntax_id();
    let root = semantic_model.get_root();
    let node = LuaAst::cast(syntax_id.to_node_from_root(root.syntax())?)?;
    let token = node.token::<LuaNameToken>()?;
    document.to_lsp_range(token.get_range())
}
