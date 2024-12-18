use emmylua_parser::{
    LuaAstNode, LuaExpr, LuaIndexExpr, LuaIndexKey, LuaSyntaxId, LuaSyntaxKind, LuaSyntaxNode,
    LuaTableExpr, LuaVarExpr,
};
use rowan::TextRange;

use crate::{
    db_index::{
        DbIndex, LuaExistFieldType, LuaGenericType, LuaIntersectionType, LuaMemberKey,
        LuaMemberOwner, LuaObjectType, LuaOperatorMetaMethod, LuaTupleType, LuaType, LuaTypeDeclId,
        LuaUnionType, TypeAssertion,
    },
    semantic::{
        instantiate::instantiate_type,
        member::{get_buildin_type_map_type_id, without_index_operator, without_members},
        InferGuard,
    },
    InFiled,
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
        &index_expr.get_root(),
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
            let member_owner = LuaMemberOwner::Element(id.clone());
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
        // LuaType::Module(_) => todo!(),
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
        LuaType::ExistField(exist_field) => {
            infer_exit_field_member(db, config, exist_field, member_key)
        }
        LuaType::Global => infer_global_field_member(db, config, member_key),
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
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin() {
            return infer_member_by_member_key(db, config, origin_type, member_key, infer_guard);
        } else {
            return infer_member_by_member_key(
                db,
                config,
                &LuaType::String,
                member_key,
                infer_guard,
            );
        }
    }

    let key: LuaMemberKey = member_key.into();
    let member_owner = LuaMemberOwner::Type(prefix_type_id.clone());
    let member_index = db.get_member_index();
    // find member by key in self
    if let Some(member_map) = member_index.get_member_map(member_owner) {
        if let Some(member_id) = member_map.get(&key) {
            let member = member_index.get_member(&member_id)?;
            return Some(member.get_decl_type().clone());
        }
    }

    // find member by key in super
    if type_decl.is_class() {
        if let Some(super_types) = type_index.get_super_types(&prefix_type_id) {
            for super_type in super_types {
                let member_type =
                    infer_member_by_member_key(db, config, &super_type, member_key, infer_guard);
                if member_type.is_some() {
                    return member_type;
                }
            }
        }
    }

    None
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
    Some(instantiate_type(&member_type, generic_params))
}

fn infer_exit_field_member(
    db: &DbIndex,
    config: &LuaInferConfig,
    exist_field: &LuaExistFieldType,
    member_key: &LuaIndexKey,
) -> InferResult {
    let base_type = exist_field.get_origin();
    let member_type =
        infer_member_by_member_key(db, config, &base_type, member_key, &mut InferGuard::new());

    let access_key: LuaMemberKey = member_key.into();
    let exit_field = exist_field.get_field();
    if access_key == *exit_field {
        match member_type {
            Some(member_type) => Some(TypeAssertion::Exist.tighten_type(member_type)),
            None => Some(LuaType::Any),
        }
    } else {
        member_type
    }
}

fn infer_member_by_operator(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    prefix_type: &LuaType,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
    infer_guard: &mut InferGuard,
) -> InferResult {
    if without_index_operator(prefix_type) {
        return None;
    }

    match &prefix_type {
        LuaType::TableConst(in_filed) => {
            infer_member_by_index_table(db, config, in_filed, root, member_key)
        }
        LuaType::Ref(decl_id) => {
            infer_member_by_index_custom_type(db, config, decl_id, member_key, root, infer_guard)
        }
        LuaType::Def(decl_id) => {
            infer_member_by_index_custom_type(db, config, decl_id, member_key, root, infer_guard)
        }
        // LuaType::Module(arc) => todo!(),
        LuaType::Array(base) => infer_member_by_index_array(db, config, base, member_key),
        LuaType::Nullable(base) => {
            infer_member_by_operator(db, config, base, member_key, root, infer_guard)
        }
        LuaType::Object(object) => infer_member_by_index_object(db, config, object, member_key),
        LuaType::Union(union) => infer_member_by_index_union(db, config, union, member_key, root),
        LuaType::Intersection(intersection) => {
            infer_member_by_index_intersection(db, config, intersection, member_key, root)
        }
        LuaType::Generic(generic) => {
            infer_member_by_index_generic(db, config, generic, member_key, root)
        }
        LuaType::TableGeneric(table_generic) => {
            infer_member_by_index_table_generic(db, config, table_generic, member_key)
        }
        LuaType::ExistField(exist_field) => {
            infer_member_by_index_exist_field(db, config, exist_field, member_key, root)
        }
        _ => None,
    }
}

