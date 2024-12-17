use code_analysis::{LuaDeclId, LuaPropertyOwnerId, LuaType};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

use super::{check_visibility, get_completion_kind, get_description, get_detail, is_deprecated};

pub fn add_decl_completion(builder: &mut CompletionBuilder, decl_id: LuaDeclId) -> Option<()> {
    let property_owner = LuaPropertyOwnerId::LuaDecl(decl_id);
    check_visibility(builder, property_owner.clone())?;

    let (name, typ) = {
        let decl = builder
            .semantic_model
            .get_db()
            .get_decl_index()
            .get_decl(&decl_id)?;
        (
            decl.get_name(),
            decl.get_type().cloned().unwrap_or(LuaType::Unknown),
        )
    };

    let mut completion_item = CompletionItem {
        label: name.to_string(),
        kind: Some(get_completion_kind(&typ)),
        data: Some(property_owner.to_string().into()),
        label_details: Some(lsp_types::CompletionItemLabelDetails {
            detail: get_detail(builder, &property_owner, &typ),
            description: get_description(builder, &typ)
        }),
        ..Default::default()
    };

    if is_deprecated(builder, property_owner.clone()) {
        completion_item.deprecated = Some(true);
    }

    builder.add_completion_item(completion_item)?;
    Some(())
}
