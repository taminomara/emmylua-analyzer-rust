use emmylua_code_analysis::LuaTypeDeclId;
use emmylua_parser::{LuaAstNode, LuaDocNameType};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    if !LuaDocNameType::can_cast(builder.trigger_token.parent()?.kind().into()) {
        return None;
    }

    let prefix_content = builder.trigger_token.text().to_string();
    let prefix = if let Some(last_sep) = prefix_content.rfind(|c| c == '.') {
        let (path, _) = prefix_content.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let file_id = builder.semantic_model.get_file_id();
    let type_index = builder.semantic_model.get_db().get_type_index();
    let results = type_index.find_type_decls(file_id, prefix);

    for (name, type_decl) in results {
        add_type_completion_item(builder, &name, type_decl);
    }
    builder.stop_here();

    Some(())
}

fn add_type_completion_item(
    builder: &mut CompletionBuilder,
    name: &str,
    type_decl: Option<LuaTypeDeclId>,
) -> Option<()> {
    let kind = match type_decl {
        Some(_) => lsp_types::CompletionItemKind::CLASS,
        None => lsp_types::CompletionItemKind::MODULE,
    };

    let completion_item = CompletionItem {
        label: name.to_string(),
        kind: Some(kind),
        ..CompletionItem::default()
    };

    builder.add_completion_item(completion_item)
}
