use std::ops::Deref;

use emmylua_parser::{LuaAstNode, LuaSyntaxId, LuaSyntaxNode, LuaTableExpr};
use smol_str::SmolStr;

use crate::{
    db_index::{DbIndex, LuaGenericType, LuaType},
    semantic::{infer_expr, LuaInferConfig},
    LuaFunctionType, LuaUnionType,
};

use super::type_substitutor::TypeSubstitutor;

pub fn tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    pattern: &LuaType,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    match pattern {
        LuaType::TplRef(tpl) => {
            if tpl.get_tpl_id().is_func() {
                substitutor.insert(tpl.get_tpl_id(), target.clone());
            }
        }
        LuaType::StrTplRef(str_tpl) => match target {
            LuaType::StringConst(s) => {
                let prefix = str_tpl.get_prefix();
                let type_name = if prefix.is_empty() {
                    s.deref().clone()
                } else {
                    SmolStr::new(format!("{}{}", prefix, s))
                };
                substitutor.insert(str_tpl.get_tpl_id(), type_name.into());
            }
            _ => {}
        },
        LuaType::Array(base) => {
            array_tpl_pattern_match(db, config, root, base, target, substitutor);
        }
        LuaType::TableGeneric(table_generic_params) => {
            table_tpl_pattern_match(db, config, root, table_generic_params, target, substitutor);
        }
        LuaType::Nullable(origin) => {
            tpl_pattern_match(db, config, root, &origin, target, substitutor);
        }
        LuaType::Generic(generic) => {
            generic_tpl_pattern_match(db, config, root, generic, target, substitutor);
        }
        LuaType::Union(union) => {
            union_tpl_pattern_match(db, config, root, union, target, substitutor);
        }
        LuaType::DocFunction(doc_func) => {
            func_tpl_pattern_match(db, config, root, doc_func, target, substitutor);
        }
        _ => {}
    }

    Some(())
}

fn array_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    base: &LuaType,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    match target {
        LuaType::Array(target_base) => {
            tpl_pattern_match(db, config, root, base, target_base, substitutor);
        }
        _ => {}
    }

    Some(())
}

fn table_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    table_generic_params: &Vec<LuaType>,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    if table_generic_params.len() != 2 {
        return None;
    }

    match target {
        LuaType::TableGeneric(target_table_generic_params) => {
            let min_len = table_generic_params
                .len()
                .min(target_table_generic_params.len());
            for i in 0..min_len {
                tpl_pattern_match(
                    db,
                    config,
                    root,
                    &table_generic_params[i],
                    &target_table_generic_params[i],
                    substitutor,
                );
            }
        }
        LuaType::TableConst(target_range) => {
            let node = LuaSyntaxId::to_node_at_range(root, target_range.value)?;
            let table_node = LuaTableExpr::cast(node)?;
            let t1 = &table_generic_params[0];
            let t2 = &table_generic_params[1];
            if table_node.is_array() {
                tpl_pattern_match(db, config, root, &t1, &LuaType::Integer, substitutor);
            } else {
                tpl_pattern_match(db, config, root, &t1, &LuaType::String, substitutor);
            }

            let first_field = table_node.get_fields().next()?;
            let expr_type = infer_expr(db, config, first_field.get_value_expr()?)?;
            tpl_pattern_match(db, config, root, t2, &expr_type, substitutor);
        }
        _ => {}
    }

    Some(())
}

fn generic_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    generic: &LuaGenericType,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    match target {
        LuaType::Generic(target_generic) => {
            let base = generic.get_base_type();
            let target_base = target_generic.get_base_type();
            if target_base != base {
                return None;
            }

            let params = generic.get_params();
            let target_params = target_generic.get_params();
            let min_len = params.len().min(target_params.len());
            for i in 0..min_len {
                tpl_pattern_match(db, config, root, &params[i], &target_params[i], substitutor);
            }
        }
        _ => {}
    }

    Some(())
}

fn union_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    union: &LuaUnionType,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    for u in union.get_types() {
        tpl_pattern_match(db, config, root, u, target, substitutor);
    }

    Some(())
}

fn func_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    tpl_func: &LuaFunctionType,
    target: &LuaType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    match target {
        LuaType::DocFunction(target_doc_func) => {
            func_tpl_pattern_match_doc_func(
                db,
                config,
                root,
                tpl_func,
                target_doc_func,
                substitutor,
            );
        }
        LuaType::Signature(signature_id) => {
            let signature = db.get_signature_index().get(&signature_id)?;
            let typed_params = signature.get_type_params();
            let rets = signature.get_return_types();
            let fake_doc_func =
                LuaFunctionType::new(false, signature.is_colon_define, typed_params, rets);
            func_tpl_pattern_match_doc_func(
                db,
                config,
                root,
                tpl_func,
                &fake_doc_func,
                substitutor,
            );
        }
        _ => {}
    }

    Some(())
}

fn func_tpl_pattern_match_doc_func(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    tpl_func: &LuaFunctionType,
    target_func: &LuaFunctionType,
    substitutor: &mut TypeSubstitutor,
) -> Option<()> {
    let tpl_func_params = tpl_func.get_params();
    let target_func_params = target_func.get_params();
    let len = tpl_func_params.len();
    for i in 0..len {
        let tpl_param_tuple = tpl_func_params.get(i)?;
        let target_param_tuple = match target_func_params.get(i) {
            Some(t) => t,
            None => break,
        };

        let tpl_param_type = tpl_param_tuple.1.clone().unwrap_or(LuaType::Any);
        let target_param_type = target_param_tuple.1.clone().unwrap_or(LuaType::Any);
        tpl_pattern_match(
            db,
            config,
            root,
            &tpl_param_type,
            &target_param_type,
            substitutor,
        );
    }

    let rets = tpl_func.get_ret();
    for (i, ret_type) in rets.iter().enumerate() {
        if let Some(target_ret_type) = &target_func.get_ret().get(i) {
            tpl_pattern_match(db, config, root, ret_type, target_ret_type, substitutor);
        }
    }

    Some(())
}
