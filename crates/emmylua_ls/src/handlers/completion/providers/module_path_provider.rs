use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaLiteralExpr, LuaStringToken,
};
use lsp_types::{CompletionItem, CompletionTextEdit, TextEdit};

use crate::handlers::completion::{
    completion_builder::CompletionBuilder, completion_data::CompletionData,
};

use super::get_text_edit_range_in_string;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let string_token = LuaStringToken::cast(builder.trigger_token.clone())?;
    let call_expr = string_token
        .get_parent::<LuaLiteralExpr>()?
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;

    let emmyrc = builder.semantic_model.get_emmyrc();
    if !call_expr.is_require() {
        return None;
    }

    let version_number = emmyrc.runtime.version.to_lua_version_number();
    let prefix_content = string_token.get_value();
    let parts: Vec<&str> = prefix_content
        .split(|c| c == '.' || c == '/' || c == '\\')
        .collect();
    let module_path = if parts.len() > 1 {
        parts[..parts.len() - 1].join(".")
    } else {
        "".to_string()
    };

    let prefix = if let Some(last_sep) = prefix_content.rfind(|c| c == '/' || c == '\\' || c == '.')
    {
        let (path, _) = prefix_content.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let text_edit_range = get_text_edit_range_in_string(builder, string_token)?;

    let db = builder.semantic_model.get_db();
    let mut module_completions = Vec::new();
    let module_info = db.get_module_index().find_module_node(&module_path)?;
    for (name, module_id) in &module_info.children {
        let child_module_node = db.get_module_index().get_module_node(module_id)?;
        let filter_text = format!("{}{}", prefix, name);
        let text_edit = TextEdit {
            range: text_edit_range.clone(),
            new_text: filter_text.clone(),
        };
        if let Some(child_file_id) = child_module_node.file_ids.first() {
            let child_module_info = db.get_module_index().get_module(*child_file_id)?;
            let data = if let Some(property_id) = &child_module_info.property_owner_id {
                CompletionData::from_property_owner_id(builder, property_id.clone(), None)
            } else {
                None
            };

            if child_module_info.is_visible(&version_number) {
                let uri = db.get_vfs().get_uri(child_file_id)?;
                let completion_item = CompletionItem {
                    label: name.clone(),
                    kind: Some(lsp_types::CompletionItemKind::FILE),
                    filter_text: Some(filter_text.clone()),
                    text_edit: Some(CompletionTextEdit::Edit(text_edit)),
                    detail: Some(uri.to_string()),
                    data,
                    ..Default::default()
                };
                module_completions.push(completion_item);
            }
        } else {
            let completion_item = CompletionItem {
                label: name.clone(),
                kind: Some(lsp_types::CompletionItemKind::FOLDER),
                filter_text: Some(filter_text.clone()),
                text_edit: Some(CompletionTextEdit::Edit(text_edit)),
                ..Default::default()
            };

            module_completions.push(completion_item);
        }
    }

    let _ = module_info;
    for completion_item in module_completions {
        builder.add_completion_item(completion_item)?;
    }
    builder.stop_here();

    Some(())
}
