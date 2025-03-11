use std::collections::{HashMap, HashSet};

use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaIndexExpr, LuaIndexKey, LuaVarExpr};

use crate::{DiagnosticCode, LuaMemberOwner, LuaType, SemanticModel};

use super::{get_lint_type_name, DiagnosticContext};

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::InjectField];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    let mut type_cache = HashMap::new();
    for stat in root.descendants::<LuaAssignStat>() {
        let (vars, _) = stat.get_var_and_expr_list();
        for var in vars.iter() {
            if let LuaVarExpr::IndexExpr(index_expr) = var {
                check_inject_field(context, semantic_model, index_expr, &mut type_cache);
            }
        }
    }
    Some(())
}

fn check_inject_field(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    index_expr: &LuaIndexExpr,
    type_cache: &mut HashMap<LuaType, (HashSet<String>, HashSet<LuaType>)>,
) -> Option<()> {
    let db = context.db;
    let prefix_expr = index_expr.get_prefix_expr()?;
    let typ = semantic_model.infer_expr(prefix_expr)?;
    let index_key = index_expr.get_index_key()?;
    let index_name = index_key.get_path_part();

    let (field_names, index_access_keys) = match &typ {
        LuaType::Ref(type_decl_id) => type_cache.entry(typ.clone()).or_insert_with(|| {
            let types = type_decl_id.collect_super_types_with_self(context.db, typ.clone());
            get_all_field_info(context, &types).unwrap_or_default()
        }),
        LuaType::Generic(generic_type) => {
            let type_decl_id = generic_type.get_base_type_id();
            type_cache.entry(typ.clone()).or_insert_with(|| {
                let types = type_decl_id.collect_super_types_with_self(context.db, typ.clone());
                get_all_field_info(context, &types).unwrap_or_default()
            })
        }
        LuaType::Def(_) => {
            return Some(());
        }
        _ => type_cache
            .entry(typ.clone())
            .or_insert_with(|| get_all_field_info(context, &vec![typ.clone()]).unwrap_or_default()),
    };

    if !field_names.contains(&index_name) {
        let mut need_diagnostic = true;
        if !index_access_keys.is_empty() {
            let index_type = match &index_key {
                LuaIndexKey::Name(_) => LuaType::String,
                LuaIndexKey::String(_) => LuaType::String,
                LuaIndexKey::Integer(_) => LuaType::Integer,
                LuaIndexKey::Expr(key_expr) => semantic_model
                    .infer_expr(key_expr.clone())
                    .unwrap_or(LuaType::Any),
                LuaIndexKey::Idx(_) => LuaType::Integer,
            };
            for index_access_key in index_access_keys.iter() {
                if semantic_model
                    .type_check(&index_access_key, &index_type)
                    .is_ok()
                {
                    need_diagnostic = false;
                    break;
                }
            }
        }

        if need_diagnostic {
            context.add_diagnostic(
                DiagnosticCode::InjectField,
                index_key.get_range()?,
                t!(
                    "Fields cannot be injected into the reference of `%{class}` for `%{field}`. ",
                    class = get_lint_type_name(&db, &typ),
                    field = index_name,
                )
                .to_string(),
                None,
            );
        }
    }

    Some(())
}

fn get_all_field_info(
    context: &mut DiagnosticContext,
    types: &Vec<LuaType>,
) -> Option<(HashSet<String>, HashSet<LuaType>)> {
    let member_index = context.db.get_member_index();
    let mut field_names: HashSet<String> = HashSet::new();
    let mut index_access_keys: HashSet<LuaType> = HashSet::new();

    for cur_type in types {
        let type_decl_id = match cur_type {
            LuaType::Ref(type_decl_id) => type_decl_id.clone(),
            LuaType::Generic(generic_type) => generic_type.get_base_type_id().clone(),
            // 处理 ---@class test: { a: number }
            LuaType::Object(object_type) => {
                let fields = object_type.get_fields();
                for (key, _) in fields {
                    let name = key.to_path();
                    if name.is_empty() {
                        continue;
                    }
                    field_names.insert(name);
                }

                for (key, _) in object_type.get_index_access() {
                    index_access_keys.insert(key.clone());
                }
                continue;
            }
            // 处理 ---@class test: table<string, boolean>
            LuaType::TableGeneric(table_type) => {
                if let Some(key_type) = table_type.get(0) {
                    index_access_keys.insert(key_type.clone());
                }
                continue;
            }
            _ => continue,
        };
        if let Some(member_map) =
            member_index.get_member_map(&LuaMemberOwner::Type(type_decl_id.clone()))
        {
            for (key, _) in member_map {
                let name = key.to_path();
                if name.is_empty() {
                    continue;
                }
                field_names.insert(name);
            }
        }
    }

    Some((field_names, index_access_keys))
}
