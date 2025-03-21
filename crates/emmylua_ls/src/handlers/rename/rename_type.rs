use std::collections::HashMap;

use emmylua_code_analysis::{LuaTypeDeclId, SemanticModel};
use lsp_types::Uri;

pub fn rename_type_references(
    semantic_model: &SemanticModel,
    type_decl_id: LuaTypeDeclId,
    new_name: String,
    result: &mut HashMap<Uri, HashMap<lsp_types::Range, String>>,
) -> Option<()> {
    let type_decl = semantic_model
        .get_db()
        .get_type_index()
        .get_type_decl(&type_decl_id)?;

    let locations = type_decl.get_locations();
    for decl_location in locations {
        let document = semantic_model.get_document_by_file_id(decl_location.file_id)?;
        let range = document.to_lsp_range(decl_location.range)?;
        result
            .entry(document.get_uri())
            .or_insert_with(HashMap::new)
            .insert(range, new_name.clone());
    }

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
        result
            .entry(location.uri)
            .or_insert_with(HashMap::new)
            .insert(location.range, new_name.clone());
    }

    Some(())
}
