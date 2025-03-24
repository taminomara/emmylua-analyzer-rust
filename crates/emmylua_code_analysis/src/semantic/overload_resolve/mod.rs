use std::{cmp::Ordering, sync::Arc};

use emmylua_parser::LuaCallExpr;

use crate::db_index::{DbIndex, LuaFunctionType, LuaType};

use super::{
    generic::instantiate_func_generic, infer::{InferCallFuncResult, InferFailReason}, infer_expr, type_check::check_type_compact, LuaInferCache
};

pub fn resolve_signature(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    overloads: Vec<Arc<LuaFunctionType>>,
    call_expr: LuaCallExpr,
    is_generic: bool,
    arg_count: Option<usize>,
) -> InferCallFuncResult {
    let args = call_expr.get_args_list()?;
    let mut expr_types = Vec::new();
    for arg in args.get_args() {
        expr_types.push(infer_expr(db, cache, arg)?);
    }
    if is_generic {
        return resolve_signature_by_generic(
            db, cache, overloads, call_expr, expr_types, arg_count,
        );
    } else {
        return resolve_signature_by_args(
            db,
            &overloads,
            &expr_types,
            call_expr.is_colon_call(),
            arg_count,
        );
    }
}

fn resolve_signature_by_generic(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    overloads: Vec<Arc<LuaFunctionType>>,
    call_expr: LuaCallExpr,
    expr_types: Vec<LuaType>,
    arg_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let mut instantiate_funcs = Vec::new();
    for func in overloads {
        let instantiate_func = instantiate_func_generic(db, cache, &func, call_expr.clone())?;
        instantiate_funcs.push(Arc::new(instantiate_func));
    }
    resolve_signature_by_args(
        db,
        &instantiate_funcs,
        &expr_types,
        call_expr.is_colon_call(),
        arg_count,
    )
}

fn resolve_signature_by_args(
    db: &DbIndex,
    overloads: &[Arc<LuaFunctionType>],
    expr_types: &[LuaType],
    is_colon_call: bool,
    arg_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let arg_count = arg_count.unwrap_or(0);
    let mut opt_funcs = Vec::with_capacity(overloads.len());
    // 函数本身签名在尾部
    for func in overloads {
        let params = func.get_params();
        if params.len() < arg_count {
            continue;
        }

        let jump_param = if is_colon_call && !func.is_colon_define() {
            1
        } else {
            0
        };
        let mut match_count = 0;
        let expr_len = expr_types.len();

        for (i, param) in params.iter().enumerate() {
            if i == 0 && jump_param > 0 {
                continue;
            }
            let expr_idx = i - jump_param;
            if expr_idx >= expr_len {
                break;
            }

            let param_type = param.1.as_ref().unwrap_or(&LuaType::Any);
            let expr_type = &expr_types[expr_idx];
            if *param_type == LuaType::Any || check_type_compact(db, param_type, expr_type).is_ok()
            {
                match_count += 1;
            }
        }
        opt_funcs.push((func, params.len(), match_count));
    }
    // 优先降序匹配`match_count`使匹配度最高的函数排在前面, 其次升序匹配`params.len()`使参数个数最少的函数排在前面.
    opt_funcs.sort_by(|a, b| match b.2.cmp(&a.2) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });

    opt_funcs
        .first()
        .map(|(func, _, _)| Arc::clone(func))
        .or_else(|| overloads.last().cloned())
}
