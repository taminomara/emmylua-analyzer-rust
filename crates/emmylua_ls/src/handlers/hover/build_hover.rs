use code_analysis::{
    humanize_type, DbIndex, LuaDeclId, LuaDocument, LuaPropertyOwnerId, LuaType, SemanticInfo,
};
use emmylua_parser::LuaSyntaxToken;
use lsp_types::{Hover, HoverContents, MarkedString, MarkupContent};

use super::hover_humanize::{hover_const_type, hover_function_type};

pub fn build_semantic_info_hover(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    semantic_info: SemanticInfo,
) -> Option<Hover> {
    let typ = semantic_info.typ;
    if semantic_info.property_owner.is_none() {
        return build_hover_without_property(db, document, token, typ);
    }

    match semantic_info.property_owner.unwrap() {
        LuaPropertyOwnerId::LuaDecl(decl_id) => build_decl_hover(db, document, token, typ, decl_id),

        _ => None,
    }
}

fn build_hover_without_property(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    typ: LuaType,
) -> Option<Hover> {
    let hover = humanize_type(db, &typ);
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: hover,
        }),
        range: document.to_lsp_range(token.text_range()),
    })
}

fn build_decl_hover(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    typ: LuaType,
    decl_id: LuaDeclId,
) -> Option<Hover> {
    let mut marked_strings = Vec::new();
    let decl = db.get_decl_index().get_decl(&decl_id)?;
    if typ.is_function() {
        let hover_text = hover_function_type(db, &typ, decl.get_name(), decl.is_local());
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            hover_text,
        ));
    } else if typ.is_const() {
        let const_value = hover_const_type(db, &typ);
        let prefix = if decl.is_local() {
            "local "
        } else {
            "(global) "
        };
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("{}{}: {}", prefix, decl.get_name(), const_value),
        ));
    } else {
        let type_humanize_text = humanize_type(db, &typ);
        let prefix = if decl.is_local() {
            "local "
        } else {
            "(global) "
        };
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("{}{}: {}", prefix, decl.get_name(), type_humanize_text),
        ));
    }

    let property_owner = LuaPropertyOwnerId::LuaDecl(decl_id);
    add_description(db, &mut marked_strings, &typ, property_owner);

    Some(Hover {
        contents: HoverContents::Array(marked_strings),
        range: document.to_lsp_range(token.text_range()),
    })
}

fn add_description(
    db: &DbIndex,
    marked_strings: &mut Vec<MarkedString>,
    typ: &LuaType,
    property_owner: LuaPropertyOwnerId,
) {
    if let Some(property) = db.get_property_index().get_property(property_owner) {
        if let Some(detail) = &property.description {
            marked_strings.push(MarkedString::from_markdown(detail.to_string()));
        }
    }

    if let LuaType::Signature(signature_id) = typ {
        let property_owner = LuaPropertyOwnerId::Signature(signature_id.clone());
        if let Some(property) = db.get_property_index().get_property(property_owner) {
            if let Some(detail) = &property.description {
                marked_strings.push(MarkedString::from_markdown(detail.to_string()));
            }
        }
    }
}
