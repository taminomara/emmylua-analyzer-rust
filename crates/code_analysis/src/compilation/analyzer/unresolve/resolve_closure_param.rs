use std::sync::Arc;

use emmylua_parser::LuaCallExpr;

use crate::{
    infer_expr, instantiate_doc_function, instantiate_type, resolve_signature, DbIndex, InferGuard,
    LuaDocParamInfo, LuaFunctionType, LuaGenericType, LuaInferConfig, LuaOperatorMetaMethod,
    LuaSignatureId, LuaType, LuaTypeDeclId,
};

use super::UnResolveClosureParams;

pub fn try_resolve_closure_params(
    db: &mut DbIndex,
    config: &mut LuaInferConfig,
    closure_params: &UnResolveClosureParams,
) -> Option<bool> {
    let call_expr = closure_params.call_expr.clone();
    let prefix_expr = call_expr.get_prefix_expr()?;
    let call_expr_type = infer_expr(db, config, prefix_expr.into())?;

    let call_doc_func = infer_doc_func(
        db,
        config,
        call_expr,
        call_expr_type,
        &mut InferGuard::new(),
    )?;

    let signature = db
        .get_signature_index_mut()
        .get_mut(&closure_params.signature_id)?;

    let expr_closure_params =
        if let Some(param_type) = call_doc_func.get_params().get(closure_params.param_idx) {
            if let Some(LuaType::DocFunction(func)) = &param_type.1 {
                func.get_params()
            } else {
                return Some(true);
            }
        } else {
            return Some(true);
        };

    let signature_params = &mut signature.param_docs;
    for (name, type_ref) in expr_closure_params {
        if signature_params.contains_key(name) {
            continue;
        }

        signature_params.insert(
            name.clone(),
            LuaDocParamInfo {
                name: name.clone(),
                type_ref: type_ref.clone().unwrap_or(LuaType::Any),
                description: None,
                nullable: false,
            },
        );
    }

    Some(true)
}

fn infer_doc_func(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    call_expr: LuaCallExpr,
    call_expr_type: LuaType,
    infer_guard: &mut InferGuard,
) -> Option<Arc<LuaFunctionType>> {
    match call_expr_type {
        LuaType::DocFunction(func) => Some(func),
        LuaType::Signature(signature_id) => {
            infer_signature_doc_function(db, config, signature_id.clone(), call_expr.clone())
        }
        LuaType::Def(type_def_id) => infer_type_doc_function(
            db,
            config,
            type_def_id.clone(),
            call_expr.clone(),
            infer_guard,
        ),
        LuaType::Ref(type_ref_id) => infer_type_doc_function(
            db,
            config,
            type_ref_id.clone(),
            call_expr.clone(),
            infer_guard,
        ),
        LuaType::Generic(generic) => {
            infer_generic_type_doc_function(db, config, &generic, call_expr.clone(), infer_guard)
        }
        _ => return None,
    }
}

fn infer_signature_doc_function(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    signature_id: LuaSignatureId,
    call_expr: LuaCallExpr,
) -> Option<Arc<LuaFunctionType>> {
    let signature = db.get_signature_index().get(&signature_id)?;
    let overloads = &signature.overloads;
    if overloads.is_empty() {
        let mut fake_doc_function = LuaFunctionType::new(
            false,
            signature.is_colon_define,
            signature.get_type_params(),
            vec![],
        );
        if signature.is_generic() {
            let instantiate_func = instantiate_doc_function(
                db,
                config,
                &fake_doc_function,
                call_expr,
                signature.is_colon_define,
            )?;

            fake_doc_function = instantiate_func;
        }

        Some(fake_doc_function.into())
    } else {
        let mut new_overloads = signature.overloads.clone();
        let fake_doc_function = Arc::new(LuaFunctionType::new(
            false,
            signature.is_colon_define,
            signature.get_type_params(),
            vec![],
        ));
        new_overloads.push(fake_doc_function);

        let doc_func = resolve_signature(
            db,
            config,
            new_overloads,
            call_expr.clone(),
            signature.is_colon_define,
            signature.is_generic(),
        )?;

        Some(doc_func)
    }
}

fn infer_type_doc_function(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    type_id: LuaTypeDeclId,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
) -> Option<Arc<LuaFunctionType>> {
    infer_guard.check(&type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        let alias_type = type_decl.get_alias_origin()?;
        return infer_doc_func(db, config, call_expr, alias_type.clone(), infer_guard);
    } else if type_decl.is_enum() {
        return None;
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

    let doc_func = resolve_signature(db, config, overloads, call_expr.clone(), false, false)?;
    Some(doc_func)
}

fn infer_generic_type_doc_function(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    generic: &LuaGenericType,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
) -> Option<Arc<LuaFunctionType>> {
    let type_id = generic.get_base_type_id();
    infer_guard.check(&type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        let alias_type = type_decl.get_alias_origin()?;
        return infer_doc_func(db, config, call_expr, alias_type.clone(), infer_guard);
    } else if type_decl.is_enum() {
        return None;
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

    let doc_func = resolve_signature(db, config, overloads, call_expr.clone(), false, false)?;
    Some(doc_func)
}
