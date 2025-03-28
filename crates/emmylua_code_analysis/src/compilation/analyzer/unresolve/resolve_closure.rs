use emmylua_parser::{LuaAstNode, LuaExpr, LuaVarExpr};

use crate::{
    infer_call_expr_func, infer_expr, infer_table_field_value_should_be, DbIndex, InferFailReason,
    InferGuard, LuaDocParamInfo, LuaDocReturnInfo, LuaInferCache, LuaType, SignatureReturnStatus,
};

use super::{
    resolve::try_resolve_return_point, UnResolveCallClosureParams, UnResolveClosureReturn,
    UnResolveParentAst, UnResolveParentClosureParams, UnResolveReturn,
};

pub fn try_resolve_closure_params(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    closure_params: &UnResolveCallClosureParams,
) -> Option<bool> {
    let call_expr = closure_params.call_expr.clone();
    let prefix_expr = call_expr.get_prefix_expr()?;
    let call_expr_type = infer_expr(db, cache, prefix_expr.into()).ok()?;

    let call_doc_func = infer_call_expr_func(
        db,
        cache,
        call_expr.clone(),
        call_expr_type,
        &mut InferGuard::new(),
        None,
    )
    .ok()?;

    let colon_call = call_expr.is_colon_call();
    let colon_define = call_doc_func.is_colon_define();

    let mut param_idx = closure_params.param_idx;
    match (colon_call, colon_define) {
        (true, false) => {
            param_idx += 1;
        }
        (false, true) => {
            if param_idx == 0 {
                return Some(true);
            }

            param_idx -= 1;
        }
        _ => {}
    }

    let mut is_async = false;
    let expr_closure_params = if let Some(param_type) = call_doc_func.get_params().get(param_idx) {
        if let Some(LuaType::DocFunction(func)) = &param_type.1 {
            if func.is_async() {
                is_async = true;
            }

            func.get_params()
        } else {
            return Some(true);
        }
    } else {
        return Some(true);
    };

    let signature = db
        .get_signature_index_mut()
        .get_mut(&closure_params.signature_id)?;

    let signature_params = &mut signature.param_docs;
    for (idx, (name, type_ref)) in expr_closure_params.iter().enumerate() {
        if signature_params.contains_key(&idx) {
            continue;
        }

        signature_params.insert(
            idx,
            LuaDocParamInfo {
                name: name.clone(),
                type_ref: type_ref.clone().unwrap_or(LuaType::Any),
                description: None,
                nullable: false,
            },
        );
    }

    signature.is_async = is_async;

    Some(true)
}

pub fn try_resolve_closure_return(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    closure_return: &mut UnResolveClosureReturn,
) -> Option<bool> {
    let call_expr = closure_return.call_expr.clone();
    let prefix_expr = call_expr.get_prefix_expr()?;
    let call_expr_type = infer_expr(db, cache, prefix_expr.into()).ok()?;
    let mut param_idx = closure_return.param_idx;
    let call_doc_func = infer_call_expr_func(
        db,
        cache,
        call_expr.clone(),
        call_expr_type,
        &mut InferGuard::new(),
        None,
    )
    .ok()?;

    let colon_define = call_doc_func.is_colon_define();
    let colon_call = call_expr.is_colon_call();
    match (colon_define, colon_call) {
        (true, false) => {
            if param_idx == 0 {
                return Some(true);
            }
            param_idx -= 1
        }
        (false, true) => {
            param_idx += 1;
        }
        _ => {}
    }

    let expr_closure_return = if let Some(param_type) = call_doc_func.get_params().get(param_idx) {
        if let Some(LuaType::DocFunction(func)) = &param_type.1 {
            func.get_ret()
        } else {
            return Some(true);
        }
    } else {
        return Some(true);
    };

    let signature = db
        .get_signature_index_mut()
        .get_mut(&closure_return.signature_id)?;

    if expr_closure_return.iter().any(|it| it.contain_tpl()) {
        return try_convert_to_func_body_infer(db, cache, closure_return);
    }

    for ret_type in expr_closure_return {
        signature.return_docs.push(LuaDocReturnInfo {
            name: None,
            type_ref: ret_type.clone(),
            description: None,
        });
    }

    signature.resolve_return = SignatureReturnStatus::DocResolve;
    Some(true)
}

fn try_convert_to_func_body_infer(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    closure_return: &mut UnResolveClosureReturn,
) -> Option<bool> {
    let mut unresolve = UnResolveReturn {
        file_id: closure_return.file_id,
        signature_id: closure_return.signature_id,
        return_points: closure_return.return_points.clone(),
        reason: InferFailReason::None,
    };

    try_resolve_return_point(db, cache, &mut unresolve)
}

pub fn try_resolve_closure_parent_params(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    closure_params: &UnResolveParentClosureParams,
) -> Option<bool> {
    let signature = db.get_signature_index().get(&closure_params.signature_id)?;

    if !signature.param_docs.is_empty() {
        return Some(true);
    }

    let member_type = match &closure_params.parent_ast {
        UnResolveParentAst::LuaFuncStat(func_stat) => {
            let func_name = func_stat.get_func_name()?;
            match func_name {
                LuaVarExpr::IndexExpr(index_expr) => {
                    infer_expr(db, cache, LuaExpr::IndexExpr(index_expr)).ok()?
                }
                LuaVarExpr::NameExpr(_) => return Some(true),
            }
        }
        UnResolveParentAst::LuaTableField(table_field) => {
            infer_table_field_value_should_be(db, cache, table_field.clone()).ok()?
        }
        UnResolveParentAst::LuaAssignStat(assign) => {
            let (vars, exprs) = assign.get_var_and_expr_list();
            let position = closure_params.signature_id.get_position();
            let idx = exprs
                .iter()
                .position(|expr| expr.get_position() == position)?;
            let var = vars.get(idx)?;

            match var {
                LuaVarExpr::IndexExpr(index_expr) => {
                    infer_expr(db, cache, LuaExpr::IndexExpr(index_expr.clone())).ok()?
                }
                LuaVarExpr::NameExpr(_) => return Some(true),
            }
        }
    };

    let LuaType::DocFunction(doc_func) = member_type else {
        return Some(true);
    };

    let signature = db
        .get_signature_index_mut()
        .get_mut(&closure_params.signature_id)?;

    if doc_func.is_async() {
        signature.is_async = true;
    }

    let colon_define = signature.is_colon_define;
    let mut params = doc_func.get_params();
    if colon_define {
        if params.len() > 1 {
            params = &params[1..];
        } else {
            params = &[];
        }
    }

    for (index, param) in params.iter().enumerate() {
        let name = signature.params.get(index).unwrap_or(&param.0);
        signature.param_docs.insert(
            index,
            LuaDocParamInfo {
                name: name.clone(),
                type_ref: param.1.clone().unwrap_or(LuaType::Any),
                description: None,
                nullable: false,
            },
        );
    }

    if signature.resolve_return == SignatureReturnStatus::UnResolve
        || signature.resolve_return == SignatureReturnStatus::InferResolve
    {
        signature.return_docs.clear();
        signature.resolve_return = SignatureReturnStatus::DocResolve;
        for ret in doc_func.get_ret() {
            signature.return_docs.push(LuaDocReturnInfo {
                name: None,
                type_ref: ret.clone(),
                description: None,
            });
        }
    }

    Some(true)
}
