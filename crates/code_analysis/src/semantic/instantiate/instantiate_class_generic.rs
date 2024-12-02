use std::collections::HashMap;

use crate::db_index::{
    LuaExistFieldType, LuaExtendedType, LuaFunctionType, LuaGenericType, LuaIntersectionType,
    LuaMultiReturn, LuaObjectType, LuaTupleType, LuaType, LuaUnionType,
};

pub fn instantiate_type(ty: &LuaType, params: &Vec<LuaType>) -> LuaType {
    match ty {
        LuaType::Array(base) => instantiate_array(base, params),
        LuaType::KeyOf(base) => instantiate_key_of(base, params),
        LuaType::Nullable(base) => instantiate_nullable(base, params),
        LuaType::Tuple(tuple) => instantiate_tuple(tuple, params),
        LuaType::DocFunction(doc_func) => instantiate_doc_function(doc_func, params),
        LuaType::Object(object) => instantiate_object(object, params),
        LuaType::Union(union) => instantiate_union(union, params),
        LuaType::Intersection(intersection) => instantiate_intersection(intersection, params),
        LuaType::Extends(extends) => instantiate_extends(extends, params),
        LuaType::Generic(generic) => instantiate_generic(generic, params),
        LuaType::TableGeneric(table_params) => instantiate_table_generic(table_params, params),
        LuaType::TplRef(idx) => instantiate_tpl_ref(*idx, params),
        LuaType::MuliReturn(multi) => instantiate_multi_return(multi, params),
        LuaType::ExistField(exit_field) => instantiate_exist_field(exit_field, params),
        _ => ty.clone(),
    }
}

fn instantiate_array(base: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(base, params);
    LuaType::Array(base.into())
}

fn instantiate_key_of(base: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(base, params);
    LuaType::KeyOf(base.into())
}

fn instantiate_nullable(inner: &LuaType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(inner, params);
    LuaType::Nullable(base.into())
}

fn instantiate_tuple(tuple: &LuaTupleType, params: &Vec<LuaType>) -> LuaType {
    let tuple_types = tuple.get_types();
    let mut new_types = Vec::new();
    for t in tuple_types {
        let t = instantiate_type(t, params);
        new_types.push(t);
    }
    LuaType::Tuple(LuaTupleType::new(new_types).into())
}

fn instantiate_doc_function(doc_func: &LuaFunctionType, params: &Vec<LuaType>) -> LuaType {
    let func_params = doc_func.get_params();
    let ret = doc_func.get_ret();
    let is_async = doc_func.is_async();
    let mut new_params = Vec::new();
    for (name, param_type) in func_params {
        if param_type.is_some() {
            let new_param = instantiate_type(param_type.as_ref().unwrap(), params);
            new_params.push((name.clone(), Some(new_param)));
            continue;
        } else {
            new_params.push((name.clone(), None));
        }
    }

    let mut new_ret = Vec::new();
    for ret_type in ret {
        let new_ret_type = instantiate_type(ret_type, params);
        new_ret.push(new_ret_type);
    }
    LuaType::DocFunction(LuaFunctionType::new(is_async, new_params, new_ret).into())
}

fn instantiate_object(object: &LuaObjectType, params: &Vec<LuaType>) -> LuaType {
    let fields = object.get_fields();
    let index_acess = object.get_index_access();

    let mut new_fields = HashMap::new();
    for (key, field) in fields {
        let new_field = instantiate_type(field, params);
        new_fields.insert(key.clone(), new_field);
    }

    let mut new_index_access = Vec::new();
    for (key, value) in index_acess {
        let key = instantiate_type(&key, params);
        let value = instantiate_type(&value, params);
        new_index_access.push((key, value));
    }

    LuaType::Object(LuaObjectType::new_with_fields(new_fields, new_index_access).into())
}

fn instantiate_union(union: &LuaUnionType, params: &Vec<LuaType>) -> LuaType {
    let types = union.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(t, params);
        new_types.push(t);
    }

    LuaType::Union(LuaUnionType::new(new_types).into())
}

fn instantiate_intersection(intersection: &LuaIntersectionType, params: &Vec<LuaType>) -> LuaType {
    let types = intersection.get_types();
    let mut new_types = Vec::new();
    for t in types {
        let t = instantiate_type(t, params);
        new_types.push(t);
    }

    LuaType::Intersection(LuaIntersectionType::new(new_types).into())
}

fn instantiate_extends(extends: &LuaExtendedType, params: &Vec<LuaType>) -> LuaType {
    let base = extends.get_base();
    let new_base = instantiate_type(base, params);
    let ext = extends.get_ext();
    let new_ext = instantiate_type(ext, params);
    LuaType::Extends(LuaExtendedType::new(new_base, new_ext).into())
}

fn instantiate_generic(generic: &LuaGenericType, params: &Vec<LuaType>) -> LuaType {
    let generic_params = generic.get_params();
    let mut new_params = Vec::new();
    for param in generic_params {
        let new_param = instantiate_type(param, params);
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

fn instantiate_table_generic(table_params: &Vec<LuaType>, params: &Vec<LuaType>) -> LuaType {
    let mut new_params = Vec::new();
    for param in table_params {
        let new_param = instantiate_type(param, params);
        new_params.push(new_param);
    }

    LuaType::TableGeneric(new_params.into())
}

fn instantiate_tpl_ref(idx: usize, params: &Vec<LuaType>) -> LuaType {
    if let Some(ty) = params.get(idx) {
        ty.clone()
    } else {
        LuaType::Unknown
    }
}

fn instantiate_multi_return(multi_returns: &LuaMultiReturn, params: &Vec<LuaType>) -> LuaType {
    match multi_returns {
        LuaMultiReturn::Base(base) => {
            let new_base = instantiate_type(base, params);
            LuaType::MuliReturn(LuaMultiReturn::Base(new_base).into())
        }
        LuaMultiReturn::Multi(types) => {
            let mut new_types = Vec::new();
            for t in types {
                let t = instantiate_type(t, params);
                new_types.push(t);
            }
            LuaType::MuliReturn(LuaMultiReturn::Multi(new_types).into())
        }
    }
}

fn instantiate_exist_field(exit_field: &LuaExistFieldType, params: &Vec<LuaType>) -> LuaType {
    let base = instantiate_type(&exit_field.get_origin(), params);
    let field = exit_field.get_field();
    LuaType::ExistField(LuaExistFieldType::new(field.clone(), base).into())
}
