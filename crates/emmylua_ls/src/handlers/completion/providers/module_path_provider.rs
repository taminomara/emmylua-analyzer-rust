use code_analysis::Emmyrc;
use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaExpr, LuaLiteralExpr, LuaStringToken,
};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let string_token = LuaStringToken::cast(builder.trigger_token.clone())?;
    let call_expr_prefix = string_token
        .get_parent::<LuaLiteralExpr>()?
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?
        .get_prefix_expr()?;

    match call_expr_prefix {
        LuaExpr::NameExpr(name_expr) => {
            let name = name_expr.get_name_text()?;
            if !is_require_call(builder.semantic_model.get_emmyrc(), &name) {
                return None;
            }
        }
        _ => return None,
    }

    let prefix_content = string_token.get_value();
    let parts: Vec<&str> = prefix_content
        .split(|c| c == '.' || c == '/' || c == '\\')
        .collect();
    let module_path = if parts.len() > 1 {
        parts[..parts.len() - 1].join(".")
    } else {
        "".to_string()
    };

    let prefix = if let Some(last_sep) = prefix_content.rfind(|c| c == '/' || c == '\\' || c == '.') {
        let (path, _) = prefix_content.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let db = builder.semantic_model.get_db();
    let mut module_completions = Vec::new();
    let module_info = db.get_module_index().find_module_node(&module_path)?;
    for (name, module_id) in &module_info.children {
        eprintln!("name: {}, module_id: {:?}", name, module_id);
        let child_module_node = db.get_module_index().get_module_node(module_id)?;
        let child_file_id = child_module_node.file_ids.first()?;
        let child_module_info = db.get_module_index().get_module(*child_file_id)?;
        let uri = db.get_vfs().get_uri(child_file_id)?;
        let filter_text = format!("{}{}", prefix, name);
        let completion_item = CompletionItem {
            label: name.clone(),
            kind: Some(lsp_types::CompletionItemKind::MODULE),
            filter_text: Some(filter_text.clone()),
            insert_text: Some(filter_text),
            label_details: Some(lsp_types::CompletionItemLabelDetails {
                detail: Some(format!("  (in {})", child_module_info.full_module_name)),
                description: Some(format!("{}", uri.as_str())),
            }),
            ..Default::default()
        };

        eprintln!("completion_item: {:?}", completion_item);
        module_completions.push(completion_item);
    }

    let _ = module_info;
    for completion_item in module_completions {
        builder.add_completion_item(completion_item)?;
    }

    Some(())
}

fn is_require_call(emmyrc: &Emmyrc, name: &str) -> bool {
    if let Some(runtime) = &emmyrc.runtime {
        if let Some(funs) = &runtime.require_like_function {
            for fun in funs {
                if name == fun {
                    return true;
                }
            }
        }
    }

    name == "require"
}
