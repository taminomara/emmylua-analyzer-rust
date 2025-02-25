use std::{collections::HashMap, ops::Deref};

use crate::{
    db_index::{
        LuaFunctionType, LuaGenericType, LuaIntersectionType, LuaMemberPathExistType,
        LuaMultiReturn, LuaObjectType, LuaTupleType, LuaType, LuaUnionType,
    },
    semantic::{member::infer_members, type_check},
    DbIndex, GenericTpl, LuaAliasCallKind, LuaAliasCallType, LuaMemberKey, LuaPropertyOwnerId,
    LuaSignatureId, TypeOps,
};

use super::type_substitutor::{SubstitutorValue, TypeSubstitutor};

pub fn instantiate_type(db: &DbIndex, ty: &LuaType, substitutor: &TypeSubstitutor) -> LuaType {
    match ty {
        LuaType::Array(base) => instantiate_array(db, base, substitutor),
        LuaType::Nullable(base) => instantiate_nullable(db, base, substitutor),
        LuaType::Tuple(tuple) => instantiate_tuple(db, tuple, substitutor),
        LuaType::DocFunction(doc_func) => instantiate_doc_function(db, doc_func, substitutor),
        LuaType::Object(object) => instantiate_object(db, object, substitutor),
        LuaType::Union(union) => instantiate_union(db, union, substitutor),
        LuaType::Intersection(intersection) => {
            instantiate_intersection(db, intersection, substitutor)
        }
        LuaType::Generic(generic) => instantiate_generic(db, generic, substitutor),
        LuaType::TableGeneric(table_params) => {
            instantiate_table_generic(db, table_params, substitutor)
        }
        LuaType::TplRef(tpl) => instantiate_tpl_ref(db, tpl, substitutor),
        LuaType::MuliReturn(multi) => instantiate_multi_return(db, multi, substitutor),
        LuaType::MemberPathExist(exit_field) => {
            instantiate_exist_field(db, exit_field, substitutor)
        }
        LuaType::Signature(sig_id) => instantiate_signature(db, sig_id, substitutor),
        LuaType::Call(alias_call) => instantiate_alias_call(db, alias_call, substitutor),
        LuaType::Variadic(inner) => instantiate_variadic_type(db, inner, substitutor),
        _ => ty.clone(),
    }
}

fn instantiate_array(db: &DbIndex, base: &LuaType, substitutor: &TypeSubstitutor) -> LuaType {
    let base = instantiate_type(db, base, substitutor);
    LuaType::Array(base.into())
}

fn instantiate_nullable(db: &DbIndex, inner: &LuaType, substitutor: &TypeSubstitutor) -> LuaType {
    let base = instantiate_type(db, inner, substitutor);
    LuaType::Nullable(base.into())
}

fn instantiate_tuple(db: &DbIndex, tuple: &LuaTupleType, substitutor: &TypeSubstitutor) -> LuaType {
    let tuple_types = tuple.get_types();
    let mut new_types = Vec::new();
    for t in tuple_types {
        if let LuaType::Variadic(inner) = t {
            if let LuaType::TplRef(tpl) = inner.deref() {
                if let Some(value) = substitutor.get(tpl.get_tpl_id()) {
                    match value {
                        SubstitutorValue::MultiTypes(types) => {
                            for typ in types {
                                new_types.push(typ.clone());
                            }
                        }
                        SubstitutorValue::Params(params) => {
                            for (_, ty) in params {
                                new_types.push(ty.clone().unwrap_or(LuaType::Unknown));
                            }
                        }
                        SubstitutorValue::Type(ty) => new_types.push(ty.clone()),
                    }
                }
            }

            break;
        }

        let t = instantiate_type(db, t, substitutor);
        new_types.push(t);
    }
    LuaType::Tuple(LuaTupleType::new(new_types).into())
}

