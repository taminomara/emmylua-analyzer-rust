mod add_decl_completion;

pub use add_decl_completion::add_decl_completion;
use code_analysis::{LuaPropertyId, LuaPropertyOwnerId, LuaType};
use lsp_types::CompletionItemKind;

use crate::util::humanize_type;

use super::completion_builder::CompletionBuilder;

fn check_visibility(builder: &CompletionBuilder, id: LuaPropertyOwnerId) -> Option<()> {
    match id {
        LuaPropertyOwnerId::Member(_) => {}
        _ => return Some(()),
    }

    let property = builder
        .semantic_model
        .get_db()
        .get_property_index()
        .get_property(id);
    if property.is_none() {
        return Some(());
    }
    // let decl = property.unwrap();
    // if let Some(visib) = &decl.visibility {
    // }
    // todo check

    Some(())
}

fn get_completion_kind(typ: &LuaType) -> CompletionItemKind {
    if typ.is_function() {
        return CompletionItemKind::FUNCTION;
    } else if typ.is_const() {
        return CompletionItemKind::CONSTANT;
    } else if typ.is_def() {
        return CompletionItemKind::CLASS;
    }

    CompletionItemKind::VARIABLE
}

fn is_deprecated(builder: &CompletionBuilder, id: LuaPropertyOwnerId) -> bool {
    let property = builder
        .semantic_model
        .get_db()
        .get_property_index()
        .get_property(id);
    if property.is_none() {
        return false;
    }

    property.unwrap().is_deprecated
}

fn get_detail(
    builder: &CompletionBuilder,
    property_owner_id: &LuaPropertyOwnerId,
    typ: &LuaType,
) -> Option<String> {
    if let LuaType::Signature(signature_id) = typ {
        let signature = builder
            .semantic_model
            .get_db()
            .get_signature_index()
            .get(&signature_id)?;

        let params_str = signature
            .get_type_params()
            .iter()
            .map(|param| param.0.clone())
            .collect::<Vec<_>>();

        return Some(format!("({})", params_str.join(", ")));
    } else {
        // show comment in detail
        let property = builder
            .semantic_model
            .get_db()
            .get_property_index()
            .get_property(property_owner_id.clone())?;

        if let Some(detail) = &property.description {
            return Some(truncate_with_ellipsis(detail, 25));
        }
    }
    None
}

fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let truncated: String = s.chars().take(max_len).collect();
        format!("   {}...", truncated)
    } else {
        format!("   {}", s)
    }
}

fn get_description(builder: &CompletionBuilder, typ: &LuaType) -> Option<String> {
    if let LuaType::Signature(signature_id) = typ {
        let signature = builder
            .semantic_model
            .get_db()
            .get_signature_index()
            .get(&signature_id)?;
        let rets = &signature.return_docs;
        if rets.len() == 1 {
            let detail = humanize_type(builder.semantic_model.get_db(), &rets[0].type_ref);
            Some(detail)
        } else if rets.len() > 1 {
            let detail = humanize_type(builder.semantic_model.get_db(), &rets[0].type_ref);
            Some(format!("{} ...", detail))
        } else {
            None
        }
    } else if typ.is_unknown() {
        return None;
    } else {
        Some(humanize_type(builder.semantic_model.get_db(), typ))
    }
}