fn infer_member_by_index_table(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    table_range: &InFiled<TextRange>,
    root: &LuaSyntaxNode,
    member_key: &LuaIndexKey,
) -> InferResult {
    let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::TableArrayExpr.into(), table_range.value);
    let table_array_expr = LuaTableExpr::cast(syntax_id.to_node_from_root(root)?)?;
    match member_key {
        LuaIndexKey::Integer(_) | LuaIndexKey::Expr(_) => {
            let first_field = table_array_expr.get_fields().next()?;
            let first_expr = first_field.get_value_expr()?;
            let ty = infer_expr(db, config, first_expr)?;
            Some(ty)
        }
        _ => None,
    }
}

fn infer_member_by_index_custom_type(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    prefix_type_id: &LuaTypeDeclId,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
    infer_guard: &mut InferGuard,
) -> InferResult {
    infer_guard.check(&prefix_type_id)?;
    let type_index = db.get_type_index();
    let type_decl = type_index.get_type_decl(&prefix_type_id)?;
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin() {
            return infer_member_by_operator(
                db,
                config,
                origin_type,
                member_key,
                root,
                infer_guard,
            );
        }
        return None;
    }

    // find member by key in self
    if let Some(operators_map) = db
        .get_operator_index()
        .get_operators_by_type(prefix_type_id)
    {
        if let Some(index_operator_ids) = operators_map.get(&LuaOperatorMetaMethod::Index) {
            for operator_id in index_operator_ids {
                let operator = db.get_operator_index().get_operator(operator_id)?;
                let operand_type = operator.get_operands().first()?;
                if operand_type.is_string() {
                    if member_key.is_string() || member_key.is_name() {
                        return Some(operator.get_result().clone());
                    } else if member_key.is_expr() {
                        let expr = member_key.get_expr()?;
                        let expr_type = infer_expr(db, config, expr.clone())?;
                        if expr_type.is_string() {
                            return Some(operator.get_result().clone());
                        }
                    }
                } else if operand_type.is_number() {
                    if member_key.is_integer() {
                        return Some(operator.get_result().clone());
                    } else if member_key.is_expr() {
                        let expr = member_key.get_expr()?;
                        let expr_type = infer_expr(db, config, expr.clone())?;
                        if expr_type.is_number() {
                            return Some(operator.get_result().clone());
                        }
                    }
                } else if let Some(expr) = member_key.get_expr() {
                    let expr_type = infer_expr(db, config, expr.clone())?;
                    if expr_type == *operand_type {
                        return Some(operator.get_result().clone());
                    }
                }
            }
        };
    }

    // find member by key in super
    if type_decl.is_class() {
        let super_types = type_index.get_super_types(&prefix_type_id)?;
        for super_type in super_types {
            let member_type =
                infer_member_by_operator(db, config, &super_type, member_key, root, infer_guard);
            if member_type.is_some() {
                return member_type;
            }
        }
    }

    None
}

fn infer_member_by_index_array(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    base: &LuaType,
    member_key: &LuaIndexKey,
) -> InferResult {
    if member_key.is_integer() {
        return Some(base.clone());
    } else if member_key.is_expr() {
        let expr = member_key.get_expr()?;
        let expr_type = infer_expr(db, config, expr.clone())?;
        if expr_type.is_number() {
            return Some(base.clone());
        }
    }

    None
}

fn infer_member_by_index_object(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    object: &LuaObjectType,
    member_key: &LuaIndexKey,
) -> InferResult {
    let access_member_type = object.get_index_access();
    if member_key.is_expr() {
        let expr = member_key.get_expr()?;
        let expr_type = infer_expr(db, config, expr.clone())?;
        for (key, field) in access_member_type {
            if *key == expr_type {
                return Some(field.clone());
            }
        }
    }

    None
}