pub fn instantiate_doc_function(
    db: &DbIndex,
    doc_func: &LuaFunctionType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let tpl_func_params = doc_func.get_params();
    let tpl_ret = doc_func.get_ret();
    let is_async = doc_func.is_async();
    let colon_define = doc_func.is_colon_define();

    let mut new_params = Vec::new();
    for i in 0..tpl_func_params.len() {
        let origin_param = &tpl_func_params[i];
        if origin_param.1.is_none() {
            new_params.push((origin_param.0.clone(), None));
            continue;
        }

        let origin_param_type = origin_param.1.clone().unwrap();
        match &origin_param_type {
            LuaType::Variadic(inner) => {
                if let LuaType::TplRef(tpl) = inner.deref() {
                    if let Some(value) = substitutor.get(tpl.get_tpl_id()) {
                        match value {
                            SubstitutorValue::Params(params) => {
                                for param in params {
                                    new_params.push(param.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {
                let new_type = instantiate_type(db, &origin_param_type, &substitutor);
                new_params.push((origin_param.0.clone(), Some(new_type)));
            }
        }
    }

    let mut new_returns = Vec::new();
    for i in 0..tpl_ret.len() {
        let ret_type = &tpl_ret[i];
        match &ret_type {
            LuaType::Variadic(inner) => {
                if let LuaType::TplRef(tpl) = inner.deref() {
                    if let Some(value) = substitutor.get(tpl.get_tpl_id()) {
                        match value {
                            SubstitutorValue::MultiTypes(types) => {
                                for typ in types {
                                    new_returns.push(typ.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {
                let new_type = instantiate_type(db, &ret_type, &substitutor);
                new_returns.push(new_type);
            }
        }
    }
    LuaType::DocFunction(
        LuaFunctionType::new(is_async, colon_define, new_params, new_returns).into(),
    )
}

fn instantiate_object(
    db: &DbIndex,
    object: &LuaObjectType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let fields = object.get_fields();
    let index_access = object.get_index_access();

    let mut new_fields = HashMap::new();
    for (key, field) in fields {
        let new_field = instantiate_type(db, field, substitutor);
        new_fields.insert(key.clone(), new_field);
    }

    let mut new_index_access = Vec::new();
    for (key, value) in index_access {
        let key = instantiate_type(db, &key, substitutor);
        let value = instantiate_type(db, &value, substitutor);
        new_index_access.push((key, value));
    }

    LuaType::Object(LuaObjectType::new_with_fields(new_fields, new_index_access).into())
}

fn instantiate_union(db: &DbIndex, union: &LuaUnionType, substitutor: &TypeSubstitutor) -> LuaType {
    let types = union.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(db, t, substitutor);
        new_types.push(t);
    }

    LuaType::Union(LuaUnionType::new(new_types).into())
}

fn instantiate_intersection(
    db: &DbIndex,
    intersection: &LuaIntersectionType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let types = intersection.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(db, t, substitutor);
        new_types.push(t);
    }

    LuaType::Intersection(LuaIntersectionType::new(new_types).into())
}

fn instantiate_generic(
    db: &DbIndex,
    generic: &LuaGenericType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let generic_params = generic.get_params();
    let mut new_params = Vec::new();
    for param in generic_params {
        let new_param = instantiate_type(db, param, substitutor);
        new_params.push(new_param);
    }

    let base = generic.get_base_type();
    let type_decl_id = if let LuaType::Ref(id) = base {
        id
    } else {
        return LuaType::Unknown;
    };

    if !substitutor.check_recursion(&type_decl_id) {
        if let Some(type_decl) = db.get_type_index().get_type_decl(&type_decl_id) {
            if type_decl.is_alias_replace() {
                let new_substitutor =
                    TypeSubstitutor::from_alias(new_params.clone(), type_decl_id.clone());
                if let Some(origin) = type_decl.get_alias_origin(db, Some(&new_substitutor)) {
                    return origin;
                }
            }
        }
    }

    LuaType::Generic(LuaGenericType::new(type_decl_id, new_params).into())
}

fn instantiate_table_generic(
    db: &DbIndex,
    table_params: &Vec<LuaType>,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let mut new_params = Vec::new();
    for param in table_params {
        let new_param = instantiate_type(db, param, substitutor);
        new_params.push(new_param);
    }

    LuaType::TableGeneric(new_params.into())
}

fn instantiate_tpl_ref(_: &DbIndex, tpl: &GenericTpl, substitutor: &TypeSubstitutor) -> LuaType {
    if let Some(value) = substitutor.get(tpl.get_tpl_id()) {
        match value {
            SubstitutorValue::Type(ty) => return ty.clone(),
            SubstitutorValue::MultiTypes(types) => {
                return types.first().unwrap_or(&LuaType::Unknown).clone();
            }
            SubstitutorValue::Params(params) => {
                return params
                    .first()
                    .unwrap_or(&(String::new(), None))
                    .1
                    .clone()
                    .unwrap_or(LuaType::Unknown);
            }
        }
    }

    LuaType::TplRef(tpl.clone().into())
}

fn instantiate_multi_return(
    db: &DbIndex,
    multi_returns: &LuaMultiReturn,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    match multi_returns {
        LuaMultiReturn::Base(base) => {
            let new_base = instantiate_type(db, base, substitutor);
            LuaType::MuliReturn(LuaMultiReturn::Base(new_base).into())
        }
        LuaMultiReturn::Multi(types) => {
            let mut new_types = Vec::new();
            for t in types {
                let t = instantiate_type(db, t, substitutor);
                new_types.push(t);
            }
            LuaType::MuliReturn(LuaMultiReturn::Multi(new_types).into())
        }
    }
}

fn instantiate_exist_field(
    db: &DbIndex,
    exit_field: &LuaMemberPathExistType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let base = instantiate_type(db, &exit_field.get_origin(), substitutor);
    let path = exit_field.get_path();
    let idx = exit_field.get_current_path_idx();
    LuaType::MemberPathExist(LuaMemberPathExistType::new(path, base, idx).into())
}

fn instantiate_signature(
    db: &DbIndex,
    signature_id: &LuaSignatureId,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    if let Some(signature) = db.get_signature_index().get(&signature_id) {
        let origin_type = {
            let rets = signature
                .return_docs
                .iter()
                .map(|ret| ret.type_ref.clone())
                .collect();
            let is_async = if let Some(property) = db
                .get_property_index()
                .get_property(LuaPropertyOwnerId::Signature(signature_id.clone()))
            {
                property.is_async
            } else {
                false
            };
            let fake_doc_function = LuaFunctionType::new(
                is_async,
                signature.is_colon_define,
                signature.get_type_params(),
                rets,
            );
            instantiate_doc_function(db, &fake_doc_function, substitutor)
        };
        if signature.overloads.is_empty() {
            return origin_type;
        } else {
            let mut result = Vec::new();
            for overload in signature.overloads.iter() {
                result.push(instantiate_doc_function(
                    db,
                    &(*overload).clone(),
                    substitutor,
                ));
            }
            result.push(origin_type); // 我们需要将原始类型放到最后
            return LuaType::Union(LuaUnionType::new(result).into());
        }
    }

    return LuaType::Signature(signature_id.clone());
}

fn instantiate_alias_call(
    db: &DbIndex,
    alias_call: &LuaAliasCallType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    let left = alias_call.get_source();
    let right = alias_call.get_operand().unwrap_or(&LuaType::Unknown);
    let left_inst = instantiate_type(db, left, substitutor);
    let right_inst = instantiate_type(db, right, substitutor);
    match alias_call.get_call_kind() {
        LuaAliasCallKind::Sub => return TypeOps::Remove.apply(&left_inst, &right_inst),
        LuaAliasCallKind::Add => return TypeOps::Union.apply(&left_inst, &right_inst),
        LuaAliasCallKind::KeyOf => {
            let members = infer_members(db, &left_inst).unwrap_or(Vec::new());
            let member_key_types = members
                .iter()
                .filter_map(|m| match &m.key {
                    LuaMemberKey::Integer(i) => Some(LuaType::DocIntegerConst(i.clone())),
                    LuaMemberKey::Name(s) => Some(LuaType::DocStringConst(s.clone().into())),
                    _ => None,
                })
                .collect::<Vec<_>>();

            return LuaType::Union(LuaUnionType::new(member_key_types).into());
        }
        LuaAliasCallKind::Extends => {
            let compact = type_check::check_type_compact(db, &right_inst, &left_inst).is_ok();
            return LuaType::BooleanConst(compact);
        }
        _ => {}
    }

    LuaType::Unknown
}

fn instantiate_variadic_type(
    db: &DbIndex,
    inner: &LuaType,
    substitutor: &TypeSubstitutor,
) -> LuaType {
    if let LuaType::TplRef(tpl) = inner {
        if let Some(value) = substitutor.get(tpl.get_tpl_id()) {
            match value {
                SubstitutorValue::Type(ty) => return ty.clone(),
                SubstitutorValue::MultiTypes(types) => {
                    return LuaType::MuliReturn(LuaMultiReturn::Multi(types.clone()).into())
                }
                SubstitutorValue::Params(params) => {
                    let types = params
                        .iter()
                        .filter_map(|(_, ty)| ty.clone())
                        .collect::<Vec<_>>();
                    return LuaType::MuliReturn(LuaMultiReturn::Multi(types).into());
                }
            }
        }
    }

    instantiate_type(db, inner, substitutor)
}
