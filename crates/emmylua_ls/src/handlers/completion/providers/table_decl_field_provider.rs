use std::collections::HashSet;

use emmylua_code_analysis::{LuaMemberInfo, LuaMemberKey};
use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaNameExpr, LuaNameToken, LuaTableExpr, LuaTableField,
};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    let table_expr = LuaNameToken::cast(builder.trigger_token.clone())?
        .get_parent::<LuaNameExpr>()?
        .get_parent::<LuaTableField>()?
        .get_parent::<LuaTableExpr>()?;

    // todo non-function completion (e.g. in other tables)
    // todo support parents which types are inferred implicitly
    let table_type = builder.semantic_model.infer_table_should_be(table_expr)?;

    let mut duplicated_set = HashSet::new();
    let member_infos = builder.semantic_model.infer_member_infos(&table_type)?;
    for member_info in member_infos {
        if duplicated_set.contains(&member_info.key) {
            continue;
        }

        duplicated_set.insert(member_info.key.clone());
        add_table_field_completion(builder, member_info);
    }

    Some(())
}

fn add_table_field_completion(
    builder: &mut CompletionBuilder,
    member_info: LuaMemberInfo,
) -> Option<()> {
    let env_completion = &builder.env_duplicate_name;
    let name = match member_info.key {
        LuaMemberKey::Name(name) => name.to_string(),
        LuaMemberKey::Integer(index) => format!("[{}]", index),
        _ => return None,
    };

    let label = if env_completion.contains(&name) {
        format!("{0} = {0},", name)
    } else {
        format!("{} = ", name)
    };

    let completion_item = CompletionItem {
        label,
        sort_text: Some("".to_string()),
        kind: Some(lsp_types::CompletionItemKind::FIELD),
        ..CompletionItem::default()
    };

    builder.add_completion_item(completion_item);
    Some(())
}
