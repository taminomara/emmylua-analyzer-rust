use crate::{DbIndex, InferGuard, LuaType, LuaTypeDeclId};

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
    infer_guard.check(super_type_ref_id)?;

    let supers = db.get_type_index().get_super_types(sub_type_ref_id)?;
    for super_type in supers {
        if let LuaType::Ref(super_id) = super_type {
            if super_id == *super_type_ref_id {
                return Some(true);
            }

            if check_sub_type_of(db, sub_type_ref_id, &super_id, infer_guard).unwrap_or(false) {
                return Some(true);
            }
        }
    }

    Some(false)
}
