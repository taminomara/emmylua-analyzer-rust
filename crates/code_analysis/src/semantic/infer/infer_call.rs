use emmylua_parser::LuaCallExpr;

use crate::{
    db_index::{DbIndex, LuaFunctionType, LuaMultiReturn, LuaSignatureId, LuaType},
    semantic::instantiate::instantiate_func,
};

use super::{infer_expr, LuaInferConfig};

pub fn infer_call_expr(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    let prefix_type = infer_expr(db, config, prefix_expr)?;

    infer_call_result(db, config, prefix_type, call_expr)
}

fn infer_call_result(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    prefix_type: LuaType,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let return_type = match prefix_type {
        LuaType::DocFunction(func) => {
            infer_call_by_doc_function(db, config, &func, call_expr.clone())?
        }
        LuaType::Signature(signature_id) => {
            infer_call_by_signature(db, config, signature_id.clone(), call_expr.clone())?
        }
        LuaType::Def(type_def_id) => {
            todo!()
        }
        LuaType::Ref(type_ref_id) => {
            todo!()
        }
        LuaType::Generic(generic) => {
            todo!()
        }
        _ => return None,
    };

    unwrapp_return_type(db, config, return_type, call_expr)
}

fn infer_call_by_doc_function(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    func: &LuaFunctionType,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let rets = func.get_ret();
    let is_generic_rets = rets.iter().any(|ret| ret.contain_tpl());
    let ret = if is_generic_rets {
        let instantiate_func = instantiate_doc_function(db, config, func, call_expr, false)?;
        let rets = instantiate_func.get_ret();
        match rets.len() {
            0 => LuaType::Nil,
            1 => rets[0].clone(),
            _ => LuaType::MuliReturn(LuaMultiReturn::Multi(rets.to_vec()).into()),
        }
    } else {
        match rets.len() {
            0 => LuaType::Nil,
            1 => rets[0].clone(),
            _ => LuaType::MuliReturn(LuaMultiReturn::Multi(rets.to_vec()).into()),
        }
    };

    Some(ret)
}

fn infer_call_by_signature(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    signature_id: LuaSignatureId,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let signature = db.get_signature_index().get(&signature_id)?;
    // let rets = signature.get_ret();
    todo!()
}

fn unwrapp_return_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    return_type: LuaType,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    Some(return_type)
}

fn instantiate_doc_function(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    func: &LuaFunctionType,
    call_expr: LuaCallExpr,
    colon_define: bool,
) -> Option<LuaFunctionType> {
    let origin_params = func.get_params();
    let mut func_param_types: Vec<_> = origin_params
        .iter()
        .map(|(_, t)| t.clone().unwrap_or(LuaType::Unknown))
        .collect();

    let mut func_return_types: Vec<_> = func.get_ret().to_vec();
    let is_async = func.is_async();

    instantiate_func(
        db,
        config,
        colon_define,
        call_expr,
        &mut func_param_types,
        &mut func_return_types,
    )?;

    let mut new_params = Vec::new();
    for i in 0..origin_params.len() {
        let new_param = func_param_types[i].clone();
        new_params.push((origin_params[i].0.clone(), Some(new_param)));
    }

    Some(LuaFunctionType::new(
        is_async,
        new_params,
        func_return_types,
    ))
}
