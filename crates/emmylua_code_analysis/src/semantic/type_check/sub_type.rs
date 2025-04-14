use crate::{DbIndex, InferGuard, LuaType, LuaTypeDeclId};
use lazy_static::lazy_static;

pub fn is_sub_type_of(
    db: &DbIndex,
    sub_type_ref_id: &LuaTypeDeclId,
    super_type_ref_id: &LuaTypeDeclId,
) -> bool {
    let mut infer_guard = InferGuard::new();
    check_sub_type_of(db, sub_type_ref_id, super_type_ref_id, &mut infer_guard).unwrap_or(false)
}

fn check_sub_type_of(
    db: &DbIndex,
    sub_type_ref_id: &LuaTypeDeclId,
    super_type_ref_id: &LuaTypeDeclId,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    infer_guard.check(super_type_ref_id).ok()?;

    let supers = db.get_type_index().get_super_types(sub_type_ref_id)?;
    for super_type in supers {
        if let LuaType::Ref(super_id) = &super_type {
            if super_id == super_type_ref_id {
                return Some(true);
            }

            if check_sub_type_of(db, sub_type_ref_id, &super_id, infer_guard).unwrap_or(false) {
                return Some(true);
            }
        }
        if let Some(super_base_type_id) = get_base_type_id(&super_type) {
            if super_base_type_id == super_type_ref_id {
                return Some(true);
            }
        }
    }

    Some(false)
}

lazy_static! {
    static ref INTEGER_ID: LuaTypeDeclId = LuaTypeDeclId::new("integer");
    static ref NUMBER_ID: LuaTypeDeclId = LuaTypeDeclId::new("number");
    static ref BOOLEAN_ID: LuaTypeDeclId = LuaTypeDeclId::new("boolean");
    static ref STRING_ID: LuaTypeDeclId = LuaTypeDeclId::new("string");
    static ref TABLE_ID: LuaTypeDeclId = LuaTypeDeclId::new("table");
    static ref FUNCTION_ID: LuaTypeDeclId = LuaTypeDeclId::new("function");
    static ref THREAD_ID: LuaTypeDeclId = LuaTypeDeclId::new("thread");
    static ref USERDATA_ID: LuaTypeDeclId = LuaTypeDeclId::new("userdata");
    static ref IO_ID: LuaTypeDeclId = LuaTypeDeclId::new("io");
    static ref GLOBAL_ID: LuaTypeDeclId = LuaTypeDeclId::new("global");
    static ref SELF_ID: LuaTypeDeclId = LuaTypeDeclId::new("self");
    static ref NIL_ID: LuaTypeDeclId = LuaTypeDeclId::new("nil");
}

pub fn get_base_type_id(typ: &LuaType) -> Option<&'static LuaTypeDeclId> {
    match typ {
        _ if typ.is_integer() => Some(&INTEGER_ID),
        _ if typ.is_number() => Some(&NUMBER_ID),
        _ if typ.is_boolean() => Some(&BOOLEAN_ID),
        _ if typ.is_string() => Some(&STRING_ID),
        _ if typ.is_table() => Some(&TABLE_ID),
        _ if typ.is_function() => Some(&FUNCTION_ID),
        _ if typ.is_thread() => Some(&THREAD_ID),
        _ if typ.is_userdata() => Some(&USERDATA_ID),
        _ if typ.is_io() => Some(&IO_ID),
        _ if typ.is_global() => Some(&GLOBAL_ID),
        _ if typ.is_self_infer() => Some(&SELF_ID),
        _ if typ.is_nil() => Some(&NIL_ID),
        _ => None,
    }
}
