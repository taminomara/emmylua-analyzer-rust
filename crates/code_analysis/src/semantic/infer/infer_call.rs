use std::{ops::Deref, sync::Arc};

use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr, LuaSyntaxKind};

use crate::{
    db_index::{
        DbIndex, LuaFunctionType, LuaGenericType, LuaInstanceType, LuaMultiReturn,
        LuaOperatorMetaMethod, LuaSignatureId, LuaType, LuaTypeDeclId,
    },
    semantic::{
        instantiate::{instantiate_func, instantiate_type},
        overload_resolve::resolve_signature,
        InferGuard,
    },
    InFiled,
};

use super::{infer_expr, LuaInferConfig};

pub fn infer_call_expr(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    if let LuaExpr::NameExpr(name_expr) = &prefix_expr {
        let name = name_expr.get_name_text();
        if let Some(func_name) = name {
            if config.is_require_function(&func_name) {
                return infer_require_call(db, config, call_expr);
            }
        }
    }

    let prefix_type = infer_expr(db, config, prefix_expr)?;

    infer_call_result(db, config, prefix_type, call_expr, &mut InferGuard::new())
}

fn infer_call_result(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    prefix_type: LuaType,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
) -> Option<LuaType> {
    let return_type = match prefix_type {
        LuaType::DocFunction(func) => {
            infer_call_by_doc_function(db, config, &func, call_expr.clone(), false)?
        }
        LuaType::Signature(signature_id) => {
            infer_call_by_signature(db, config, signature_id.clone(), call_expr.clone())?
        }
        LuaType::Def(type_def_id) => infer_call_by_custom_type(
            db,
            config,
            type_def_id.clone(),
            call_expr.clone(),
            infer_guard,
        )?,
        LuaType::Ref(type_ref_id) => infer_call_by_custom_type(
            db,
            config,
            type_ref_id.clone(),
            call_expr.clone(),
            infer_guard,
        )?,
        LuaType::Generic(generic) => {
            infer_call_by_custom_generic_type(db, config, &generic, call_expr.clone(), infer_guard)?
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
    colon_define: bool,
) -> Option<LuaType> {
    let rets = func.get_ret();
    let is_generic_rets = rets.iter().any(|ret| ret.contain_tpl());
    let ret = if is_generic_rets {
        let instantiate_func = instantiate_doc_function(db, config, func, call_expr, colon_define)?;
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
    if !signature.is_resolve_return() {
        return None;
    }

    let overloads = &signature.overloads;
    if overloads.is_empty() {
        let rets = &signature.return_docs;
        let ret = if rets.len() == 1 {
            rets[0].type_ref.clone()
        } else if rets.is_empty() {
            return Some(LuaType::Nil);
        } else {
            LuaType::MuliReturn(
                LuaMultiReturn::Multi(rets.iter().map(|r| r.type_ref.clone()).collect()).into(),
            )
        };

        if signature.is_generic() && ret.contain_tpl() {
            let fake_doc_function = LuaFunctionType::new(
                false,
                signature.is_colon_define,
                signature.get_type_params(),
                vec![ret.clone()],
            );
            let instantiate_func = instantiate_doc_function(
                db,
                config,
                &fake_doc_function,
                call_expr,
                signature.is_colon_define,
            )?;
            let rets = instantiate_func.get_ret();
            return rets.get(0).cloned();
        }

        return Some(ret);
    } else {
        let mut new_overloads = signature.overloads.clone();
        let rets = &signature.return_docs;
        let ret = if rets.len() == 1 {
            rets[0].type_ref.clone()
        } else if rets.is_empty() {
            return Some(LuaType::Nil);
        } else {
            LuaType::MuliReturn(
                LuaMultiReturn::Multi(rets.iter().map(|r| r.type_ref.clone()).collect()).into(),
            )
        };
        let fake_doc_function = Arc::new(LuaFunctionType::new(
            false,
            signature.is_colon_define,
            signature.get_type_params(),
            vec![ret.clone()],
        ));
        new_overloads.push(fake_doc_function);

        let doc_func = resolve_signature(
            db,
            config,
            new_overloads,
            call_expr.clone(),
            signature.is_colon_define,
            signature.is_generic(),
            None,
        )?;
        return infer_call_by_doc_function(
            db,
            config,
            &doc_func,
            call_expr,
            signature.is_colon_define,
        );
    }
}

pub fn instantiate_doc_function(
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
        colon_define,
        new_params,
        func_return_types,
    ))
}

fn infer_call_by_custom_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    type_id: LuaTypeDeclId,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
) -> Option<LuaType> {
    infer_guard.check(&type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        let alias_type = type_decl.get_alias_origin()?;
        return infer_call_result(db, config, alias_type.clone(), call_expr, infer_guard);
    } else if type_decl.is_enum() {
        return Some(LuaType::Unknown);
    }

    let operator_index = db.get_operator_index();
    let operator_map = operator_index.get_operators_by_type(&type_id)?;
    let operator_ids = operator_map.get(&LuaOperatorMetaMethod::Call)?;
    let mut overloads = Vec::new();
    for overload_id in operator_ids {
        let operator = operator_index.get_operator(overload_id)?;
        let func = operator.get_call_operator_type()?;
        match func {
            LuaType::DocFunction(f) => {
                overloads.push(f.clone());
            }
            _ => {}
        }
    }

    let doc_func = resolve_signature(db, config, overloads, call_expr.clone(), false, false, None)?;
    return infer_call_by_doc_function(db, config, &doc_func, call_expr, false);
}

