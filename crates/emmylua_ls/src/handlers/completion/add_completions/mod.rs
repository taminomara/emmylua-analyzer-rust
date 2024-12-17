mod add_decl_completion;

pub use add_decl_completion::add_decl_completion;
use code_analysis::{LuaPropertyOwnerId, LuaType};
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

fn get_detail(builder: &CompletionBuilder, typ: &LuaType) -> Option<String> {
    // if let LuaType::Signature(signature_id) = typ {
    //     let signature = builder
    //         .semantic_model
    //         .get_db()
    //         .get_signature_index()
    //         .get(&signature_id)?;
    //     let rets = &signature.return_docs;
    //     if rets.len() == 1 {
    //         let detail = humanize_type(builder.semantic_model.get_db(), &rets[0].type_ref);
    //         Some(detail)
    //     } else {
    //         let detail = humanize_type(builder.semantic_model.get_db(), &rets[0].type_ref);
    //         Some(format!("{} ...", detail))
    //     }
    // } else if typ.is_unknown() {
    //     return None;
    // } else {
    //     Some(humanize_type(builder.semantic_model.get_db(), typ))
    // }
    None
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
