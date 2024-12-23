use std::collections::HashMap;

use crate::{
    db_index::{
        LuaExistFieldType, LuaExtendedType, LuaFunctionType, LuaGenericType, LuaIntersectionType,
        LuaMultiReturn, LuaObjectType, LuaTupleType, LuaType, LuaUnionType,
    },
    DbIndex, GenericTpl, LuaPropertyOwnerId, LuaSignatureId,
};

pub fn instantiate_type(db: &DbIndex, ty: &LuaType, params: &Vec<LuaType>) -> LuaType {
    match ty {
        LuaType::Array(base) => instantiate_array(db, base, params),
        LuaType::KeyOf(base) => instantiate_key_of(db, base, params),
        LuaType::Nullable(base) => instantiate_nullable(db, base, params),
        LuaType::Tuple(tuple) => instantiate_tuple(db, tuple, params),
        LuaType::DocFunction(doc_func) => instantiate_doc_function(db, doc_func, params),
        LuaType::Object(object) => instantiate_object(db, object, params),
        LuaType::Union(union) => instantiate_union(db, union, params),
        LuaType::Intersection(intersection) => instantiate_intersection(db, intersection, params),
        LuaType::Extends(extends) => instantiate_extends(db, extends, params),
        LuaType::Generic(generic) => instantiate_generic(db, generic, params),
        LuaType::TableGeneric(table_params) => instantiate_table_generic(db, table_params, params),
        LuaType::TplRef(tpl) => instantiate_tpl_ref(db, tpl, params),
        LuaType::FuncTplRef(tpl) => instantiate_tpl_ref(db, tpl, params),
        LuaType::MuliReturn(multi) => instantiate_multi_return(db, multi, params),
        LuaType::ExistField(exit_field) => instantiate_exist_field(db, exit_field, params),
        LuaType::Signature(sig_id) => instantiate_signature(db, sig_id, params),
        _ => ty.clone(),
    }
}

fn instantiate_array(db: &DbIndex, base: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(db, base, params);
    LuaType::Array(base.into())
}

fn instantiate_key_of(db: &DbIndex, base: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(db, base, params);
    LuaType::KeyOf(base.into())
}

fn instantiate_nullable(db: &DbIndex, inner: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(db, inner, params);
    LuaType::Nullable(base.into())
}

fn instantiate_tuple(db: &DbIndex, tuple: &LuaTupleType, params: &Vec<LuaType>) -> LuaType {
    let tuple_types = tuple.get_types();
    let mut new_types = Vec::new();
    for t in tuple_types {
        let t = instantiate_type(db, t, params);
        new_types.push(t);
    }
    LuaType::Tuple(LuaTupleType::new(new_types).into())
}

fn instantiate_doc_function(
    db: &DbIndex,
    doc_func: &LuaFunctionType,
    params: &Vec<LuaType>,
) -> LuaType {
    let func_params = doc_func.get_params();
    let ret = doc_func.get_ret();
    let is_async = doc_func.is_async();
    let colon_define = doc_func.is_colon_define();
    let mut new_params = Vec::new();
    for (name, param_type) in func_params {
        if param_type.is_some() {
            let new_param = instantiate_type(db, param_type.as_ref().unwrap(), params);
            new_params.push((name.clone(), Some(new_param)));
            continue;
        } else {
            new_params.push((name.clone(), None));
        }
    }

    let mut new_ret = Vec::new();
    for ret_type in ret {
        let new_ret_type = instantiate_type(db, ret_type, params);
        new_ret.push(new_ret_type);
    }
    LuaType::DocFunction(LuaFunctionType::new(is_async, colon_define, new_params, new_ret).into())
}