fn infer_call_by_custom_generic_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    generic: &LuaGenericType,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
) -> Option<LuaType> {
    let type_id = generic.get_base_type_id();
    infer_guard.check(&type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        // todo: add generic deal
        let alias_type = type_decl.get_alias_origin()?;
        return infer_call_result(db, config, alias_type.clone(), call_expr, infer_guard);
    } else if type_decl.is_enum() {
        return Some(LuaType::Unknown);
    }

    let generic_params = generic.get_params();
    let operator_index = db.get_operator_index();
    let operator_map = operator_index.get_operators_by_type(&type_id)?;
    let operator_ids = operator_map.get(&LuaOperatorMetaMethod::Call)?;
    let mut overloads = Vec::new();
    for overload_id in operator_ids {
        let operator = operator_index.get_operator(overload_id)?;
        let func = operator.get_call_operator_type()?;
        let new_f = instantiate_type(db, func, generic_params);
        match new_f {
            LuaType::DocFunction(f) => {
                overloads.push(f.clone());
            }
            _ => {}
        }
    }

    let doc_func = resolve_signature(db, config, overloads, call_expr.clone(), false, false, None)?;
    return infer_call_by_doc_function(db, config, &doc_func, call_expr, false);
}

fn unwrapp_return_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    return_type: LuaType,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    match &return_type {
        LuaType::Table | LuaType::TableConst(_) | LuaType::Any | LuaType::Unknown => {
            let id = InFiled {
                file_id: config.get_file_id(),
                value: call_expr.get_range(),
            };

            return Some(LuaType::Instance(
                LuaInstanceType::new(return_type, id).into(),
            ));
        }
        LuaType::MuliReturn(multi) => {
            let parent = call_expr.syntax().parent();
            if let Some(parent) = parent {
                match parent.kind().into() {
                    LuaSyntaxKind::AssignStat
                    | LuaSyntaxKind::LocalStat
                    | LuaSyntaxKind::ReturnStat
                    | LuaSyntaxKind::TableArrayExpr
                    | LuaSyntaxKind::CallArgList => {
                        let next_expr = call_expr.syntax().next_sibling();
                        if next_expr.is_none() {
                            return Some(return_type);
                        }
                    }
                    _ => {}
                }
            }

            return multi.get_type(0).cloned();
        }
        LuaType::Variadic(inner) => {
            return Some(inner.deref().clone());
        }
        LuaType::SelfInfer => {
            let prefix_expr = call_expr.get_prefix_expr();
            if let Some(prefix_expr) = prefix_expr {
                if let LuaExpr::IndexExpr(index) = prefix_expr {
                    let self_expr = index.get_prefix_expr();
                    if let Some(self_expr) = self_expr {
                        let self_type = infer_expr(db, config, self_expr.into());
                        return self_type;
                    }
                }
            }
        }
        _ => {}
    }

    Some(return_type)
}

fn infer_require_call(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let arg_list = call_expr.get_args_list()?;
    let first_arg = arg_list.get_args().next()?;
    let require_path_type = infer_expr(db, config, first_arg)?;
    let module_path: String = match &require_path_type {
        LuaType::StringConst(module_path) => module_path.as_ref().to_string(),
        _ => {
            return None;
        }
    };

    let module_info = db.get_module_index().find_module(&module_path)?;
    module_info.export_type.clone()
}
