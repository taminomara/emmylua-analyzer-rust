use std::ops::Deref;

use crate::{semantic::type_check::is_sub_type_of, DbIndex, LuaType, LuaTypeDeclId, VariadicType};

use super::{
    check_general_type_compact, sub_type::get_base_type_id,
    type_check_fail_reason::TypeCheckFailReason, type_check_guard::TypeCheckGuard, TypeCheckResult,
};

pub fn check_simple_type_compact(
    db: &DbIndex,
    source: &LuaType,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    match source {
        LuaType::Unknown | LuaType::Any => return Ok(()),
        LuaType::Nil => {
            if let LuaType::Nil = compact_type {
                return Ok(());
            }
        }
        LuaType::Table | LuaType::TableConst(_) => {
            if matches!(
                compact_type,
                LuaType::Table
                    | LuaType::TableConst(_)
                    | LuaType::Tuple(_)
                    | LuaType::Array(_)
                    | LuaType::Object(_)
                    | LuaType::Ref(_)
                    | LuaType::Def(_)
                    | LuaType::TableGeneric(_)
                    | LuaType::Generic(_)
                    | LuaType::Global
                    | LuaType::Userdata
                    | LuaType::Instance(_)
                    | LuaType::Any
            ) {
                return Ok(());
            }
        }
        LuaType::Userdata => {
            if matches!(
                compact_type,
                LuaType::Userdata | LuaType::Ref(_) | LuaType::Def(_)
            ) {
                return Ok(());
            }
        }
        LuaType::Function => {
            if matches!(
                compact_type,
                LuaType::Function | LuaType::DocFunction(_) | LuaType::Signature(_)
            ) {
                return Ok(());
            }
        }
        LuaType::Thread => {
            if let LuaType::Thread = compact_type {
                return Ok(());
            }
        }
        LuaType::Boolean | LuaType::BooleanConst(_) => {
            if compact_type.is_boolean() {
                return Ok(());
            }
        }
        LuaType::String | LuaType::StringConst(_) => match compact_type {
            LuaType::String
            | LuaType::StringConst(_)
            | LuaType::DocStringConst(_)
            | LuaType::StrTplRef(_) => {
                return Ok(());
            }
            LuaType::Ref(_) => {
                if check_base_type_for_ref_compact(db, source, compact_type, check_guard).is_ok() {
                    return Ok(());
                }
            }
            LuaType::Def(id) => {
                if id.get_name() == "string" {
                    return Ok(());
                }
            }
            _ => {}
        },
        LuaType::Integer | LuaType::IntegerConst(_) => match compact_type {
            LuaType::Integer | LuaType::IntegerConst(_) | LuaType::DocIntegerConst(_) => {
                return Ok(());
            }
            LuaType::Ref(_) => {
                if check_base_type_for_ref_compact(db, source, compact_type, check_guard).is_ok() {
                    return Ok(());
                }
            }
            _ => {}
        },
        LuaType::Number | LuaType::FloatConst(_) => {
            if matches!(
                compact_type,
                LuaType::Number
                    | LuaType::FloatConst(_)
                    | LuaType::Integer
                    | LuaType::IntegerConst(_)
                    | LuaType::DocIntegerConst(_)
            ) {
                return Ok(());
            }
        }
        LuaType::Io => {
            if let LuaType::Io = compact_type {
                return Ok(());
            }
        }
        LuaType::Global => {
            if let LuaType::Global = compact_type {
                return Ok(());
            }
        }
        LuaType::DocIntegerConst(i) => match compact_type {
            LuaType::IntegerConst(j) => {
                if i == j {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            LuaType::Integer => return Err(TypeCheckFailReason::TypeNotMatch),
            LuaType::DocIntegerConst(j) => {
                if i == j {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            _ => {}
        },
        LuaType::DocStringConst(s) => match compact_type {
            LuaType::StringConst(t) => {
                if s == t {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            LuaType::String => return Err(TypeCheckFailReason::TypeNotMatch),
            LuaType::DocStringConst(t) => {
                if s == t {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            _ => {}
        },
        LuaType::DocBooleanConst(b) => match compact_type {
            LuaType::BooleanConst(t) => {
                if b == t {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            LuaType::Boolean => return Err(TypeCheckFailReason::TypeNotMatch),
            LuaType::DocBooleanConst(t) => {
                if b == t {
                    return Ok(());
                }

                return Err(TypeCheckFailReason::TypeNotMatch);
            }
            _ => {}
        },
        LuaType::StrTplRef(_) => {
            if compact_type.is_string() {
                return Ok(());
            }
        }
        LuaType::TplRef(_) => return Ok(()),
        LuaType::Namespace(source_namespace) => {
            if let LuaType::Namespace(compact_namespace) = compact_type {
                if source_namespace == compact_namespace {
                    return Ok(());
                }
            }
        }
        LuaType::Variadic(source_type) => {
            return check_variadic_type_compact(db, source_type, compact_type, check_guard);
        }
        _ => {}
    }

    if let LuaType::Union(union) = compact_type {
        for sub_compact in union.get_types() {
            if check_simple_type_compact(db, source, sub_compact, check_guard.next_level()?)
                .is_err()
            {
                return Err(TypeCheckFailReason::TypeNotMatch);
            }
        }

        return Ok(());
    }

    // complex infer
    Err(TypeCheckFailReason::TypeNotMatch)
}

fn get_alias_real_type<'a>(db: &'a DbIndex, compact_type: &'a LuaType) -> Option<&'a LuaType> {
    match compact_type {
        LuaType::Ref(type_decl_id) => {
            let type_decl = db.get_type_index().get_type_decl(type_decl_id)?;
            if type_decl.is_alias() {
                return get_alias_real_type(db, type_decl.get_alias_ref()?);
            }
            Some(compact_type)
        }
        _ => Some(compact_type),
    }
}

fn check_base_type_for_ref_compact(
    db: &DbIndex,
    source: &LuaType,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    if let LuaType::Ref(_) = compact_type {
        let real_type = get_alias_real_type(db, compact_type).unwrap_or(compact_type);
        match &real_type {
            LuaType::MultiLineUnion(multi_line_union) => {
                if multi_line_union.get_unions().iter().all(|(t, _)| {
                    let next_guard = check_guard.next_level();
                    match next_guard {
                        Ok(guard) => check_general_type_compact(db, source, t, guard).is_ok(),
                        Err(_) => return false,
                    }
                }) {
                    return Ok(());
                }
            }
            LuaType::Ref(type_decl_id) => {
                if let Some(source_id) = get_base_type_id(&source) {
                    if is_sub_type_of(db, type_decl_id, &source_id) {
                        return Ok(());
                    }
                }
                if let Some(decl) = db.get_type_index().get_type_decl(type_decl_id) {
                    if decl.is_enum() {
                        if check_enum_fields_match_source(db, source, type_decl_id, check_guard)
                            .is_ok()
                        {
                            return Ok(());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Err(TypeCheckFailReason::TypeNotMatch)
}

/// 检查`enum`的所有字段是否匹配`source`
fn check_enum_fields_match_source(
    db: &DbIndex,
    source: &LuaType,
    enum_type_decl_id: &LuaTypeDeclId,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    if let Some(decl) = db.get_type_index().get_type_decl(enum_type_decl_id) {
        if let Some(LuaType::Union(enum_fields)) = decl.get_enum_field_type(db) {
            let is_match = enum_fields.get_types().iter().all(|field| {
                let next_guard = check_guard.next_level();
                match next_guard {
                    Ok(guard) => check_general_type_compact(db, source, field, guard).is_ok(),
                    Err(_) => return false,
                }
            });
            if is_match {
                return Ok(());
            }
        }
    }
    Err(TypeCheckFailReason::TypeNotMatch)
}

fn check_variadic_type_compact(
    db: &DbIndex,
    source_type: &VariadicType,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    match &source_type {
        VariadicType::Base(source_base) => {
            if let LuaType::Variadic(compact_variadic) = compact_type {
                match compact_variadic.deref() {
                    VariadicType::Base(compact_base) => {
                        if source_base == compact_base {
                            return Ok(());
                        }
                    }
                    VariadicType::Multi(compact_multi) => {
                        for compact_type in compact_multi {
                            if check_simple_type_compact(
                                db,
                                source_base,
                                compact_type,
                                check_guard.next_level()?,
                            )
                            .is_err()
                            {
                                return Err(TypeCheckFailReason::TypeNotMatch);
                            }
                        }
                    }
                }
            }
        }
        VariadicType::Multi(_) => {}
    }

    Ok(())
}
