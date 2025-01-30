use std::{collections::HashMap, ops::Deref};

use emmylua_parser::{LuaAstNode, LuaSyntaxId, LuaSyntaxNode, LuaTableExpr};
use smol_str::SmolStr;

use crate::{
    db_index::{DbIndex, LuaGenericType, LuaType},
    semantic::{infer_expr, LuaInferConfig},
    LuaFunctionType, LuaUnionType,
};

#[allow(unused)]
pub fn tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    pattern: &LuaType,
    target: &LuaType,
    result: &mut HashMap<usize, LuaType>,
) -> Option<()> {
    match pattern {
        LuaType::FuncTplRef(tpl) => {
            result.insert(tpl.get_tpl_id(), target.clone());
        }
        LuaType::StrTplRef(str_tpl) => match target {
            LuaType::StringConst(s) => {
                let prefix = str_tpl.get_prefix();
                let type_name = if prefix.is_empty() {
                    s.deref().clone()
                } else {
                    SmolStr::new(format!("{}{}", prefix, s))
                };
                result.insert(str_tpl.get_usize(), type_name.into());
            }
            _ => {}
        },
        LuaType::Array(base) => {
            array_tpl_pattern_match(db, config, root, base, target, result);
        }
        LuaType::TableGeneric(table_generic_params) => {
            table_tpl_pattern_match(db, config, root, table_generic_params, target, result);
        }
        LuaType::Nullable(origin) => {
            tpl_pattern_match(db, config, root, &origin, target, result);
        }
        LuaType::Generic(generic) => {
            generic_tpl_pattern_match(db, config, root, generic, target, result);
        }
        LuaType::Union(union) => {
            union_tpl_pattern_match(db, config, root, union, target, result);
        }
        LuaType::DocFunction(doc_func) => {
            func_tpl_pattern_match(db, config, root, doc_func, target, result);
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
    result: &mut HashMap<usize, LuaType>,
) -> Option<()> {
    match target {
        LuaType::Array(target_base) => {
            tpl_pattern_match(db, config, root, base, target_base, result);
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
    result: &mut HashMap<usize, LuaType>,
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
                    result,
                );
            }
        }
        LuaType::TableConst(target_range) => {
            let node = LuaSyntaxId::to_node_at_range(root, target_range.value)?;
            let table_node = LuaTableExpr::cast(node)?;
            let t1 = &table_generic_params[0];
            let t2 = &table_generic_params[1];
            if table_node.is_array() {
                tpl_pattern_match(db, config, root, &t1, &LuaType::Integer, result);
            } else {
                tpl_pattern_match(db, config, root, &t1, &LuaType::String, result);
            }

            let first_field = table_node.get_fields().next()?;
            let expr_type = infer_expr(db, config, first_field.get_value_expr()?)?;
            tpl_pattern_match(db, config, root, t2, &expr_type, result);
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
    result: &mut HashMap<usize, LuaType>,
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
                tpl_pattern_match(db, config, root, &params[i], &target_params[i], result);
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
    result: &mut HashMap<usize, LuaType>,
) -> Option<()> {
    for u in union.get_types() {
        tpl_pattern_match(db, config, root, u, target, result);
    }

    Some(())
}

#[allow(unused)]
fn func_tpl_pattern_match(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    root: &LuaSyntaxNode,
    doc_func: &LuaFunctionType,
    target: &LuaType,
    result: &mut HashMap<usize, LuaType>,
) -> Option<()> {
    match target {
        LuaType::DocFunction(target_doc_func) => {
            let params = doc_func.get_params();
            let target_params = target_doc_func.get_params();
            for (i, param_tuple) in params.iter().enumerate() {
                let target_param_tuple = target_params.get(i)?;
                if param_tuple.1.is_some() && target_param_tuple.1.is_some() {
                    let param_type = param_tuple.1.clone()?;
                    let target_param_type = target_param_tuple.1.clone()?;
                    tpl_pattern_match(db, config, root, &param_type, &target_param_type, result);
                }
            }

            let rets = doc_func.get_ret();
            for (i, ret_type) in rets.iter().enumerate() {
                if let Some(target_ret_type) = &target_doc_func.get_ret().get(i) {
                    tpl_pattern_match(db, config, root, ret_type, target_ret_type, result);
                }
            }
        }
        LuaType::Signature(signature_id) => {
            let params = doc_func.get_params();
            let signature = db.get_signature_index().get(&signature_id)?;

            for (i, param_tuple) in params.iter().enumerate() {
                let signature_param_info = signature.get_param_info_by_id(i);
                if param_tuple.1.is_some() && signature_param_info.is_some() {
                    let param_type = param_tuple.1.clone()?;
                    let target_param_type = &signature_param_info.unwrap().type_ref;
                    tpl_pattern_match(db, config, root, &param_type, target_param_type, result);
                }
            }

            let rets = doc_func.get_ret();
            for (i, ret_type) in rets.iter().enumerate() {
                if let Some(signature_ret_info) = &signature.return_docs.get(i) {
                    tpl_pattern_match(db, config, root, ret_type, &signature_ret_info.type_ref, result);
                }
            }
        }
        _ => {}
    }

    Some(())
}
