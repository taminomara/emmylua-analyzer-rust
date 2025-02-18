use emmylua_code_analysis::{
    DbIndex, LuaDecl, LuaDeclId, LuaDocument, LuaMember, LuaMemberId, LuaMemberKey, LuaMemberOwner,
    LuaPropertyOwnerId, LuaSignatureId, LuaType, LuaTypeDeclId, RenderLevel, SemanticInfo,
    SemanticModel,
};
use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaSyntaxKind, LuaSyntaxToken, LuaTableField};
use lsp_types::{Hover, HoverContents, MarkedString, MarkupContent};

use emmylua_code_analysis::humanize_type;

use super::hover_humanize::{hover_const_type, hover_function_type};

pub struct HoverContent {
    /// 类型签名
    pub type_signature: MarkedString,
    /// 所在路径
    pub location_path: Option<MarkedString>,
    /// 详细描述
    pub detailed_description: Vec<MarkedString>,
}

pub fn build_semantic_info_hover(
    semantic_model: &SemanticModel,
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    semantic_info: SemanticInfo,
) -> Option<Hover> {
    let typ = semantic_info.clone().typ;
    if semantic_info.property_owner.is_none() {
        return build_hover_without_property(db, document, token, typ);
    }

    let hover_content = build_hover_content(
        Some(semantic_model),
        db,
        Some(typ),
        semantic_info.property_owner.unwrap(),
        true,
    );
    if let Some(hover_content) = hover_content {
        build_hover_result(hover_content, document.to_lsp_range(token.text_range()))
    } else {
        None
    }
}

fn build_hover_without_property(
    db: &DbIndex,
    document: &LuaDocument,
    token: LuaSyntaxToken,
    typ: LuaType,
) -> Option<Hover> {
    let hover = humanize_type(db, &typ, RenderLevel::Detailed);
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: hover,
        }),
        range: document.to_lsp_range(token.text_range()),
    })
}

pub fn build_hover_content(
    semantic_model: Option<&SemanticModel>,
    db: &DbIndex,
    typ: Option<LuaType>,
    property_id: LuaPropertyOwnerId,
    fun_ret_newline: bool, // 是否将函数签名的返回值显示为独立行
) -> Option<HoverContent> {
    match property_id {
        LuaPropertyOwnerId::LuaDecl(decl_id) => {
            if let Some(semantic_model) = semantic_model {
                if let Some(typ) = typ {
                    build_decl_hover(semantic_model, db, typ, decl_id, fun_ret_newline)
                } else {
                    let decl = db.get_decl_index().get_decl(&decl_id)?;
                    let typ = decl.get_type()?;
                    build_decl_hover(semantic_model, db, typ.clone(), decl_id, fun_ret_newline)
                }
            } else {
                None
            }
        }
        LuaPropertyOwnerId::Member(member_id) => {
            if let Some(semantic_model) = semantic_model {
                if let Some(typ) = typ {
                    build_member_hover(semantic_model, db, typ, member_id, fun_ret_newline)
                } else {
                    let member = db.get_member_index().get_member(&member_id)?;
                    let typ = member.get_decl_type();
                    build_member_hover(semantic_model, db, typ.clone(), member_id, fun_ret_newline)
                }
            } else {
                None
            }
        }
        LuaPropertyOwnerId::TypeDecl(type_decl_id) => build_type_decl_hover(db, type_decl_id),
        _ => None,
    }
}