fn instantiate_object(db: &DbIndex, object: &LuaObjectType, params: &Vec<LuaType>) -> LuaType {
    let fields = object.get_fields();
    let index_acess = object.get_index_access();

    let mut new_fields = HashMap::new();
    for (key, field) in fields {
        let new_field = instantiate_type(db, field, params);
        new_fields.insert(key.clone(), new_field);
    }

    let mut new_index_access = Vec::new();
    for (key, value) in index_acess {
        let key = instantiate_type(db, &key, params);
        let value = instantiate_type(db, &value, params);
        new_index_access.push((key, value));
    }

    LuaType::Object(LuaObjectType::new_with_fields(new_fields, new_index_access).into())
}

fn instantiate_union(db: &DbIndex, union: &LuaUnionType, params: &Vec<LuaType>) -> LuaType {
    let types = union.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(db, t, params);
        new_types.push(t);
    }

    LuaType::Union(LuaUnionType::new(new_types).into())
}

fn instantiate_intersection(
    db: &DbIndex,
    intersection: &LuaIntersectionType,
    params: &Vec<LuaType>,
) -> LuaType {
    let types = intersection.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(db, t, params);
        new_types.push(t);
    }

    LuaType::Intersection(LuaIntersectionType::new(new_types).into())
}

fn instantiate_extends(db: &DbIndex, extends: &LuaExtendedType, params: &Vec<LuaType>) -> LuaType {
    let base = extends.get_base();
    let new_base = instantiate_type(db, base, params);
    let ext = extends.get_ext();
    let new_ext = instantiate_type(db, ext, params);
    LuaType::Extends(LuaExtendedType::new(new_base, new_ext).into())
}

fn instantiate_generic(db: &DbIndex, generic: &LuaGenericType, params: &Vec<LuaType>) -> LuaType {
    let generic_params = generic.get_params();
    let mut new_params = Vec::new();
    for param in generic_params {
        let new_param = instantiate_type(db, param, params);
        new_params.push(new_param);
    }

    let base = generic.get_base_type();
    let decl_id = if let LuaType::Ref(id) = base {
        id
    } else {
        return LuaType::Unknown;
    };

    LuaType::Generic(LuaGenericType::new(decl_id, new_params).into())
}

fn instantiate_table_generic(
    db: &DbIndex,
    table_params: &Vec<LuaType>,
    params: &Vec<LuaType>,
) -> LuaType {
    let mut new_params = Vec::new();
    for param in table_params {
        let new_param = instantiate_type(db, param, params);
        new_params.push(new_param);
    }

    LuaType::TableGeneric(new_params.into())
}

fn instantiate_tpl_ref(_: &DbIndex, tpl: &GenericTpl, params: &Vec<LuaType>) -> LuaType {
    if let Some(ty) = params.get(tpl.get_tpl_id()) {
        ty.clone()
    } else {
        LuaType::Unknown
    }
}

fn instantiate_multi_return(
    db: &DbIndex,
    multi_returns: &LuaMultiReturn,
    params: &Vec<LuaType>,
) -> LuaType {
    match multi_returns {
        LuaMultiReturn::Base(base) => {
            let new_base = instantiate_type(db, base, params);
            LuaType::MuliReturn(LuaMultiReturn::Base(new_base).into())
        }
        LuaMultiReturn::Multi(types) => {
            let mut new_types = Vec::new();
            for t in types {
                let t = instantiate_type(db, t, params);
                new_types.push(t);
            }
            LuaType::MuliReturn(LuaMultiReturn::Multi(new_types).into())
        }
    }
}

fn instantiate_exist_field(
    db: &DbIndex,
    exit_field: &LuaExistFieldType,
    params: &Vec<LuaType>,
) -> LuaType {
    let base = instantiate_type(db, &exit_field.get_origin(), params);
    let field = exit_field.get_field();
    LuaType::ExistField(LuaExistFieldType::new(field.clone(), base).into())
}

fn instantiate_signature(
    db: &DbIndex,
    signature_id: &LuaSignatureId,
    params: &Vec<LuaType>,
) -> LuaType {
    if let Some(signature) = db.get_signature_index().get(&signature_id) {
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
        let instantiate_func = instantiate_doc_function(db, &fake_doc_function, params);
        return instantiate_func;
    }

    return LuaType::Signature(signature_id.clone());
}
