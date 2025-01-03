use code_analysis::{EmmyrcFilenameConvention, ModuleInfo};
use emmylua_parser::{LuaAstNode, LuaNameExpr};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let name_expr = LuaNameExpr::cast(builder.trigger_token.parent()?)?;
    // optimize for large project
    let prefix = name_expr.get_name_text()?.to_lowercase();
    let file_convension = builder
        .semantic_model
        .get_emmyrc()
        .completion
        .auto_require_naming_convention;
    let file_id = builder.semantic_model.get_file_id();
    let module_infos = builder
        .semantic_model
        .get_db()
        .get_module_index()
        .get_module_infos();

    let mut completions = Vec::new();
    for module_info in module_infos {
        if module_info.visible
            && module_info.file_id != file_id
            && module_info.export_type.is_some()
        {
            add_module_completion_item(
                builder,
                &prefix,
                &module_info,
                file_convension,
                &mut completions,
            );
        }
    }

    for completion in completions {
        builder.add_completion_item(completion);
    }

    Some(())
}

fn add_module_completion_item(
    builder: &CompletionBuilder,
    prefix: &str,
    module_info: &ModuleInfo,
    file_convension: EmmyrcFilenameConvention,
    completions: &mut Vec<CompletionItem>,
) -> Option<()> {
    let completion_name = module_name_convert(&module_info.name, file_convension);
    if !completion_name.to_lowercase().starts_with(prefix) {
        return None;
    }

    if builder.env_duplicate_name.contains(&completion_name) {
        return None;
    }

    let completion_item = CompletionItem {
        label: completion_name,
        kind: Some(lsp_types::CompletionItemKind::MODULE),
        label_details: Some(lsp_types::CompletionItemLabelDetails {
            detail: Some(format!("    (in {})", module_info.full_module_name)),
            ..Default::default()
        }),
        ..Default::default()
    };

    completions.push(completion_item);

    Some(())
}

fn module_name_convert(name: &str, file_convension: EmmyrcFilenameConvention) -> String {
    let mut module_name = name.to_string();

    match file_convension {
        EmmyrcFilenameConvention::SnakeCase => {
            module_name = to_snake_case(&module_name);
        }
        EmmyrcFilenameConvention::CamelCase => {
            module_name = to_camel_case(&module_name);
        }
        EmmyrcFilenameConvention::PascalCase => {
            module_name = to_pascal_case(&module_name);
        }
        EmmyrcFilenameConvention::Keep => {}
    }

    module_name
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            result.push('_');
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }
    result
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut next_uppercase = false;
    for (i, ch) in s.chars().enumerate() {
        if ch == '_' || ch == '-' || ch == '.' {
            next_uppercase = true;
        } else if next_uppercase {
            result.push(ch.to_ascii_uppercase());
            next_uppercase = false;
        } else if i == 0 {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut next_uppercase = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' || ch == '.' {
            next_uppercase = true;
        } else if next_uppercase {
            result.push(ch.to_ascii_uppercase());
            next_uppercase = false;
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }
    result
}
