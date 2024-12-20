use std::collections::HashMap;

use code_analysis::{LuaSignatureId, SemanticModel};
use emmylua_parser::{LuaAst, LuaAstNode, LuaCallExpr, LuaClosureExpr};
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel};

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
                build_call_expr_hint(semantic_model, &mut result, call_expr);
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
                let typ_desc = format!(":{}", humanize_type(db, &typ));
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

fn build_call_expr_hint(
    semantic_model: &mut SemanticModel,
    result: &mut Vec<InlayHint>,
    call_expr: LuaCallExpr,
) -> Option<()> {
    // let file_id = semantic_model.get_file_id();
    // let signature_id = LuaSignatureId::new(file_id, &call_expr);
    // let prefix_expr = call_expr
    // todo
    Some(())
}