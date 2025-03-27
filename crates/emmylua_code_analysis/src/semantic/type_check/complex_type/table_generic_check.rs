use crate::{
    check_type_compact,
    semantic::type_check::{check_general_type_compact, type_check_guard::TypeCheckGuard},
    DbIndex, LuaMemberKey, LuaMemberOwner, LuaType, TypeCheckFailReason, TypeCheckResult,
};

pub fn check_table_generic_type_compact(
    db: &DbIndex,
    source_generic_param: &Vec<LuaType>,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    match compact_type {
        LuaType::Table | LuaType::Global => return Ok(()),
        LuaType::TableGeneric(compact_generic_param) => {
            if source_generic_param.len() == 2 && compact_generic_param.len() == 2 {
                let source_key = &source_generic_param[0];
                let source_value = &source_generic_param[1];
                let compact_key = &compact_generic_param[0];
                let compact_value = &compact_generic_param[1];

                if check_general_type_compact(
                    db,
                    source_key,
                    compact_key,
                    check_guard.next_level()?,
                )
                .is_err()
                    || check_general_type_compact(
                        db,
                        source_value,
                        compact_value,
                        check_guard.next_level()?,
                    )
                    .is_err()
                {
                    return Err(TypeCheckFailReason::TypeNotMatch);
                }
                return Ok(());
            }
        }
        LuaType::TableConst(inst) => {
            let table_member_owner = LuaMemberOwner::Element(inst.clone());
            return check_table_generic_compact_member_owner(
                db,
                source_generic_param,
                table_member_owner,
                check_guard.next_level()?,
            );
        }
        LuaType::Array(base) => {
            if source_generic_param.len() == 2 {
                let key = &source_generic_param[0];
                let value = &source_generic_param[1];
                if key.is_any() || key.is_integer() && check_type_compact(db, value, base).is_ok() {
                    return Ok(());
                }
            }
        }
        LuaType::Tuple(tuple) => {
            if source_generic_param.len() == 2 {
                let key = &source_generic_param[0];
                let value = &source_generic_param[1];
                if key.is_any() {
                    for tuple_type in tuple.get_types() {
                        if check_general_type_compact(
                            db,
                            value,
                            tuple_type,
                            check_guard.next_level()?,
                        )
                        .is_err()
                        {
                            return Err(TypeCheckFailReason::TypeNotMatch);
                        }
                    }

                    return Ok(());
                }

                return Ok(());
            }
        }
        // maybe support object
        // need check later
        LuaType::Ref(_) | LuaType::Def(_) | LuaType::Userdata => return Ok(()),
        _ => {}
    }

    Err(TypeCheckFailReason::TypeNotMatch)
}

fn check_table_generic_compact_member_owner(
    db: &DbIndex,
    source_generic_params: &Vec<LuaType>,
    member_owner: LuaMemberOwner,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    if source_generic_params.len() != 2 {
        return Err(TypeCheckFailReason::TypeNotMatch);
    }

    let member_index = db.get_member_index();
    let members = match member_index.get_members(&member_owner) {
        Some(members) => members,
        None => return Ok(()),
    };

    let source_key = &source_generic_params[0];
    let source_value = &source_generic_params[1];

    for member in members {
        let key = member.get_key();
        let key_type = match key {
            LuaMemberKey::Integer(i) => LuaType::IntegerConst(*i),
            LuaMemberKey::Name(s) => LuaType::StringConst(s.clone().into()),
            _ => LuaType::Any,
        };

        let member_type = member.get_decl_type();
        if check_general_type_compact(db, source_key, &key_type, check_guard.next_level()?).is_err()
            || check_general_type_compact(db, source_value, &member_type, check_guard.next_level()?)
                .is_err()
        {
            return Err(TypeCheckFailReason::TypeNotMatch);
        }
    }

    Ok(())
}
