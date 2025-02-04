use emmylua_code_analysis::{
    DbIndex, LuaDeclId, LuaDocument, LuaMemberId, LuaMemberKey, LuaMemberOwner, LuaPropertyOwnerId,
    LuaSignatureId, LuaType, LuaTypeDeclId, SemanticInfo,
};
use emmylua_parser::LuaSyntaxToken;
use lsp_types::{Hover, HoverContents, MarkedString, MarkupContent};

use emmylua_code_analysis::humanize_type;

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
        LuaPropertyOwnerId::Member(member_id) => {
            build_member_hover(db, document, token, typ, member_id)
        }
        LuaPropertyOwnerId::TypeDecl(type_decl_id) => {
            build_type_decl_hover(db, document, token, type_decl_id)
        }
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
    add_description(db, &mut marked_strings, property_owner);

    if let LuaType::Signature(signature_id) = typ {
        add_signature_description(db, &mut marked_strings, signature_id);
    }

    Some(Hover {
        contents: HoverContents::Array(marked_strings),
        range: document.to_lsp_range(token.text_range()),
    })
}

fn build_member_hover(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    typ: LuaType,
    member_id: LuaMemberId,
) -> Option<Hover> {
    let mut marked_strings = Vec::new();
    let member = db.get_member_index().get_member(&member_id)?;

    let member_name = match member.get_key() {
        LuaMemberKey::Name(name) => name.to_string(),
        LuaMemberKey::Integer(i) => format!("[{}]", i),
        _ => return None,
    };

    if typ.is_function() {
        let hover_text = hover_function_type(db, &typ, &member_name, false);
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            hover_text,
        ));

        if let LuaMemberOwner::Type(ty) = &member.get_owner() {
            marked_strings.push(MarkedString::from_markdown(format!(
                "in class `{}`",
                ty.get_name()
            )));
        }
    } else if typ.is_const() {
        let const_value = hover_const_type(db, &typ);
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("(field) {}: {}", member_name, const_value),
        ));
    } else {
        let type_humanize_text = humanize_type(db, &typ);
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("(field) {}: {}", member_name, type_humanize_text),
        ));
    }

    add_description(
        db,
        &mut marked_strings,
        LuaPropertyOwnerId::Member(member_id),
    );

    if let LuaType::Signature(signature_id) = typ {
        add_signature_description(db, &mut marked_strings, signature_id);
    }

    Some(Hover {
        contents: HoverContents::Array(marked_strings),
        range: document.to_lsp_range(token.text_range()),
    })
}

fn add_description(
    db: &DbIndex,
    marked_strings: &mut Vec<MarkedString>,
    property_owner: LuaPropertyOwnerId,
) {
    if let Some(property) = db.get_property_index().get_property(property_owner.clone()) {
        if let Some(detail) = &property.description {
            marked_strings.push(MarkedString::from_markdown(detail.to_string()));
        }
    }
}

fn add_signature_description(
    db: &DbIndex,
    marked_strings: &mut Vec<MarkedString>,
    signature_id: LuaSignatureId,
) -> Option<()> {
    let signature = db.get_signature_index().get(&signature_id)?;
    let param_count = signature.params.len();
    let mut s = String::new();
    for i in 0..param_count {
        let param_info = match signature.get_param_info_by_id(i) {
            Some(info) => info,
            None => continue,
        };

        if let Some(description) = &param_info.description {
            s.push_str(&format!("@param `{}`", param_info.name));
            s.push_str(&format!(" - {}", description));
            s.push_str("\n");
        }
    }

    if !s.is_empty() {
        marked_strings.push(MarkedString::from_markdown(s));
    }
    Some(())
}

fn build_type_decl_hover(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    type_decl_id: LuaTypeDeclId,
) -> Option<Hover> {
    let mut marked_strings = Vec::new();
    let type_decl = db.get_type_index().get_type_decl(&type_decl_id)?;
    if type_decl.is_alias() {
        if let Some(origin) = type_decl.get_alias_origin(db, None) {
            let origin_type = humanize_type(db, &origin);
            marked_strings.push(MarkedString::from_language_code(
                "lua".to_string(),
                format!("(type alias) {} = {}", type_decl.get_name(), origin_type),
            ));
        } else {
            marked_strings.push(MarkedString::from_language_code(
                "lua".to_string(),
                format!("(type alias) {}", type_decl.get_name()),
            ));

            let mut s = String::new();
            let member_ids = type_decl.get_alias_union_members()?;
            for member_id in member_ids {
                let member = db.get_member_index().get_member(&member_id)?;
                let type_humanize_text = humanize_type(db, &member.get_decl_type());
                let property_owner = LuaPropertyOwnerId::Member(member_id.clone());
                let description = db
                    .get_property_index()
                    .get_property(property_owner)
                    .and_then(|p| p.description.clone());
                if let Some(description) = description {
                    s.push_str(&format!(
                        "    | {}  --{}\n",
                        type_humanize_text, description
                    ));
                } else {
                    s.push_str(&format!("    | {}\n", type_humanize_text));
                }
            }

            marked_strings.push(MarkedString::from_language_code("lua".to_string(), s));
        }
    } else if type_decl.is_enum() {
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("(enum) {}", type_decl.get_name()),
        ));
    } else {
        marked_strings.push(MarkedString::from_language_code(
            "lua".to_string(),
            format!("(class) {}", type_decl.get_name()),
        ));
    }

    let property_owner = LuaPropertyOwnerId::TypeDecl(type_decl_id);
    add_description(db, &mut marked_strings, property_owner);

    Some(Hover {
        contents: HoverContents::Array(marked_strings),
        range: document.to_lsp_range(token.text_range()),
    })
}
