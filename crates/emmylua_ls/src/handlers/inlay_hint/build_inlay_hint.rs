use std::collections::HashMap;

use code_analysis::{LuaFunctionType, LuaPropertyOwnerId, LuaSignatureId, LuaType, SemanticModel};
use emmylua_parser::{LuaAst, LuaAstNode, LuaCallExpr, LuaClosureExpr, LuaExpr, LuaLocalName};
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel};
use rowan::NodeOrToken;

use crate::util::humanize_type;

pub fn build_inlay_hints(semantic_model: &mut SemanticModel) -> Option<Vec<InlayHint>> {
    let mut result = Vec::new();
    let root = semantic_model.get_root();
    for node in root.clone().descendants::<LuaAst>() {
        match node {
            LuaAst::LuaClosureExpr(closure) => {
                build_closure_hint(semantic_model, &mut result, closure);
            }
            LuaAst::LuaCallExpr(call_expr) => {
                build_call_expr_param_hint(semantic_model, &mut result, call_expr.clone());
                build_call_expr_await_hint(semantic_model, &mut result, call_expr);
            }
            LuaAst::LuaLocalName(local_name) => {
                build_local_name_hint(semantic_model, &mut result, local_name);
            }
            _ => {}
        }
    }

    Some(result)
}

fn build_closure_hint(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    closure: LuaClosureExpr,
) -> Option<()> {
    if !semantic_model.get_emmyrc().hint.param_hint {
        return Some(());
    }
    let file_id = semantic_model.get_file_id();
    let signature_id = LuaSignatureId::new(file_id, &closure);
    let signature = semantic_model
        .get_db()
        .get_signature_index()
        .get(&signature_id)?;

    let lua_params = closure.get_params_list()?;
    let signature_params = signature.get_type_params();
    let mut lua_params_map = HashMap::new();
    for param in lua_params.get_params() {
        if let Some(name_token) = param.get_name_token() {
            let name = name_token.get_name_text().to_string();
            lua_params_map.insert(name, param);
        } else if param.is_dots() {
            lua_params_map.insert("...".to_string(), param);
        }
    }

    let document = semantic_model.get_document();
    let db = semantic_model.get_db();
    for (signature_param_name, typ) in &signature_params {
        if let Some(typ) = typ {
            if let Some(lua_param) = lua_params_map.get(signature_param_name) {
                let lsp_range = document.to_lsp_range(lua_param.get_range())?;
                let typ_desc = format!(": {}", humanize_type(db, &typ));
                let hint = InlayHint {
                    kind: Some(InlayHintKind::PARAMETER),
                    label: InlayHintLabel::String(typ_desc),
                    position: lsp_range.end,
                    text_edits: None,
                    tooltip: None,
                    padding_left: Some(true),
                    padding_right: None,
                    data: None,
                };
                result.push(hint);
            }
        }
    }

    Some(())
}

fn build_call_expr_param_hint(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    call_expr: LuaCallExpr,
) -> Option<()> {
    if !semantic_model.get_emmyrc().hint.param_hint {
        return Some(());
    }

    let prefix_expr = call_expr.get_prefix_expr()?;
    let semantic_info =
        semantic_model.get_semantic_info(NodeOrToken::Node(prefix_expr.syntax().clone()))?;

    let call_args_list = call_expr.get_args_list()?;
    let colon_call = call_expr.is_colon_call();
    match semantic_info.typ {
        LuaType::DocFunction(f) => {
            build_call_args_for_func_type(
                semantic_model,
                result,
                call_args_list.get_args().collect(),
                colon_call,
                &f,
            );
        }
        LuaType::Signature(signature_id) => {
            build_call_args_for_signature(
                semantic_model,
                result,
                call_args_list.get_args().collect(),
                colon_call,
                signature_id,
            );
        }
        _ => {}
    }
    Some(())
}

fn build_call_expr_await_hint(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    let semantic_info =
        semantic_model.get_semantic_info(NodeOrToken::Node(prefix_expr.syntax().clone()))?;

    match semantic_info.typ {
        LuaType::DocFunction(f) => {
            if f.is_async() {
                let range = call_expr.get_range();
                let document = semantic_model.get_document();
                let lsp_range = document.to_lsp_range(range)?;
                let hint = InlayHint {
                    kind: Some(InlayHintKind::TYPE),
                    label: InlayHintLabel::String("await".to_string()),
                    position: lsp_range.start,
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: Some(true),
                    data: None,
                };
                result.push(hint);
            }
        }
        LuaType::Signature(signature_id) => {
            let property_owner_id = LuaPropertyOwnerId::Signature(signature_id);
            let property = semantic_model
                .get_db()
                .get_property_index()
                .get_property(property_owner_id)?;
            if property.is_async {
                let range = call_expr.get_range();
                let document = semantic_model.get_document();
                let lsp_range = document.to_lsp_range(range)?;
                let hint = InlayHint {
                    kind: Some(InlayHintKind::TYPE),
                    label: InlayHintLabel::String("await".to_string()),
                    position: lsp_range.start,
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: Some(true),
                    data: None,
                };
                result.push(hint);
            }
        }
        _ => {}
    }
    Some(())
}

