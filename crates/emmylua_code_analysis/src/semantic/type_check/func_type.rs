use crate::{
    db_index::{
        DbIndex, LuaFunctionType, LuaOperatorMetaMethod, LuaSignatureId, LuaType, LuaTypeDeclId,
    },
    semantic::{InferGuard, LuaInferConfig},
};

use super::infer_type_compact;

pub fn infer_doc_func_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source_func: &LuaFunctionType,
    compact_type: &LuaType,
    infer_guard: &mut InferGuard,
) -> bool {
    match compact_type {
        LuaType::DocFunction(compact_func) => infer_doc_func_type_compact_for_params(
            db,
            config,
            source_func,
            compact_func,
            infer_guard,
            false,
        ),
        LuaType::Signature(signature_id) => infer_doc_func_type_compact_for_signature(
            db,
            config,
            source_func,
            signature_id,
            infer_guard,
        )
        .unwrap_or(false),
        LuaType::Ref(type_id) => infer_doc_func_type_compact_for_custom_type(
            db,
            config,
            source_func,
            type_id,
            infer_guard,
        )
        .unwrap_or(false),
        LuaType::Def(type_id) => infer_doc_func_type_compact_for_custom_type(
            db,
            config,
            source_func,
            type_id,
            infer_guard,
        )
        .unwrap_or(false),
        _ => false,
    }
}

fn infer_doc_func_type_compact_for_params(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source_func: &LuaFunctionType,
    compact_func: &LuaFunctionType,
    infer_guard: &mut InferGuard,
    compact_conlon_define: bool,
) -> bool {
    let source_params = source_func.get_params();
    let mut compact_params: Vec<(String, Option<LuaType>)> =
        compact_func.get_params().iter().cloned().collect();
    if compact_conlon_define {
        compact_params.insert(0, ("self".to_string(), None));
    }

    let compact_len = compact_params.len();
    for i in 0..compact_len {
        let source_param = match source_params.get(i) {
            Some(p) => p,
            None => {
                return false;
            }
        };
        let compact_param = &compact_params[i];

        let source_param_type = &source_param.1;
        // too many complex session to handle varargs
        if source_param.0 == "..." {
            if infer_doc_func_type_compact_for_varargs(
                db,
                config,
                source_param_type,
                &compact_params[i..],
                infer_guard,
            ) {
                break;
            }

            return false;
        }

        if compact_param.0 == "..." {
            break;
        }

        let compact_param_type = &compact_param.1;

        match (source_param_type, compact_param_type) {
            (Some(source_type), Some(compact_type)) => {
                if !infer_type_compact(db, config, source_type, compact_type, infer_guard) {
                    return false;
                }
            }
            _ => {}
        }
    }
    // todo check return type
    true
}

fn infer_doc_func_type_compact_for_varargs(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    varargs: &Option<LuaType>,
    compact_params: &[(String, Option<LuaType>)],
    infer_guard: &mut InferGuard,
) -> bool {
    if let Some(varargs) = varargs {
        let varargs_len = compact_params.len();
        let varargs_type = varargs;
        for i in 0..varargs_len {
            let compact_param = &compact_params[i];
            let compact_param_type = &compact_param.1;
            if let Some(compact_param_type) = compact_param_type {
                if !infer_type_compact(db, config, varargs_type, compact_param_type, infer_guard) {
                    return false;
                }
            }
        }
    }

    true
}

fn infer_doc_func_type_compact_for_signature(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source_func: &LuaFunctionType,
    signature_id: &LuaSignatureId,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    let signature = db.get_signature_index().get(signature_id)?;
    if signature.is_generic() {
        return Some(true);
    }

    let signature_params = signature.get_type_params();
    for overload_func in &signature.overloads {
        if infer_doc_func_type_compact_for_params(
            db,
            config,
            source_func,
            overload_func,
            infer_guard,
            signature.is_colon_define,
        ) {
            return Some(true);
        }
    }

    let fake_doc_func = LuaFunctionType::new(
        false,
        signature.is_colon_define,
        signature_params.iter().cloned().collect(),
        Vec::new(),
    );
    let r = infer_doc_func_type_compact_for_params(
        db,
        config,
        &source_func,
        &fake_doc_func,
        infer_guard,
        signature.is_colon_define,
    );

    Some(r)
}

// check type is callable
fn infer_doc_func_type_compact_for_custom_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source_func: &LuaFunctionType,
    custom_type_id: &LuaTypeDeclId,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    infer_guard.check(custom_type_id)?;

    let decl_type = db.get_type_index().get_type_decl(custom_type_id)?;
    if decl_type.is_alias() {
        let alias_type = decl_type.get_alias_origin(db, None)?;
        return Some(infer_doc_func_type_compact(
            db,
            config,
            source_func,
            &alias_type,
            infer_guard,
        ));
    }

    if decl_type.is_class() {
        let operators = db
            .get_operator_index()
            .get_operators_by_type(custom_type_id)?;
        let call_operators = operators.get(&LuaOperatorMetaMethod::Call)?;
        for operator_id in call_operators {
            let operator = db.get_operator_index().get_operator(operator_id)?;
            let call_type = operator.get_call_operator_type()?;
            if infer_doc_func_type_compact(db, config, source_func, &call_type, infer_guard) {
                return Some(true);
            }
        }
    }

    Some(false)
}