fn build_decl_hover(
    semantic_model: &SemanticModel,
    db: &DbIndex,
    typ: LuaType,
    decl_id: LuaDeclId,
    fun_ret_newline: bool,
) -> Option<HoverContent> {
    let decl = db.get_decl_index().get_decl(&decl_id)?;

    let type_signature;
    let mut location_path = None;
    let mut detailed_description = Vec::new();

    let mut owner_member = None;
    let mut owner_decl = None;

    // 处理类型签名
    if typ.is_function() {
        let property_owner: Option<LuaPropertyOwnerId> = get_decl_owner(semantic_model, &decl);
        match property_owner {
            Some(LuaPropertyOwnerId::Member(member_id)) => {
                owner_member = Some(db.get_member_index().get_member(&member_id).unwrap());
            }
            Some(LuaPropertyOwnerId::LuaDecl(decl_id)) => {
                owner_decl = Some(db.get_decl_index().get_decl(&decl_id).unwrap());
            }
            _ => {}
        }
        let hover_text =
            hover_function_type(db, &typ, owner_member, decl.get_name(), fun_ret_newline);
        type_signature = MarkedString::from_language_code("lua".to_string(), hover_text);
        if let Some(owner_member) = owner_member {
            if let LuaMemberOwner::Type(ty) = &owner_member.get_owner() {
                if ty.get_name() != ty.get_simple_name() {
                    location_path = Some(MarkedString::from_markdown(format!(
                        "{}{} `{}`",
                        "&nbsp;&nbsp;",
                        "in class",
                        ty.get_name()
                    )));
                }
            }
        }
        detailed_description.push(MarkedString::String("---".to_string()));
    } else if typ.is_const() {
        let const_value = hover_const_type(db, &typ);
        let prefix = if decl.is_local() {
            "local "
        } else {
            "(global) "
        };
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("{}{}: {}", prefix, decl.get_name(), const_value),
        );
    } else {
        let type_humanize_text = humanize_type(db, &typ, RenderLevel::Detailed);
        let prefix = if decl.is_local() {
            "local "
        } else {
            "(global) "
        };
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("{}{}: {}", prefix, decl.get_name(), type_humanize_text),
        );
    }

    // 处理其他
    let property_owner = LuaPropertyOwnerId::LuaDecl(decl_id);
    // 如果`decl`没有描述, 则尝试从`owner_member/owner_decl`获取描述
    if !add_description(db, &mut detailed_description, property_owner) {
        if let Some(owner_member) = owner_member {
            add_description(
                db,
                &mut detailed_description,
                LuaPropertyOwnerId::Member(owner_member.get_id()),
            );
        } else if let Some(owner_decl) = owner_decl {
            add_description(
                db,
                &mut detailed_description,
                LuaPropertyOwnerId::LuaDecl(owner_decl.get_id()),
            );
        }
    }

    if let LuaType::Signature(signature_id) = typ {
        add_signature_description(db, &mut detailed_description, signature_id);
        if !fun_ret_newline {
            add_signature_ret_description(db, &mut detailed_description, signature_id);
        }
    }

    Some(HoverContent {
        type_signature,
        location_path,
        detailed_description,
    })
}

fn build_member_hover(
    semantic_model: &SemanticModel,
    db: &DbIndex,
    typ: LuaType,
    member_id: LuaMemberId,
    fun_ret_newline: bool,
) -> Option<HoverContent> {
    let member = db.get_member_index().get_member(&member_id)?;
    let member_name = match member.get_key() {
        LuaMemberKey::Name(name) => name.to_string(),
        LuaMemberKey::Integer(i) => format!("[{}]", i),
        _ => return None,
    };

    let type_signature;
    let mut location_path = None;
    let mut detailed_description = Vec::new();

    let mut function_member = None;
    if typ.is_function() {
        function_member = get_member_function_member(semantic_model, member_id);
        let hover_text = hover_function_type(
            db,
            &typ,
            function_member.or_else(|| Option::from(member)),
            &member_name,
            fun_ret_newline,
        );

        type_signature = MarkedString::from_language_code("lua".to_string(), hover_text);

        let valid_member = function_member.as_ref().unwrap_or(&member);
        if let LuaMemberOwner::Type(ty) = &valid_member.get_owner() {
            if ty.get_name() != ty.get_simple_name() {
                location_path = Some(MarkedString::from_markdown(format!(
                    "{}{} `{}`",
                    "&nbsp;&nbsp;",
                    "in class",
                    ty.get_name()
                )));
            }
        }
        detailed_description.push(MarkedString::String("---".to_string()));
    } else if typ.is_const() {
        let const_value = hover_const_type(db, &typ);
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("(field) {}: {}", member_name, const_value),
        );
    } else {
        let type_humanize_text = humanize_type(db, &typ, RenderLevel::Simple);
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("(field) {}: {}", member_name, type_humanize_text),
        );
    }

    // 如果`decl`没有描述, 则从`owner_member`获取描述
    if !add_description(
        db,
        &mut detailed_description,
        LuaPropertyOwnerId::Member(member_id),
    ) {
        if let Some(owner_member) = function_member {
            add_description(
                db,
                &mut detailed_description,
                LuaPropertyOwnerId::Member(owner_member.get_id()),
            );
        }
    }

    if let LuaType::Signature(signature_id) = typ {
        add_signature_description(db, &mut detailed_description, signature_id);
        if !fun_ret_newline {
            add_signature_ret_description(db, &mut detailed_description, signature_id);
        }
    }

    Some(HoverContent {
        type_signature,
        location_path,
        detailed_description,
    })
}

