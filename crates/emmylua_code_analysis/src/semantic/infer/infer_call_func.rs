use std::sync::Arc;

use emmylua_parser::{LuaAstNode, LuaCallExpr};

use crate::{
    CacheEntry, CacheKey, DbIndex, LuaFunctionType, LuaGenericType, LuaOperatorMetaMethod,
    LuaSignatureId, LuaType, LuaTypeDeclId, LuaUnionType,
};

use super::super::{
    instantiate::{instantiate_func_generic, TypeSubstitutor},
    instantiate_type, resolve_signature, InferGuard, LuaInferCache,
};

pub fn infer_call_expr_func(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    call_expr: LuaCallExpr,
    call_expr_type: LuaType,
    infer_guard: &mut InferGuard,
    args_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let syntax_id = call_expr.get_syntax_id();
    let key = CacheKey::Call(syntax_id, args_count);
    match cache.get(&key) {
        Some(cache) => match cache {
            CacheEntry::CallCache(ty) => return Some(ty.clone()),
            _ => return None,
        },
        None => {}
    }

    cache.ready_cache(&key);
    let result = match call_expr_type {
        LuaType::DocFunction(func) => Some(func),
        LuaType::Signature(signature_id) => infer_signature_doc_function(
            db,
            cache,
            signature_id.clone(),
            call_expr.clone(),
            args_count,
        ),
        LuaType::Def(type_def_id) => infer_type_doc_function(
            db,
            cache,
            type_def_id.clone(),
            call_expr.clone(),
            infer_guard,
            args_count,
        ),
        LuaType::Ref(type_ref_id) => infer_type_doc_function(
            db,
            cache,
            type_ref_id.clone(),
            call_expr.clone(),
            infer_guard,
            args_count,
        ),
        LuaType::Generic(generic) => infer_generic_type_doc_function(
            db,
            cache,
            &generic,
            call_expr.clone(),
            infer_guard,
            args_count,
        ),
        LuaType::Union(union) => {
            // 此时我们将其视为泛型实例化联合体
            if union.get_types().len() > 1
                && union
                    .get_types()
                    .iter()
                    .all(|t| matches!(t, LuaType::DocFunction(_)))
            {
                infer_generic_doc_function_union(db, cache, &union, call_expr, args_count)
            } else {
                None
            }
        }
        _ => return None,
    };

    if let Some(result_type) = &result {
        cache.add_cache(&key, CacheEntry::CallCache(result_type.clone()));
    } else {
        cache.remove(&key);
    }

    result
}

fn infer_generic_doc_function_union(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    union: &LuaUnionType,
    call_expr: LuaCallExpr,
    args_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let overloads = union
        .get_types()
        .iter()
        .filter_map(|typ| match typ {
            LuaType::DocFunction(f) => Some(f.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let doc_func = resolve_signature(db, cache, overloads, call_expr.clone(), false, args_count)?;

    Some(doc_func)
}

fn infer_signature_doc_function(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    signature_id: LuaSignatureId,
    call_expr: LuaCallExpr,
    args_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let signature = db.get_signature_index().get(&signature_id)?;
    let overloads = &signature.overloads;
    if overloads.is_empty() {
        let mut fake_doc_function = LuaFunctionType::new(
            signature.is_async,
            signature.is_colon_define,
            signature.get_type_params(),
            vec![],
        );
        if signature.is_generic() {
            fake_doc_function = instantiate_func_generic(db, cache, &fake_doc_function, call_expr)?;
        }

        Some(fake_doc_function.into())
    } else {
        let mut new_overloads = signature.overloads.clone();
        let fake_doc_function = Arc::new(LuaFunctionType::new(
            signature.is_async,
            signature.is_colon_define,
            signature.get_type_params(),
            vec![],
        ));
        new_overloads.push(fake_doc_function);

        let doc_func = resolve_signature(
            db,
            cache,
            new_overloads,
            call_expr.clone(),
            signature.is_generic(),
            args_count,
        )?;

        Some(doc_func)
    }
}

fn infer_type_doc_function(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    type_id: LuaTypeDeclId,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
    args_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    infer_guard.check(&type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        let origin_type = type_decl.get_alias_origin(db, None)?;
        return infer_call_expr_func(
            db,
            cache,
            call_expr,
            origin_type.clone(),
            infer_guard,
            args_count,
        );
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

    let doc_func = resolve_signature(db, cache, overloads, call_expr.clone(), false, args_count)?;
    Some(doc_func)
}

fn infer_generic_type_doc_function(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    generic: &LuaGenericType,
    call_expr: LuaCallExpr,
    infer_guard: &mut InferGuard,
    args_count: Option<usize>,
) -> Option<Arc<LuaFunctionType>> {
    let type_id = generic.get_base_type_id();
    infer_guard.check(&type_id)?;
    let generic_params = generic.get_params();
    let substitutor = TypeSubstitutor::from_type_array(generic_params.clone());

    let type_decl = db.get_type_index().get_type_decl(&type_id)?;
    if type_decl.is_alias() {
        let origin_type = type_decl.get_alias_origin(db, Some(&substitutor))?;
        return infer_call_expr_func(
            db,
            cache,
            call_expr,
            origin_type.clone(),
            infer_guard,
            args_count,
        );
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
        let new_f = instantiate_type(db, func, &substitutor);
        match new_f {
            LuaType::DocFunction(f) => {
                overloads.push(f.clone());
            }
            _ => {}
        }
    }

    let doc_func = resolve_signature(db, cache, overloads, call_expr.clone(), false, args_count)?;
    Some(doc_func)
}
