use code_analysis::{DbIndex, LuaMemberInfo, LuaMemberKey, LuaType};
use lsp_types::CompletionItem;

use crate::handlers::completion::completion_builder::CompletionBuilder;

use super::{
    check_visibility, get_completion_kind, get_description, get_detail, is_deprecated, CallDisplay,
    CompletionData,
};

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
    let property_owner = &member_info.property_owner_id;
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
            LuaMemberKey::Name(name) => format!("\"{}\"", name.to_string()),
            LuaMemberKey::Integer(index) => format!("{}", index),
            _ => return None,
        },
    };

    let display = get_call_show(
        builder.semantic_model.get_db(),
        member_info.get_origin_type(),
        status,
    )
    .unwrap_or(CallDisplay::None);

    let typ = member_info.typ;
    if status == CompletionTriggerStatus::Colon && !typ.is_function() {
        return None;
    }

    let data = if let Some(id) = &property_owner {
        CompletionData::from_property_owner_id(id.clone().into())
    } else {
        None
    };

    let detail = if let Some(id) = &property_owner {
        get_detail(builder, id, &typ, display)
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

fn get_call_show(
    db: &DbIndex,
    typ: &LuaType,
    status: CompletionTriggerStatus,
) -> Option<CallDisplay> {
    let (colon_call, colon_define) = match typ {
        LuaType::Signature(sig_id) => {
            let signature = db.get_signature_index().get(sig_id)?;
            let colon_define = signature.is_colon_define;
            let colon_call = status == CompletionTriggerStatus::Colon;
            (colon_call, colon_define)
        }
        LuaType::DocFunction(func) => {
            let colon_define = func.is_colon_define();
            let colon_call = status == CompletionTriggerStatus::Colon;
            (colon_call, colon_define)
        }
        _ => return None,
    };

    match (colon_call, colon_define) {
        (false, true) => Some(CallDisplay::AddSelf),
        (true, false) => Some(CallDisplay::RemoveFirst),
        _ => Some(CallDisplay::None),
    }
}