fn build_type_decl_hover(db: &DbIndex, type_decl_id: LuaTypeDeclId) -> Option<HoverContent> {
    let type_decl = db.get_type_index().get_type_decl(&type_decl_id)?;
    let type_signature;
    let mut detailed_description = Vec::new();
    if type_decl.is_alias() {
        if let Some(origin) = type_decl.get_alias_origin(db, None) {
            let origin_type = humanize_type(db, &origin, RenderLevel::Detailed);
            type_signature = MarkedString::from_language_code(
                "lua".to_string(),
                format!("(type alias) {} = {}", type_decl.get_name(), origin_type),
            );
        } else {
            let mut s = String::new();
            s.push_str(&format!("(type alias) {}\n", type_decl.get_name()));
            let member_ids = type_decl.get_alias_union_members()?;
            for member_id in member_ids {
                let member = db.get_member_index().get_member(&member_id)?;
                let type_humanize_text =
                    humanize_type(db, &member.get_decl_type(), RenderLevel::Minimal);
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
            type_signature = MarkedString::from_language_code("lua".to_string(), s);
        }
    } else if type_decl.is_enum() {
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("(enum) {}", type_decl.get_name()),
        );
    } else {
        let humanize_text = humanize_type(
            db,
            &LuaType::Def(type_decl_id.clone()),
            RenderLevel::Detailed,
        );
        type_signature = MarkedString::from_language_code(
            "lua".to_string(),
            format!("(class) {}", humanize_text),
        );
    }

    let property_owner = LuaPropertyOwnerId::TypeDecl(type_decl_id);
    add_description(db, &mut detailed_description, property_owner);

    Some(HoverContent {
        type_signature,
        location_path: None,
        detailed_description,
    })
}

fn add_description(
    db: &DbIndex,
    marked_strings: &mut Vec<MarkedString>,
    property_owner: LuaPropertyOwnerId,
) -> bool {
    let mut has_description = false;
    if let Some(property) = db.get_property_index().get_property(property_owner.clone()) {
        if let Some(detail) = &property.description {
            has_description = true;
            marked_strings.push(MarkedString::from_markdown(detail.to_string()));
        }
    }
    has_description
}

pub fn add_signature_description(
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
            s.push_str(&format!(
                "@*param* `{}` — {}\n",
                param_info.name, description
            ));
        }
    }

    if !s.is_empty() {
        marked_strings.push(MarkedString::from_markdown(s));
    }
    Some(())
}

pub fn add_signature_ret_description(
    db: &DbIndex,
    marked_strings: &mut Vec<MarkedString>,
    signature_id: LuaSignatureId,
) -> Option<()> {
    let signature = db.get_signature_index().get(&signature_id)?;
    let mut s = String::new();
    for i in 0..signature.return_docs.len() {
        let ret_info = &signature.return_docs[i];
        if let Some(description) = ret_info.description.clone() {
            s.push_str(&format!(
                "@*return* {} — {}\n",
                match &ret_info.name {
                    Some(name) if !name.is_empty() => format!("`{}` ", name),
                    _ => "".to_string(),
                },
                description
            ));
        }
    }
    marked_strings.push(MarkedString::from_markdown(s));
    Some(())
}

