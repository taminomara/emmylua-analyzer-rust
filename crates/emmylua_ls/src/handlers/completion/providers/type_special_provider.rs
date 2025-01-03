use code_analysis::{
    InferGuard, LuaFunctionType, LuaMemberId, LuaPropertyOwnerId, LuaType, LuaTypeDeclId,
    LuaUnionType,
};
use emmylua_parser::{
    LuaAstNode, LuaCallArgList, LuaCallExpr, LuaExpr, LuaSyntaxKind, LuaSyntaxToken, LuaTokenKind,
};
use lsp_types::CompletionItem;

use crate::handlers::{
    completion::completion_builder::CompletionBuilder, signature_helper::get_current_param_index,
};

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let typ = get_token_should_type(builder)?;
    dispatch_type(builder, typ, &mut InferGuard::new());
    Some(())
}

fn dispatch_type(
    builder: &mut CompletionBuilder,
    typ: LuaType,
    infer_guard: &mut InferGuard,
) -> Option<()> {
    match typ {
        LuaType::Ref(type_ref_id) => {
            add_type_ref_completion(builder, type_ref_id.clone(), infer_guard);
        }
        LuaType::Union(union_typ) => {
            add_union_member_completion(builder, &union_typ, infer_guard);
        }
        LuaType::Nullable(typ) => {
            dispatch_type(builder, (*typ).clone(), infer_guard);
        }
        LuaType::DocFunction(func) => {
            add_lambda_completion(builder, &func);
        }
        _ => {}
    }

    Some(())
}

fn add_type_ref_completion(
    builder: &mut CompletionBuilder,
    type_ref_id: LuaTypeDeclId,
    infer_guard: &mut InferGuard,
) -> Option<()> {
    infer_guard.check(&type_ref_id)?;

    let type_decl = builder
        .semantic_model
        .get_db()
        .get_type_index()
        .get_type_decl(&type_ref_id)?;
    if type_decl.is_alias() {
        if let Some(origin) = type_decl.get_alias_origin() {
            return dispatch_type(builder, origin.clone(), infer_guard);
        }
        let member_ids = type_decl.get_alias_union_members()?.to_vec();

        for member_id in member_ids {
            add_alias_member_completion(builder, &member_id);
        }
        builder.stop_here();
    } else if type_decl.is_enum() {
        // let member_ids = type_decl.get_enum_members()?.to_vec();
        // for member_id in member_ids {
        //     add_alias_member_completion(builder, &member_id);
        // }
        builder.stop_here();
    }

    Some(())
}

fn add_union_member_completion(
    builder: &mut CompletionBuilder,
    union_typ: &LuaUnionType,
    infer_guard: &mut InferGuard,
) -> Option<()> {
    for union_sub_typ in union_typ.get_types() {
        let name = match union_sub_typ {
            LuaType::DocStringConst(s) => to_enum_label(builder, s),
            LuaType::DocIntergerConst(i) => i.to_string(),
            _ => {
                dispatch_type(builder, union_sub_typ.clone(), infer_guard);
                continue;
            }
        };

        let completion_item = CompletionItem {
            label: name,
            kind: Some(lsp_types::CompletionItemKind::ENUM_MEMBER),
            ..Default::default()
        };

        builder.add_completion_item(completion_item);
    }

    Some(())
}

fn get_token_should_type(builder: &mut CompletionBuilder) -> Option<LuaType> {
    let token = builder.trigger_token.clone();
    let mut parent_node = token.parent()?;
    if LuaExpr::can_cast(parent_node.kind().into()) {
        parent_node = parent_node.parent()?;
    }

    match parent_node.kind().into() {
        LuaSyntaxKind::CallArgList => {
            return infer_call_arg_list(builder, LuaCallArgList::cast(parent_node)?, token);
        }
        LuaSyntaxKind::BinaryExpr => {
            // infer_binary_expr(builder, binary_expr)?;
        }
        _ => {}
    }

    None
}

fn infer_call_arg_list(
    builder: &mut CompletionBuilder,
    call_arg_list: LuaCallArgList,
    token: LuaSyntaxToken,
) -> Option<LuaType> {
    let call_expr = call_arg_list.get_parent::<LuaCallExpr>()?;
    let param_idx = get_current_param_index(&call_expr, &token)?;
    let call_expr_func = builder
        .semantic_model
        .infer_call_expr_func(call_expr.clone(), Some(param_idx + 1))?;

    let typ = call_expr_func.get_params().get(param_idx)?.1.clone()?;
    Some(typ)
}

fn add_alias_member_completion(
    builder: &mut CompletionBuilder,
    member_id: &LuaMemberId,
) -> Option<()> {
    let member = builder
        .semantic_model
        .get_db()
        .get_member_index()
        .get_member(&member_id)?;

    let typ = member.get_decl_type();
    let name = match typ {
        LuaType::DocStringConst(s) => to_enum_label(builder, s),
        LuaType::DocIntergerConst(i) => i.to_string(),
        _ => return None,
    };

    let propperty_owner_id = LuaPropertyOwnerId::Member(member_id.clone());
    let description = if let Some(property) = builder
        .semantic_model
        .get_db()
        .get_property_index()
        .get_property(propperty_owner_id)
    {
        if property.description.is_some() {
            Some(*(property.description.clone().unwrap()))
        } else {
            None
        }
    } else {
        None
    };

    let completion_item = CompletionItem {
        label: name,
        kind: Some(lsp_types::CompletionItemKind::ENUM_MEMBER),
        label_details: Some(lsp_types::CompletionItemLabelDetails {
            detail: description,
            description: None,
        }),
        ..Default::default()
    };

    builder.add_completion_item(completion_item);

    Some(())
}

fn to_enum_label(builder: &CompletionBuilder, str: &str) -> String {
    if matches!(
        builder.trigger_token.kind().into(),
        LuaTokenKind::TkString | LuaTokenKind::TkLongString
    ) {
        str.to_string()
    } else {
        format!("\"{}\"", str)
    }
}

fn add_lambda_completion(builder: &mut CompletionBuilder, func: &LuaFunctionType) -> Option<()> {
    let params_str = func
        .get_params()
        .iter()
        .map(|p| p.0.clone())
        .collect::<Vec<_>>();
    let label = format!("function ({}) end", params_str.join(", "));
    let insert_text = format!("function ({})\n\t$0\nend", params_str.join(", "));

    let completion_item = CompletionItem {
        label,
        kind: Some(lsp_types::CompletionItemKind::FUNCTION),
        sort_text: Some("0".to_string()),
        insert_text: Some(insert_text),
        insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
        ..Default::default()
    };

    builder.add_completion_item(completion_item);
    Some(())
}
