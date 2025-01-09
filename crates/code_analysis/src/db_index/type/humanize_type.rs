use crate::{
    DbIndex, GenericTpl, LuaExistFieldType, LuaExtendedType, LuaFunctionType, LuaGenericType,
    LuaInstanceType, LuaIntersectionType, LuaMemberKey, LuaMultiReturn, LuaObjectType,
    LuaSignatureId, LuaStringTplType, LuaTupleType, LuaType, LuaTypeDeclId, LuaUnionType,
};

#[allow(unused)]
pub fn humanize_type(db: &DbIndex, ty: &LuaType) -> String {
    match ty {
        LuaType::Any => "any".to_string(),
        LuaType::Nil => "nil".to_string(),
        LuaType::Boolean => "boolean".to_string(),
        LuaType::Number => "number".to_string(),
        LuaType::String => "string".to_string(),
        LuaType::Table => "table".to_string(),
        LuaType::Function => "function".to_string(),
        LuaType::Thread => "thread".to_string(),
        LuaType::Userdata => "userdata".to_string(),
        LuaType::IntegerConst(i) => i.to_string(),
        LuaType::FloatConst(f) => f.to_string(),
        LuaType::TableConst(_) => "table".to_string(),
        LuaType::Def(id) => humanize_def_type(db, id),
        LuaType::Union(union) => humanize_union_type(db, union),
        LuaType::Tuple(tuple) => humanize_tuple_type(db, tuple),
        LuaType::Unknown => "unknown".to_string(),
        LuaType::Integer => "interger".to_string(),
        LuaType::Io => "io".to_string(),
        LuaType::SelfInfer => "self".to_string(),
        LuaType::BooleanConst(b) => b.to_string(),
        LuaType::StringConst(s) => format!("\"{}\"", s),
        LuaType::DocStringConst(s) => format!("\"{}\"", s),
        LuaType::DocIntergerConst(i) => i.to_string(),
        LuaType::Ref(id) => {
            if let Some(type_decl) = db.get_type_index().get_type_decl(id) {
                type_decl.get_name().to_string()
            } else {
                id.get_name().to_string()
            }
        }
        LuaType::Module(module_path) => humanize_module_type(db, module_path),
        LuaType::Array(arr_inner) => humanize_array_type(db, arr_inner),
        LuaType::KeyOf(base_type) => humanize_key_of_type(db, base_type),
        LuaType::Nullable(inner) => humanize_nullable_type(db, inner),
        LuaType::DocFunction(lua_func) => humanize_doc_function_type(db, lua_func),
        LuaType::Object(object) => humanize_object_type(db, object),
        LuaType::Intersection(inter) => humanize_intersect_type(db, inter),
        LuaType::Extends(ext) => humanize_extend_type(db, ext),
        LuaType::Generic(generic) => humanize_generic_type(db, generic),
        LuaType::TableGeneric(table_generic_params) => {
            humanize_table_generic_type(db, table_generic_params)
        }
        LuaType::TplRef(tpl) => humanize_tpl_ref_type(tpl),
        LuaType::StrTplRef(str_tpl) => humanize_str_tpl_ref_type(str_tpl),
        LuaType::FuncTplRef(tpl) => humanize_tpl_ref_type(tpl),
        LuaType::MuliReturn(multi) => humanize_multi_return_type(db, multi),
        LuaType::ExistField(exist_field) => humanize_exist_field_type(db, exist_field),
        LuaType::Instance(ins) => humanize_instance_type(db, ins),
        LuaType::Signature(signature_id) => humanize_signature_type(db, signature_id),
        _ => "unknown".to_string(),
    }
}

fn humanize_def_type(db: &DbIndex, id: &LuaTypeDeclId) -> String {
    let type_decl = db.get_type_index().get_type_decl(id);
    if type_decl.is_none() {
        return id.get_name().to_string();
    }

    let type_decl = type_decl.unwrap();
    let simple_name = type_decl.get_name();
    let generic = db.get_type_index().get_generic_params(id);
    if generic.is_none() {
        return simple_name.to_string();
    }

    let generic_names = generic
        .unwrap()
        .iter()
        .map(|it| it.0.clone())
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}<{}>", simple_name, generic_names)
}

fn humanize_union_type(db: &DbIndex, union: &LuaUnionType) -> String {
    let types = union.get_types();
    let dots = if types.len() > 10 { "..." } else { "" };

    let type_str = types
        .iter()
        .take(10)
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join("|");
    format!("({}{})", type_str, dots)
}

fn humanize_tuple_type(db: &DbIndex, tuple: &LuaTupleType) -> String {
    let types = tuple.get_types();
    let dots = if types.len() > 10 { "..." } else { "" };

    let type_str = types
        .iter()
        .take(10)
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join(",");
    format!("({}{})", type_str, dots)
}

fn humanize_module_type(db: &DbIndex, module_path: &str) -> String {
    let module = db.get_module_index().find_module(module_path);
    if module.is_none() {
        return format!("module({})", module_path);
    }

    let module = module.unwrap();
    if module.export_type.is_none() {
        return format!("module({})", module_path);
    }

    humanize_type(db, &module.export_type.clone().unwrap())
}

fn humanize_array_type(db: &DbIndex, inner: &LuaType) -> String {
    let element_type = humanize_type(db, inner);
    format!("{}[]", element_type)
}

fn humanize_key_of_type(db: &DbIndex, inner: &LuaType) -> String {
    let element_type = humanize_type(db, inner);
    format!("keyof {}", element_type)
}