// 获取`decl`可能的来源
fn get_decl_owner<'a>(
    semantic_model: &SemanticModel,
    decl: &LuaDecl,
) -> Option<LuaPropertyOwnerId> {
    let root = semantic_model
        .get_db()
        .get_vfs()
        .get_syntax_tree(&decl.get_file_id())?
        .get_red_root();
    let node = decl.get_value_syntax_id()?.to_node_from_root(&root)?;
    semantic_model.get_property_owner_id(node.into())
}

/*
-- 处理以下情况
local A = {
    b = Class.MethodA -- hover b 时类型为 Class.MethodA
}
A.c, A.d = Class.MethodA, Class.MethodB
 */
fn get_member_function_member<'a>(
    semantic_model: &'a SemanticModel,
    member_id: LuaMemberId,
) -> Option<&'a LuaMember> {
    let root = semantic_model
        .get_db()
        .get_vfs()
        .get_syntax_tree(&member_id.file_id)?
        .get_red_root();
    let cur_node = member_id.get_syntax_id().to_node_from_root(&root)?;

    match member_id.get_syntax_id().get_kind() {
        LuaSyntaxKind::TableFieldAssign => match cur_node {
            table_field_node if LuaTableField::can_cast(table_field_node.kind().into()) => {
                let table_field = LuaTableField::cast(table_field_node)?;
                let value_expr_syntax_id = table_field.get_value_expr()?.get_syntax_id();
                let expr = value_expr_syntax_id.to_node_from_root(&root)?;
                let property_owner = semantic_model.get_property_owner_id(expr.clone().into());
                match property_owner {
                    Some(LuaPropertyOwnerId::Member(member_id)) => semantic_model
                        .get_db()
                        .get_member_index()
                        .get_member(&member_id),
                    _ => None,
                }
            }
            _ => None,
        },
        LuaSyntaxKind::IndexExpr => {
            let assign_node = cur_node.parent()?;
            match assign_node {
                assign_node if LuaAssignStat::can_cast(assign_node.kind().into()) => {
                    let assign_stat = LuaAssignStat::cast(assign_node)?;
                    let (vars, exprs) = assign_stat.get_var_and_expr_list();
                    let mut member = None;
                    for (var, expr) in vars.iter().zip(exprs.iter()) {
                        if var.syntax().text_range() == cur_node.text_range() {
                            let expr = expr.get_syntax_id().to_node_from_root(&root)?;
                            let property_owner =
                                semantic_model.get_property_owner_id(expr.clone().into());
                            member = match property_owner {
                                Some(LuaPropertyOwnerId::Member(member_id)) => semantic_model
                                    .get_db()
                                    .get_member_index()
                                    .get_member(&member_id),
                                _ => None,
                            };
                            break;
                        }
                    }
                    member
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn build_hover_result(
    hover_content: HoverContent,
    range: Option<lsp_types::Range>,
) -> Option<Hover> {
    let mut result = String::new();
    match hover_content.type_signature {
        MarkedString::String(s) => {
            result.push_str(&format!("\n{}\n", s));
        }
        MarkedString::LanguageString(s) => {
            result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
        }
    }
    if let Some(location_path) = hover_content.location_path {
        match location_path {
            MarkedString::String(s) => {
                result.push_str(&format!("\n{}\n", s));
            }
            _ => {}
        }
    }

    for marked_string in hover_content.detailed_description {
        match marked_string {
            MarkedString::String(s) => {
                result.push_str(&format!("\n{}\n", s));
            }
            MarkedString::LanguageString(s) => {
                result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
            }
        }
    }
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: result,
        }),
        range,
    })
}
