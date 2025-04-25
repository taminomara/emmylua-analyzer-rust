use std::collections::HashMap;

use crate::{
    humanize_type, semantic::member::infer_members, DbIndex, LuaMemberKey, LuaMemberOwner,
    LuaObjectType, LuaType, LuaTypeCache, LuaTypeDeclId, LuaUnionType, RenderLevel,
};

use super::{
    check_general_type_compact, is_sub_type_of, sub_type::get_base_type_id,
    type_check_fail_reason::TypeCheckFailReason, type_check_guard::TypeCheckGuard, TypeCheckResult,
};

pub fn check_ref_type_compact(
    db: &DbIndex,
    source_id: &LuaTypeDeclId,
    compact_type: &LuaType,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let type_decl = db
        .get_type_index()
        .get_type_decl(source_id)
        // unreachable!
        .ok_or(TypeCheckFailReason::TypeNotMatchWithReason(
            t!("type `%{name}` not found.", name = source_id.get_name()).to_string(),
        ))?;

    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin(db, None) {
            return check_general_type_compact(
                db,
                &origin_type,
                compact_type,
                check_guard.next_level()?,
            );
        }

        return Err(TypeCheckFailReason::TypeNotMatch);
    }

    if type_decl.is_enum() {
        match compact_type {
            LuaType::Def(compact_id) => {
                if source_id == compact_id {
                    return Ok(());
                }
            }
            LuaType::Ref(compact_id) => {
                if source_id == compact_id {
                    return Ok(());
                }
            }
            _ => {}
        };

        let enum_member_owner = LuaMemberOwner::Type(source_id.clone());
        let enum_members = db
            .get_member_index()
            .get_members(&enum_member_owner)
            .ok_or(TypeCheckFailReason::TypeNotMatch)?;

        let mut union_types = Vec::new();
        if type_decl.is_enum_key() {
            for enum_member in enum_members {
                let member_key = enum_member.get_key();
                let fake_type = match member_key {
                    LuaMemberKey::Name(name) => LuaType::DocStringConst(name.clone().into()),
                    LuaMemberKey::Integer(i) => LuaType::IntegerConst(i.clone()),
                    LuaMemberKey::None => continue,
                    LuaMemberKey::Expr(_) => continue,
                };

                union_types.push(fake_type);
            }
        } else {
            for member in enum_members {
                if let Some(type_cache) =
                    db.get_type_index().get_type_cache(&member.get_id().into())
                {
                    let member_fake_type = match type_cache.as_type() {
                        LuaType::StringConst(s) => &LuaType::DocStringConst(s.clone().into()),
                        LuaType::IntegerConst(i) => &LuaType::DocIntegerConst(i.clone()),
                        _ => &type_cache.as_type(),
                    };

                    union_types.push(member_fake_type.clone());
                }
            }
        }

        // 当 enum 的值全为整数常量时, 可能会用于位运算, 此时右值推断为整数
        if union_types
            .iter()
            .all(|t| matches!(t, LuaType::DocIntegerConst(_)))
        {
            match compact_type {
                LuaType::Integer => {
                    return Ok(());
                }
                _ => {}
            }
        }

        let fake_union_type = LuaType::Union(LuaUnionType::new(union_types).into());
        return check_general_type_compact(
            db,
            &fake_union_type,
            compact_type,
            check_guard.next_level()?,
        );
    } else {
        let compact_id;
        match compact_type {
            LuaType::Def(id) => compact_id = id.clone(),
            LuaType::Ref(id) => compact_id = id.clone(),
            LuaType::TableConst(range) => {
                let table_member_owner = LuaMemberOwner::Element(range.clone());
                return check_ref_type_compact_table(
                    db,
                    source_id,
                    table_member_owner,
                    check_guard.next_level()?,
                );
            }
            LuaType::Object(object_type) => {
                return check_ref_type_compact_object(
                    db,
                    object_type,
                    source_id,
                    check_guard.next_level()?,
                );
            }
            LuaType::Table => {
                return Ok(());
            }
            LuaType::Union(union_type) => {
                let union_types = union_type.get_types();
                for typ in union_types {
                    match check_general_type_compact(
                        db,
                        &LuaType::Ref(source_id.clone()),
                        &typ,
                        check_guard.next_level()?,
                    ) {
                        Ok(_) => {
                            continue;
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(());
            }
            _ => match get_base_type_id(compact_type) {
                Some(base_type_id) => compact_id = base_type_id.clone(),
                None => return Err(TypeCheckFailReason::TypeNotMatch),
            },
        };

        if *source_id == compact_id {
            return Ok(());
        }

        if is_sub_type_of(db, &compact_id, source_id) {
            return Ok(());
        }

        // This is not the correct logic, but explicit conversion in Lua looks a bit ugly, and too strict,
        // so we have to assume that Lua automatically converts from superclass to subclass.
        if is_sub_type_of(db, source_id, &compact_id) {
            return Ok(());
        }

        // `compact`为枚举时也需要额外处理
        if let LuaType::Ref(compact_id) = compact_type {
            if let Some(compact_decl) = db.get_type_index().get_type_decl(compact_id) {
                if compact_decl.is_enum() {
                    let source = LuaType::Ref(source_id.clone());
                    if let Some(LuaType::Union(enum_fields)) = compact_decl.get_enum_field_type(db)
                    {
                        let is_match = enum_fields.get_types().iter().all(|field| {
                            let next_guard = check_guard.next_level();
                            match next_guard {
                                Ok(guard) => {
                                    check_general_type_compact(db, &source, field, guard).is_ok()
                                }
                                Err(_) => return false,
                            }
                        });
                        if is_match {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    Err(TypeCheckFailReason::TypeNotMatch)
}

fn check_ref_type_compact_table(
    db: &DbIndex,
    source_type_id: &LuaTypeDeclId,
    table_owner: LuaMemberOwner,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let member_index = db.get_member_index();
    let table_member_map = match member_index.get_members(&table_owner) {
        Some(members) => {
            let mut map = HashMap::new();
            for member in members {
                map.insert(member.get_key().clone(), member.get_id().clone());
            }
            map
        }
        None => HashMap::new(),
    };

    let source_type_owner = LuaMemberOwner::Type(source_type_id.clone());
    let source_type_members = match member_index.get_members(&source_type_owner) {
        Some(members) => members,
        // empty member donot need check
        None => return Ok(()),
    };

    for source_member in source_type_members {
        let source_member_type = db
            .get_type_index()
            .get_type_cache(&source_member.get_id().into())
            .unwrap_or(&LuaTypeCache::InferType(LuaType::Any))
            .as_type();
        let key = source_member.get_key();

        if let Some(table_member_id) = table_member_map.get(key) {
            let table_member = member_index
                .get_member(table_member_id)
                .ok_or(TypeCheckFailReason::TypeNotMatch)?;
            let table_member_type = db
                .get_type_index()
                .get_type_cache(&table_member.get_id().into())
                .unwrap_or(&LuaTypeCache::InferType(LuaType::Any))
                .as_type();
            if !check_general_type_compact(
                db,
                &source_member_type,
                &table_member_type,
                check_guard.next_level()?,
            )
            .is_ok()
            {
                return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                    t!(
                        "member %{name} type not match, expect %{expect}, got %{got}",
                        name = key.to_path(),
                        expect = humanize_type(db, &source_member_type, RenderLevel::Simple),
                        got = humanize_type(db, &&table_member_type, RenderLevel::Simple)
                    )
                    .to_string(),
                ));
            }
        } else if source_member_type.is_optional() {
            continue;
        } else {
            return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                t!("missing member %{name}, in table", name = key.to_path()).to_string(),
            ));
        }
    }

    let supers = db.get_type_index().get_super_types(source_type_id);
    if let Some(supers) = supers {
        let table_type = LuaType::TableConst(
            table_owner
                .get_element_range()
                .ok_or(TypeCheckFailReason::TypeNotMatch)?
                .clone(),
        );
        for super_type in supers {
            let result =
                check_general_type_compact(db, &super_type, &table_type, check_guard.next_level()?);
            if !result.is_ok() {
                return result;
            }
        }
    }

    Ok(())
}

fn check_ref_type_compact_object(
    db: &DbIndex,
    object_type: &LuaObjectType,
    source_type_id: &LuaTypeDeclId,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    // ref 可能继承自其他类型, 所以需要使用 infer_members 来获取所有成员
    let source_type_members = match infer_members(db, &LuaType::Ref(source_type_id.clone())) {
        Some(members) => members,
        None => return Ok(()),
    };

    for source_member in source_type_members {
        let source_member_type = source_member.typ;
        let key = source_member.key;
        let field_type = get_object_field_type(object_type, &key);
        if let Some(field_type) = field_type {
            if check_general_type_compact(
                db,
                &source_member_type,
                &field_type,
                check_guard.next_level()?,
            )
            .is_err()
            {
                return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                    t!(
                        "member %{name} type not match, expect %{expect}, got %{got}",
                        name = key.to_path(),
                        expect = humanize_type(db, &source_member_type, RenderLevel::Simple),
                        got = humanize_type(db, &field_type, RenderLevel::Simple)
                    )
                    .to_string(),
                ));
            }
        } else if source_member_type.is_optional() {
            continue;
        } else {
            return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                t!("missing member %{name}, in table", name = key.to_path()).to_string(),
            ));
        }
    }

    Ok(())
}

fn get_object_field_type<'a>(
    object_type: &'a LuaObjectType,
    key: &LuaMemberKey,
) -> Option<&'a LuaType> {
    let field_type = object_type.get_field(&key);
    if let Some(field_type) = field_type {
        return Some(field_type);
    }
    match key {
        LuaMemberKey::Expr(t) => {
            for (index_key, value) in object_type.get_index_access() {
                if index_key == t {
                    return Some(value);
                }
            }
        }
        _ => {}
    }
    None
}