fn humanize_nullable_type(db: &DbIndex, inner: &LuaType) -> String {
    let element_type = humanize_type(db, inner);
    format!("{}?", element_type)
}

fn humanize_doc_function_type(db: &DbIndex, lua_func: &LuaFunctionType) -> String {
    let prev = if lua_func.is_async() { "async " } else { "" };
    let params = lua_func
        .get_params()
        .iter()
        .map(|param| {
            let name = param.0.clone();
            if let Some(ty) = &param.1 {
                format!("{}: {}", name, humanize_type(db, ty))
            } else {
                name.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    let rets = lua_func.get_ret();

    if rets.is_empty() {
        return format!("{}({})", prev, params);
    }

    let ret_strs = rets
        .iter()
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join(",");

    if rets.len() > 1 {
        return format!("{}({}) => ({})", prev, params, ret_strs);
    }
    format!("{}({}) => {}", prev, params, ret_strs)
}

fn humanize_object_type(db: &DbIndex, object: &LuaObjectType) -> String {
    let fields = object
        .get_fields()
        .iter()
        .map(|field| {
            let name = field.0.clone();
            let ty_str = humanize_type(db, field.1);
            match name {
                LuaMemberKey::Integer(i) => format!("[{}]: {}", i, ty_str),
                LuaMemberKey::Name(s) => format!("{}: {}", s, ty_str),
                LuaMemberKey::None => ty_str,
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    let access = object
        .get_index_access()
        .iter()
        .map(|(key, value)| {
            let key_str = humanize_type(db, key);
            let value_str = humanize_type(db, value);
            format!("[{}]: {}", key_str, value_str)
        })
        .collect::<Vec<_>>()
        .join(",");

    if access.is_empty() {
        return format!("{{ {} }}", fields);
    }
    format!("{{ {}, {} }}", fields, access)
}

fn humanize_intersect_type(db: &DbIndex, inter: &LuaIntersectionType) -> String {
    let types = inter.get_types();
    let dots = if types.len() > 10 { "..." } else { "" };

    let type_str = types
        .iter()
        .take(10)
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join("&");
    format!("({}{})", type_str, dots)
}

fn humanize_extend_type(db: &DbIndex, ext: &LuaExtendedType) -> String {
    let base = humanize_type(db, ext.get_base());
    let extends = humanize_type(db, ext.get_ext());

    format!("{} extends {}", base, extends)
}

fn humanize_generic_type(db: &DbIndex, generic: &LuaGenericType) -> String {
    let base_id = generic.get_base_type_id();
    let type_decl = db.get_type_index().get_type_decl(&base_id);
    if type_decl.is_none() {
        return base_id.get_name().to_string();
    }

    let simple_name = type_decl.unwrap().get_name();

    let generic_params = generic
        .get_params()
        .iter()
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join(",");

    format!("{}<{}>", simple_name, generic_params)
}

fn humanize_table_generic_type(db: &DbIndex, table_generic_params: &Vec<LuaType>) -> String {
    let generic_params = table_generic_params
        .iter()
        .map(|ty| humanize_type(db, ty))
        .collect::<Vec<_>>()
        .join(",");

    format!("table<{}>", generic_params)
}

fn humanize_tpl_ref_type(tpl: &GenericTpl) -> String {
    tpl.get_name().to_string()
}

fn humanize_str_tpl_ref_type(str_tpl: &LuaStringTplType) -> String {
    let prefix = str_tpl.get_prefix();
    if prefix.is_empty() {
        str_tpl.get_name().to_string()
    } else {
        format!("{}`{}`", prefix, str_tpl.get_name())
    }
}

fn humanize_multi_return_type(db: &DbIndex, multi: &LuaMultiReturn) -> String {
    match multi {
        LuaMultiReturn::Base(base) => {
            let base_str = humanize_type(db, base);
            format!("{} ...", base_str)
        }
        LuaMultiReturn::Multi(types) => {
            let type_str = types
                .iter()
                .map(|ty| humanize_type(db, ty))
                .collect::<Vec<_>>()
                .join(",");
            format!("({})", type_str)
        }
    }
}

// optimize
fn humanize_exist_field_type(db: &DbIndex, exist_field: &LuaExistFieldType) -> String {
    humanize_type(db, exist_field.get_origin())
}

fn humanize_instance_type(db: &DbIndex, ins: &LuaInstanceType) -> String {
    humanize_type(db, ins.get_base())
}

fn humanize_signature_type(db: &DbIndex, signature_id: &LuaSignatureId) -> String {
    let signature = db.get_signature_index().get(signature_id);
    if signature.is_none() {
        return "unknown".to_string();
    }

    let signature = signature.unwrap();
    let params = signature
        .get_type_params()
        .iter()
        .map(|param| {
            let name = param.0.clone();
            if let Some(ty) = &param.1 {
                format!("{}: {}", name, humanize_type(db, ty))
            } else {
                name.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    let generics = signature
        .generic_params
        .iter()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let rets = signature
        .return_docs
        .iter()
        .map(|ret| humanize_type(db, &ret.type_ref))
        .collect::<Vec<_>>();

    let generic_str = if generics.is_empty() {
        "".to_string()
    } else {
        format!("<{}>", generics)
    };

    let ret_str = if rets.is_empty() {
        "".to_string()
    } else {
        format!(" => {}", rets.join(","))
    };

    format!("fun{}({}){}", generic_str, params, ret_str)
}