fn build_call_args_for_func_type(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    call_args: Vec<LuaExpr>,
    colon_call: bool,
    func_type: &LuaFunctionType,
) -> Option<()> {
    let call_args_len = call_args.len();
    let mut params = func_type
        .get_params()
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();

    let colon_define = func_type.is_colon_define();
    match (colon_call, colon_define) {
        (false, true) => {
            params.insert(0, "self".to_string());
        }
        (true, false) => {
            if params.len() > 0 {
                params.remove(0);
            }
        }
        _ => {}
    }

    for (idx, name) in params.iter().enumerate() {
        if idx >= call_args_len {
            break;
        }

        if name == "..." {
            for i in idx..call_args_len {
                let arg = &call_args[i];
                let range = arg.get_range();
                let document = semantic_model.get_document();
                let lsp_range = document.to_lsp_range(range)?;
                let hint = InlayHint {
                    kind: Some(InlayHintKind::PARAMETER),
                    label: InlayHintLabel::String(format!("var{}:", i - idx)),
                    position: lsp_range.start,
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: Some(true),
                    data: None,
                };
                result.push(hint);
            }
            break;
        }

        let arg = &call_args[idx];
        let range = arg.get_range();
        let document = semantic_model.get_document();
        let lsp_range = document.to_lsp_range(range)?;
        let hint = InlayHint {
            kind: Some(InlayHintKind::PARAMETER),
            label: InlayHintLabel::String(format!("{}:", name)),
            position: lsp_range.start,
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: Some(true),
            data: None,
        };
        result.push(hint);
    }

    Some(())
}

fn build_call_args_for_signature(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    call_args: Vec<LuaExpr>,
    colon_call: bool,
    signature_id: LuaSignatureId,
) -> Option<()> {
    let signature = semantic_model
        .get_db()
        .get_signature_index()
        .get(&signature_id)?;
    let call_args_len = call_args.len();
    let mut params = signature
        .get_type_params()
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();

    let colon_define = signature.is_colon_define;
    match (colon_call, colon_define) {
        (false, true) => {
            params.insert(0, "self".to_string());
        }
        (true, false) => {
            if params.len() > 0 {
                params.remove(0);
            }
        }
        _ => {}
    }

    for (idx, name) in params.iter().enumerate() {
        if idx >= call_args_len {
            break;
        }

        if name == "..." {
            for i in idx..call_args_len {
                let arg = &call_args[i];
                let range = arg.get_range();
                let document = semantic_model.get_document();
                let lsp_range = document.to_lsp_range(range)?;
                let hint = InlayHint {
                    kind: Some(InlayHintKind::PARAMETER),
                    label: InlayHintLabel::String(format!("var{}:", i - idx)),
                    position: lsp_range.start,
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: Some(true),
                    data: None,
                };
                result.push(hint);
            }
            break;
        }

        let arg = &call_args[idx];
        let range = arg.get_range();
        let document = semantic_model.get_document();
        let lsp_range = document.to_lsp_range(range)?;
        let hint = InlayHint {
            kind: Some(InlayHintKind::PARAMETER),
            label: InlayHintLabel::String(format!("{}:", name)),
            position: lsp_range.start,
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: Some(true),
            data: None,
        };
        result.push(hint);
    }

    Some(())
}

fn build_local_name_hint(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    local_name: LuaLocalName,
) -> Option<()> {
    if !semantic_model.get_emmyrc().hint.local_hint {
        return Some(());
    }

    let typ = semantic_model
        .get_semantic_info(NodeOrToken::Node(local_name.syntax().clone()))?
        .typ
        .clone();

    let document = semantic_model.get_document();
    let db = semantic_model.get_db();
    let range = local_name.get_range();
    let lsp_range = document.to_lsp_range(range)?;

    let typ_desc = humanize_type(db, &typ);
    let hint = InlayHint {
        kind: Some(InlayHintKind::TYPE),
        label: InlayHintLabel::String(format!(": {}", typ_desc)),
        position: lsp_range.end,
        text_edits: None,
        tooltip: None,
        padding_left: Some(true),
        padding_right: None,
        data: None,
    };
    result.push(hint);

    Some(())
}
