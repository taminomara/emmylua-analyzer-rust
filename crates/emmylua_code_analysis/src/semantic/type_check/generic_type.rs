use crate::{DbIndex, LuaGenericType, LuaType, TypeSubstitutor};

use super::{
    check_general_type_compact, type_check_fail_reason::TypeCheckFailReason,
    type_check_guard::TypeCheckGuard, TypeCheckResult,
};

pub fn check_generic_type_compact(
    db: &DbIndex,
    source_generic: &LuaGenericType,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    // Do not check generic classes that have not been instantiated yet
    if source_generic.contain_tpl() {
        return Ok(());
    }

    let source_base_id = source_generic.get_base_type_id();
    let type_decl = db
        .get_type_index()
        .get_type_decl(&source_base_id)
        .ok_or(TypeCheckFailReason::TypeNotMatch)?;

    let type_params = source_generic.get_params();

    if type_decl.is_alias() {
        let substitutor = TypeSubstitutor::from_alias(type_params.clone(), source_base_id);
        if let Some(origin_type) = type_decl.get_alias_origin(db, Some(&substitutor)) {
            return check_general_type_compact(
                db,
                &origin_type,
                compact_type,
                check_guard.next_level()?,
            );
        }
    }

    match compact_type {
        LuaType::Generic(compact_generic) => {
            return check_generic_type_compact_generic(
                db,
                source_generic,
                compact_generic,
                check_guard.next_level()?,
            )
        }
        _ => {
            return Err(TypeCheckFailReason::TypeNotMatch);
        }
    }
}

fn check_generic_type_compact_generic(
    db: &DbIndex,
    source_generic: &LuaGenericType,
    compact_generic: &LuaGenericType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let source_base_id = source_generic.get_base_type_id();
    let compact_base_id = compact_generic.get_base_type_id();
    if source_base_id != compact_base_id {
        return Err(TypeCheckFailReason::TypeNotMatch);
    }

    let source_params = source_generic.get_params();
    let compact_params = compact_generic.get_params();
    if source_params.len() != compact_params.len() {
        return Err(TypeCheckFailReason::TypeNotMatch);
    }

    for i in 0..source_params.len() {
        let source_param = &source_params[i];
        let compact_param = &compact_params[i];
        if check_general_type_compact(db, source_param, compact_param, check_guard.next_level()?)
            .is_err()
        {
            return Err(TypeCheckFailReason::TypeNotMatch);
        }
    }

    Ok(())
}