fn infer_member_by_index_union(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    union: &LuaUnionType,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
) -> InferResult {
    let mut member_types = Vec::new();
    for member in union.get_types() {
        let member_type =
            infer_member_by_operator(db, config, member, member_key, root, &mut InferGuard::new());
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

fn infer_member_by_index_intersection(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    intersection: &LuaIntersectionType,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
) -> InferResult {
    let mut member_type = LuaType::Unknown;
    for member in intersection.get_types() {
        let sub_member_type =
            infer_member_by_operator(db, config, member, member_key, root, &mut InferGuard::new())?;
        if member_type.is_unknown() {
            member_type = sub_member_type;
        } else if member_type != sub_member_type {
            return None;
        }
    }

    Some(member_type)
}

fn infer_member_by_index_generic(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    generic: &LuaGenericType,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
) -> InferResult {
    let base_type = generic.get_base_type();
    let type_decl_id = if let LuaType::Ref(id) = base_type {
        id
    } else {
        return None;
    };
    let generic_params = generic.get_params();
    let type_index = db.get_type_index();
    let type_decl = type_index.get_type_decl(&type_decl_id)?;
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin() {
            return infer_member_by_operator(
                db,
                config,
                &instantiate_type(origin_type, generic_params),
                member_key,
                root,
                &mut InferGuard::new(),
            );
        }
        return None;
    }

    let operator_index = db.get_operator_index();
    let operator_maps = operator_index.get_operators_by_type(&type_decl_id)?;
    let index_operator_ids = operator_maps.get(&LuaOperatorMetaMethod::Index)?;
    for index_operator_id in index_operator_ids {
        let index_operator = operator_index.get_operator(index_operator_id)?;
        let operand_type = index_operator.get_operands().first()?;
        let instianted_operand_type = instantiate_type(&operand_type, generic_params);
        if instianted_operand_type.is_string() {
            if member_key.is_string() || member_key.is_name() {
                return Some(instantiate_type(
                    index_operator.get_result(),
                    generic_params,
                ));
            } else if member_key.is_expr() {
                let expr = member_key.get_expr()?;
                let expr_type = infer_expr(db, config, expr.clone())?;
                if expr_type.is_string() {
                    return Some(instantiate_type(
                        index_operator.get_result(),
                        generic_params,
                    ));
                }
            }
        } else if instianted_operand_type.is_number() {
            if member_key.is_integer() {
                return Some(instantiate_type(
                    index_operator.get_result(),
                    generic_params,
                ));
            } else if member_key.is_expr() {
                let expr = member_key.get_expr()?;
                let expr_type = infer_expr(db, config, expr.clone())?;
                if expr_type.is_number() {
                    return Some(instantiate_type(
                        index_operator.get_result(),
                        generic_params,
                    ));
                }
            }
        } else if let Some(expr) = member_key.get_expr() {
            let expr_type = infer_expr(db, config, expr.clone())?;
            if expr_type == *operand_type {
                return Some(instantiate_type(
                    index_operator.get_result(),
                    generic_params,
                ));
            }
        }
    }

    // for supers
    let supers = type_index.get_super_types(&type_decl_id)?;
    for super_type in supers {
        let member_type = infer_member_by_operator(
            db,
            config,
            &instantiate_type(&super_type, generic_params),
            member_key,
            root,
            &mut InferGuard::new(),
        );
        if member_type.is_some() {
            return member_type;
        }
    }

    None
}

fn infer_member_by_index_table_generic(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    table_params: &Vec<LuaType>,
    member_key: &LuaIndexKey,
) -> InferResult {
    if table_params.len() != 2 {
        return None;
    }

    let key_type = &table_params[0];
    let value_type = &table_params[1];
    if key_type.is_string() {
        if member_key.is_string() || member_key.is_name() {
            return Some(value_type.clone());
        } else if member_key.is_expr() {
            let expr = member_key.get_expr()?;
            let expr_type = infer_expr(db, config, expr.clone())?;
            if expr_type.is_string() {
                return Some(value_type.clone());
            }
        }
    } else if key_type.is_number() {
        if member_key.is_integer() {
            return Some(value_type.clone());
        } else if member_key.is_expr() {
            let expr = member_key.get_expr()?;
            let expr_type = infer_expr(db, config, expr.clone())?;
            if expr_type.is_number() {
                return Some(value_type.clone());
            }
        }
    } else {
        let expr = member_key.get_expr()?;
        let expr_type = infer_expr(db, config, expr.clone())?;
        if expr_type == *key_type {
            return Some(value_type.clone());
        }
    }

    None
}

fn infer_member_by_index_exist_field(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    exist_field: &LuaExistFieldType,
    member_key: &LuaIndexKey,
    root: &LuaSyntaxNode,
) -> InferResult {
    let base_type = exist_field.get_origin();
    let member_type = infer_member_by_operator(
        db,
        config,
        &base_type,
        member_key,
        root,
        &mut InferGuard::new(),
    );

    let access_key: LuaMemberKey = member_key.into();
    let exit_field = exist_field.get_field();
    if access_key == *exit_field {
        match member_type {
            Some(member_type) => Some(TypeAssertion::Exist.tighten_type(member_type)),
            None => Some(LuaType::Any),
        }
    } else {
        member_type
    }
}

fn infer_global_field_member(
    db: &DbIndex,
    _: &LuaInferConfig,
    member_key: &LuaIndexKey,
) -> InferResult {
    let name = member_key.get_name()?.get_name_text();
    let global_member = db
        .get_decl_index()
        .get_global_decl_id(&LuaMemberKey::Name(name.to_string().into()))?;

    let decl = db.get_decl_index().get_decl(&global_member)?;
    Some(decl.get_type()?.clone())
}
