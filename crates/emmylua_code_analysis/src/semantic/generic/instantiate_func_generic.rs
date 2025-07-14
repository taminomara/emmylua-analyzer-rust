use std::{collections::HashSet, ops::Deref};

use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr};
use internment::ArcIntern;
use smol_str::SmolStr;

use crate::{
    db_index::{DbIndex, LuaType},
    semantic::{
        generic::{
            tpl_context::TplContext,
            tpl_pattern::{
                multi_param_tpl_pattern_match_multi_return, tpl_pattern_match,
                variadic_tpl_pattern_match,
            },
        },
        infer::InferFailReason,
        infer_expr, LuaInferCache,
    },
    LuaFunctionType, TypeVisitTrait,
};

use super::{instantiate_type_generic::instantiate_doc_function, TypeSubstitutor};

pub fn instantiate_func_generic(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    func: &LuaFunctionType,
    call_expr: LuaCallExpr,
) -> Result<LuaFunctionType, InferFailReason> {
    let mut generic_tpls = HashSet::new();
    let mut contain_self = false;
    func.visit_type(&mut |t| match t {
        LuaType::TplRef(generic_tpl) | LuaType::ConstTplRef(generic_tpl) => {
            let tpl_id = generic_tpl.get_tpl_id();
            if tpl_id.is_func() {
                generic_tpls.insert(tpl_id);
            }
        }
        LuaType::StrTplRef(str_tpl) => {
            generic_tpls.insert(str_tpl.get_tpl_id());
        }
        LuaType::SelfInfer => {
            contain_self = true;
        }
        _ => {}
    });

    let origin_params = func.get_params();
    let mut func_param_types: Vec<_> = origin_params
        .iter()
        .map(|(_, t)| t.clone().unwrap_or(LuaType::Unknown))
        .collect();

    let arg_exprs = call_expr
        .get_args_list()
        .ok_or(InferFailReason::None)?
        .get_args()
        .collect::<Vec<_>>();
    let mut substitutor = TypeSubstitutor::new();
    if !generic_tpls.is_empty() {
        substitutor.add_need_infer_tpls(generic_tpls);
        let mut context = TplContext {
            db,
            cache,
            substitutor: &mut substitutor,
            root: call_expr.get_root(),
        };

        let colon_call = call_expr.is_colon_call();
        let colon_define = func.is_colon_define();
        match (colon_define, colon_call) {
            (true, false) => {
                func_param_types.insert(0, LuaType::Any);
            }
            (false, true) => {
                if !func_param_types.is_empty() {
                    func_param_types.remove(0);
                }
            }
            _ => {}
        }

        let mut unresolve_tpls = vec![];
        for i in 0..func_param_types.len() {
            if i >= arg_exprs.len() {
                break;
            }

            if context.substitutor.is_infer_all_tpl() {
                break;
            }

            let func_param_type = &func_param_types[i];
            let call_arg_expr = &arg_exprs[i];
            if !func_param_type.contain_tpl() {
                continue;
            }

            if !func_param_type.is_variadic() {
                if check_expr_can_later_infer(&mut context, func_param_type, call_arg_expr)? {
                    // If the argument cannot be inferred later, we will handle it later.
                    unresolve_tpls.push((func_param_type.clone(), call_arg_expr.clone()));
                    continue;
                }
            }

            let arg_type = infer_expr(db, context.cache, call_arg_expr.clone())?;

            match (func_param_type, &arg_type) {
                (LuaType::Variadic(variadic), _) => {
                    let mut arg_types = vec![];
                    for arg_expr in &arg_exprs[i..] {
                        let arg_type = infer_expr(db, context.cache, arg_expr.clone())?;
                        arg_types.push(arg_type);
                    }

                    variadic_tpl_pattern_match(&mut context, variadic, &arg_types)?;
                    break;
                }
                (_, LuaType::Variadic(variadic)) => {
                    multi_param_tpl_pattern_match_multi_return(
                        &mut context,
                        &func_param_types[i..],
                        variadic,
                    )?;
                    break;
                }
                _ => {
                    tpl_pattern_match(&mut context, func_param_type, &arg_type)?;
                }
            }
        }

        if !context.substitutor.is_infer_all_tpl() {
            for (func_param_type, call_arg_expr) in unresolve_tpls {
                let closure_type = infer_expr(db, context.cache, call_arg_expr)?;

                tpl_pattern_match(&mut context, &func_param_type, &closure_type)?;
            }
        }
    }

    substitutor.set_call_expr(call_expr.clone());
    if contain_self {
        infer_self_type(db, cache, &call_expr, &mut substitutor)?;
    }

    if let LuaType::DocFunction(f) = instantiate_doc_function(db, func, &substitutor) {
        Ok(f.deref().clone())
    } else {
        Ok(func.clone())
    }
}

fn infer_self_type(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    call_expr: &LuaCallExpr,
) -> Option<LuaType> {
    let prefix_expr = call_expr.get_prefix_expr();
    if let Some(prefix_expr) = prefix_expr {
        if let LuaExpr::IndexExpr(index) = prefix_expr {
            let self_expr = index.get_prefix_expr();
            if let Some(self_expr) = self_expr {
                let self_type = infer_expr(db, cache, self_expr.into()).ok()?;
                let self_type = build_self_type(db, &self_type);
                return Some(self_type);
            }
        }
    }

    None
}

fn check_expr_can_later_infer(
    context: &mut TplContext,
    func_param_type: &LuaType,
    call_arg_expr: &LuaExpr,
) -> Result<bool, InferFailReason> {
    let doc_function = match func_param_type {
        LuaType::DocFunction(doc_func) => doc_func.clone(),
        LuaType::Signature(sig_id) => {
            let sig = context
                .db
                .get_signature_index()
                .get(&sig_id)
                .ok_or(InferFailReason::None)?;

            sig.to_doc_func_type()
        }
        _ => return Ok(false),
    };

    if let LuaExpr::ClosureExpr(_) = call_arg_expr {
        return Ok(true);
    }

    let doc_params = doc_function.get_params();
    let variadic_count = doc_params
        .iter()
        .filter_map(|(_, t)| {
            if let Some(LuaType::Variadic(_)) = t {
                Some(())
            } else {
                None
            }
        })
        .count();

    Ok(variadic_count > 1)
}
