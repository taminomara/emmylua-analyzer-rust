use std::collections::HashMap;

use emmylua_parser::{LuaAstNode, LuaSyntaxId, LuaSyntaxNode, LuaTableExpr};
use internment::ArcIntern;

use crate::{
    db_index::{DbIndex, LuaGenericType, LuaType},
    semantic::{infer_expr, LuaInferConfig},
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
        LuaType::FuncTplRef(size) => {
            result.insert(*size, target.clone());
        }
        LuaType::StrTplRef(str_tpl) => match target {
            LuaType::StringConst(s) => {
                let prefix = str_tpl.get_prefix();
                let type_name = if prefix.is_empty() {
                    s.clone()
                } else {
                    ArcIntern::new(format!("{}.{}", prefix, s))
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