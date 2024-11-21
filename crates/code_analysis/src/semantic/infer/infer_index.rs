use emmylua_parser::{LuaExpr, LuaIndexExpr, LuaIndexKey, LuaVarExpr};
use rowan::TextRange;

use crate::{
    db_index::{
        DbIndex, LuaDeclTypeKind, LuaGenericType, LuaIntersectionType, LuaMemberKey,
        LuaMemberOwner, LuaObjectType, LuaTupleType, LuaType, LuaTypeDeclId, LuaUnionType,
    },
    semantic::{
        member::{get_buildin_type_map_type_id, without_index_operator, without_members},
        InferGuard,
    },
};

use super::{infer_expr, InferResult, LuaInferConfig};

pub fn infer_index_expr(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    index_expr: LuaIndexExpr,
) -> InferResult {
    let prefix_expr = index_expr.get_prefix_expr()?;
    let prefix_type = match prefix_expr {
        LuaVarExpr::IndexExpr(prefix_index) => {
            infer_expr(db, config, LuaExpr::IndexExpr(prefix_index))?
        }
        LuaVarExpr::NameExpr(prefix_name) => {
            infer_expr(db, config, LuaExpr::NameExpr(prefix_name))?
        }
    };

    let member_key = index_expr.get_index_key()?;
    if let Some(member_type) = infer_member_by_member_key(
        db,
        config,
        &prefix_type,
        &member_key,
        &mut InferGuard::new(),
    ) {
        return Some(member_type);
    }

    if let Some(member_type) = infer_member_by_operator(
        db,
        config,
        &prefix_type,
        &member_key,
        &mut InferGuard::new(),
    ) {
        return Some(member_type);
    }

    None
}

fn infer_member_by_member_key(
    db: &DbIndex,
    config: &LuaInferConfig,
    prefix_type: &LuaType,
    member_key: &LuaIndexKey,
    infer_guard: &mut InferGuard,
) -> InferResult {
    if without_members(prefix_type) {
        return None;
    }

    match &prefix_type {
        LuaType::TableConst(id) => {
            let member_owner = LuaMemberOwner::Table(id.clone());
            infer_table_member(db, member_owner, member_key)
        }
        LuaType::String | LuaType::Io | LuaType::StringConst(_) => {
            let decl_id = get_buildin_type_map_type_id(&prefix_type)?;
            infer_custom_type_member(db, config, decl_id, member_key, infer_guard)
        }
        LuaType::Ref(decl_id) => {
            infer_custom_type_member(db, config, decl_id.clone(), member_key, infer_guard)
        }
        LuaType::Def(decl_id) => {
            infer_custom_type_member(db, config, decl_id.clone(), member_key, infer_guard)
        }
        LuaType::SelfInfer => todo!(),
        LuaType::Module(_) => todo!(),
        LuaType::KeyOf(_) => {
            let decl_id = LuaTypeDeclId::new("string");
            infer_custom_type_member(db, config, decl_id, member_key, infer_guard)
        }
        LuaType::Nullable(inner_type) => {
            infer_member_by_member_key(db, config, &inner_type, member_key, infer_guard)
        }
        LuaType::Tuple(tuple_type) => infer_tuple_member(tuple_type, member_key),
        LuaType::Object(object_type) => infer_object_member(object_type, member_key),
        LuaType::Union(union_type) => infer_union_member(db, config, union_type, member_key),
        LuaType::Intersection(intersection_type) => {
            infer_intersection_member(db, config, intersection_type, member_key)
        }
        LuaType::Generic(generic_type) => {
            infer_generic_member(db, config, generic_type, member_key)
        }
        LuaType::ExistField(exist_field) => todo!(),
        _ => None,
    }
}

fn infer_table_member(
    db: &DbIndex,
    table_owner: LuaMemberOwner,
    member_key: &LuaIndexKey,
) -> InferResult {
    let member_index = db.get_member_index();
    let member_map = member_index.get_member_map(table_owner)?;
    let key: LuaMemberKey = member_key.into();
    let member_id = member_map.get(&key)?;
    let member = member_index.get_member(&member_id)?;
    Some(member.get_decl_type().clone())
}

fn infer_custom_type_member(
    db: &DbIndex,
    config: &LuaInferConfig,
    prefix_type_id: LuaTypeDeclId,
    member_key: &LuaIndexKey,
    infer_guard: &mut InferGuard,
) -> InferResult {
    infer_guard.check(&prefix_type_id)?;
    let type_index = db.get_type_index();
    let type_decl = type_index.get_type_decl(&prefix_type_id)?;
    if type_decl.get_kind() == LuaDeclTypeKind::Alias {
        let alias_types = type_index.get_super_types(&prefix_type_id)?;
        let origin_type = alias_types.first()?;
        return infer_member_by_member_key(db, config, origin_type, member_key, infer_guard);
    }

    let member_owner = LuaMemberOwner::Type(prefix_type_id);
    let member_index = db.get_member_index();
    let member_map = member_index.get_member_map(member_owner)?;
    let key: LuaMemberKey = member_key.into();
    let member_id = member_map.get(&key)?;
    let member = member_index.get_member(&member_id)?;
    // TODO for enum
    Some(member.get_decl_type().clone())
}

fn infer_tuple_member(tuple_type: &LuaTupleType, member_key: &LuaIndexKey) -> InferResult {
    let key = member_key.into();
    if let LuaMemberKey::Integer(i) = key {
        let index = i as usize;
        return Some(tuple_type.get_type(index)?.clone());
    }

    None
}

fn infer_object_member(object_type: &LuaObjectType, member_key: &LuaIndexKey) -> InferResult {
    let member_type = object_type.get_field(&member_key.into())?;
    Some(member_type.clone())
}

fn infer_union_member(
    db: &DbIndex,
    config: &LuaInferConfig,
    union_type: &LuaUnionType,
    member_key: &LuaIndexKey,
) -> InferResult {
    let mut member_types = Vec::new();
    for member in union_type.get_types() {
        let member_type =
            infer_member_by_member_key(db, config, member, member_key, &mut InferGuard::new());
        if let Some(member_type) = member_type {
            member_types.push(member_type);
        }
    }

    if member_types.is_empty() {
        return None;
    }

    if member_types.len() == 1 {
        return Some(member_types[0].clone());
    }

    Some(LuaType::Union(LuaUnionType::new(member_types).into()))
}

fn infer_intersection_member(
    db: &DbIndex,
    config: &LuaInferConfig,
    intersection_type: &LuaIntersectionType,
    member_key: &LuaIndexKey,
) -> InferResult {
    let mut member_type = LuaType::Unknown;
    for member in intersection_type.get_types() {
        let sub_member_type =
            infer_member_by_member_key(db, config, member, member_key, &mut InferGuard::new())?;
        if member_type.is_unknown() {
            member_type = sub_member_type;
        } else if member_type != sub_member_type {
            return None;
        }
    }

    Some(member_type)
}

fn infer_generic_member(
    db: &DbIndex,
    config: &LuaInferConfig,
    generic_type: &LuaGenericType,
    member_key: &LuaIndexKey,
) -> InferResult {
    let base_type = generic_type.get_base_type();
    let member_type =
        infer_member_by_member_key(db, config, &base_type, member_key, &mut InferGuard::new())?;

    let generic_params = generic_type.get_params();
    


    None
}

fn infer_member_by_operator(
    db: &DbIndex,
    config: &LuaInferConfig,
    prefix_type: &LuaType,
    member_key: &LuaIndexKey,
    infer_guard: &mut InferGuard,
) -> InferResult {
    if without_index_operator(prefix_type) {
        return None;
    }
    None
}
