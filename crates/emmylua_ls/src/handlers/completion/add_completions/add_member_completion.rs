use code_analysis::{LuaMemberInfo, LuaMemberKey};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

use super::{check_visibility, get_completion_kind, get_description, get_detail, is_deprecated};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompletionTriggerStatus {
    Dot,
    Colon,
    InString,
    LeftBracket,
}

pub fn add_member_completion(
    builder: &mut CompletionBuilder,
    member_info: LuaMemberInfo,
    status: CompletionTriggerStatus,
) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }
    let property_owner = member_info.property_owner_id;
    if let Some(property_owner) = &property_owner {
        check_visibility(builder, property_owner.clone())?;
    }

    let member_key = &member_info.key;
    let label = match status {
        CompletionTriggerStatus::Dot => match member_key {
            LuaMemberKey::Name(name) => name.to_string(),
            LuaMemberKey::Integer(index) => index.to_string(),
            _ => return None,
        },
        CompletionTriggerStatus::Colon => match member_key {
            LuaMemberKey::Name(name) => name.to_string(),
            _ => return None,
        },
        CompletionTriggerStatus::InString => match member_key {
            LuaMemberKey::Name(name) => name.to_string(),
            _ => return None,
        },
        CompletionTriggerStatus::LeftBracket => match member_key {
            LuaMemberKey::Name(name) => format!("[{}]", name.to_string()),
            LuaMemberKey::Integer(index) => format!("[{}]", index),
            _ => return None,
        },
    };

    let typ = member_info.typ;

    let data = if let Some(id) = &property_owner {
        Some(id.to_string().into())
    } else {
        None
    };

    let detail = if let Some(id) = &property_owner {
        get_detail(builder, id, &typ)
    } else {
        None
    };

    let description = get_description(builder, &typ);

    let deprecated = if let Some(id) = &property_owner {
        Some(is_deprecated(builder, id.clone()))
    } else {
        None
    };

    let completion_item = CompletionItem {
        label,
        kind: Some(get_completion_kind(&typ)),
        data,
        label_details: Some(lsp_types::CompletionItemLabelDetails {
            detail,
            description,
        }),
        deprecated,
        ..Default::default()
    };

    builder.add_completion_item(completion_item)?;

    Some(())
}
