use std::sync::Arc;

use emmylua_parser::LuaCallExpr;

use crate::db_index::{DbIndex, LuaFunctionType, LuaType};

use super::{
    infer_expr, instantiate::instantiate_func, type_check::check_type_compact, LuaInferConfig,
};

pub fn resolve_signature(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    overloads: Vec<Arc<LuaFunctionType>>,
    call_expr: LuaCallExpr,
    colon_define: bool,
    is_generic: bool,
    arg_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let args = call_expr.get_args_list()?;
    let mut expr_types = Vec::new();
    for arg in args.get_args() {
        expr_types.push(infer_expr(db, infer_config, arg)?);
    }

    let colon_call = call_expr.is_colon_call();
    let mut expr_types = expr_types;
    match (colon_call, colon_define) {
        (true, true) | (false, false) => {}
        (false, true) => {
            if expr_types.len() > 0 {
                expr_types.remove(0);
            }
        }
        (true, false) => {
            expr_types.insert(0, LuaType::Any);
        }
    }

    if is_generic {
        return resolve_signature_by_generic(
            db,
            infer_config,
            overloads,
            colon_define,
            call_expr,
            expr_types,
            arg_count,
        );
    } else {
        return resolve_signature_by_args(db, overloads, expr_types, arg_count);
    }
}

fn resolve_signature_by_generic(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    overloads: Vec<Arc<LuaFunctionType>>,
    colon_define: bool,
    call_expr: LuaCallExpr,
    expr_types: Vec<LuaType>,
    arg_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let mut max_match = -1;
    let mut matched_func: Option<Arc<LuaFunctionType>> = None;
    let mut instantiate_funcs = Vec::new();
    for func in overloads {
        let params = func.get_params();
        let mut func_param_types: Vec<_> = params
            .iter()
            .map(|(_, t)| t.clone().unwrap_or(LuaType::Any))
            .collect();
        let mut func_return_types = func.get_ret().to_vec();

        instantiate_func(
            db,
            infer_config,
            colon_define,
            call_expr.clone(),
            &mut func_param_types,
            &mut func_return_types,
        );
        let mut new_params = Vec::new();
        for i in 0..params.len() {
            let new_param = func_param_types[i].clone();
            new_params.push((params[i].0.clone(), Some(new_param)));
        }
        let new_func = Arc::new(LuaFunctionType::new(
            func.is_async(),
            func.is_colon_define(),
            new_params,
            func_return_types,
        ));
        instantiate_funcs.push(new_func);
    }

    for func in &instantiate_funcs {
        let params = func.get_params();
        let mut match_count = 0;
        if params.len() < arg_count.unwrap_or(0) {
            continue;
        }

        for (i, param) in params.iter().enumerate() {
            if i >= expr_types.len() {
                break;
            }

            let param_type = param.1.clone().unwrap_or(LuaType::Any);
            let expr_type = &expr_types[i];
            if param_type == LuaType::Any {
                match_count += 1;
            } else if check_type_compact(db, &param_type, expr_type) {
                match_count += 1;
            }
        }
        if match_count > max_match {
            max_match = match_count;
            matched_func = Some(func.clone());
        }
    }

    if matched_func.is_none() && !instantiate_funcs.is_empty() {
        matched_func = Some(instantiate_funcs[0].clone());
    }

    matched_func
}

fn resolve_signature_by_args(
    db: &DbIndex,
    overloads: Vec<Arc<LuaFunctionType>>,
    expr_types: Vec<LuaType>,
    arg_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let mut max_match = -1;
    let mut matched_func: Option<Arc<LuaFunctionType>> = None;

    for func in &overloads {
        let params = func.get_params();
        let mut match_count = 0;
        if params.len() < arg_count.unwrap_or(0) {
            continue;
        }

        for (i, param) in params.iter().enumerate() {
            if i >= expr_types.len() {
                break;
            }

            let param_type = param.1.clone().unwrap_or(LuaType::Any);
            let expr_type = &expr_types[i];
            if param_type == LuaType::Any {
                match_count += 1;
            } else if check_type_compact(db, &param_type, expr_type) {
                match_count += 1;
            }
        }
        if match_count > max_match {
            max_match = match_count;
            matched_func = Some(func.clone());
        }
    }

    if matched_func.is_none() && !overloads.is_empty() {
        matched_func = Some(overloads[0].clone());
    }

    matched_func
}
