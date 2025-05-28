use std::{collections::HashMap, sync::Arc};

use crate::{
    humanize_type, semantic::member::find_members, DbIndex, LuaMemberKey, LuaMemberOwner,
    LuaObjectType, LuaTupleType, LuaType, LuaTypeCache, LuaTypeDeclId, LuaUnionType, RenderLevel,
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
            LuaType::Def(compact_id) | LuaType::Ref(compact_id) => {
                if source_id == compact_id {
                    return Ok(());
                }
            }
            _ => {}
        };
        // 移除掉枚举类型本身
        let mut compact_type = compact_type.clone();
        match compact_type {
            LuaType::Union(union_types) => {
                let mut new_union_types = Vec::new();
                let union_types = union_types.get_types();
                for typ in union_types {
                    if let LuaType::Def(compact_id) | LuaType::Ref(compact_id) = typ {
                        if compact_id != source_id {
                            new_union_types.push(typ.clone());
                        }
                        continue;
                    }
                    new_union_types.push(typ.clone());
                }
                compact_type = LuaType::Union(Arc::new(LuaUnionType::new(new_union_types)));
            }
            _ => {}
        }

        let Some(enum_fields) = type_decl.get_enum_field_type(db) else {
            return Err(TypeCheckFailReason::TypeNotMatch);
        };

        if let LuaType::Union(union_types) = &enum_fields {
            // 当 enum 的值全为整数常量时, 可能会用于位运算, 此时右值推断为整数
            if union_types
                .get_types()
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
        }

        return check_general_type_compact(
            db,
            &enum_fields,
            &compact_type,
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
            LuaType::Tuple(tuple_type) => {
                return check_ref_type_compact_tuple(
                    db,
                    tuple_type,
                    source_id,
                    check_guard.next_level()?,
                );
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
                        for field in enum_fields.get_types() {
                            check_general_type_compact(
                                db,
                                &source,
                                field,
                                check_guard.next_level()?,
                            )?;
                        }

                        return Ok(());
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
            match check_general_type_compact(
                db,
                &source_member_type,
                &table_member_type,
                check_guard.next_level()?,
            ) {
                Ok(_) => {}
                Err(TypeCheckFailReason::TypeNotMatch) => {
                    return Err(TypeCheckFailReason::TypeNotMatchWithReason(
                        t!(
                            "member %{name} type not match, expect %{expect}, got %{got}",
                            name = key.to_path(),
                            expect = humanize_type(db, &source_member_type, RenderLevel::Simple),
                            got = humanize_type(db, &&table_member_type, RenderLevel::Simple)
                        )
                        .to_string(),
                    ))
                }
                Err(e) => return Err(e),
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
            check_general_type_compact(db, &super_type, &table_type, check_guard.next_level()?)?;
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
    let source_type_members = match find_members(db, &LuaType::Ref(source_type_id.clone())) {
        Some(members) => members,
        None => return Ok(()),
    };

    for source_member in source_type_members {
        let source_member_type = source_member.typ;
        let key = source_member.key;
        let field_type = get_object_field_type(object_type, &key);
        if let Some(field_type) = field_type {
            match check_general_type_compact(
                db,
                &source_member_type,
                &field_type,
                check_guard.next_level()?,
            ) {
                Ok(_) => {}
                Err(TypeCheckFailReason::TypeNotMatch) => {
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
                Err(e) => return Err(e),
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

fn check_ref_type_compact_tuple(
    db: &DbIndex,
    tuple_type: &LuaTupleType,
    source_type_id: &LuaTypeDeclId,
    check_guard: TypeCheckGuard,
) -> TypeCheckResult {
    let source_type_members = match find_members(db, &LuaType::Ref(source_type_id.clone())) {
        Some(members) => members,
        None => return Ok(()),
    };

    let tuple_types = tuple_type.get_types();
    for member in source_type_members {
        match member.key {
            LuaMemberKey::Integer(index) => {
                // 在 lua 中数组索引从 1 开始, 当数组被解析为元组时也必然从 1 开始
                if index <= 0 {
                    return Err(TypeCheckFailReason::TypeNotMatch);
                }
                if let Some(tuple_type) = tuple_types.get(index as usize - 1) {
                    check_general_type_compact(
                        db,
                        &member.typ,
                        tuple_type,
                        check_guard.next_level()?,
                    )?;
                } else {
                    return Err(TypeCheckFailReason::TypeNotMatch);
                }
            }
            _ => return Err(TypeCheckFailReason::TypeNotMatch),
        }
    }
    return Ok(());
}
