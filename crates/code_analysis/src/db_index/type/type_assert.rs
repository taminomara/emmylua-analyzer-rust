use std::sync::Arc;

use crate::db_index::LuaReferenceKey;

use super::{LuaExistField, LuaType, LuaUnionType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeAssertion {
    Exist,
    IsNativeLuaType(LuaType),
    FieldExist(Arc<LuaReferenceKey>),
}

#[allow(unused)]
impl TypeAssertion {
    pub fn tighten_type(&self, source: LuaType) -> LuaType {
        match self {
            TypeAssertion::Exist => remove_nil_and_not_false(source),
            TypeAssertion::IsNativeLuaType(t) => force_type(t.clone(), source),
            TypeAssertion::FieldExist(key) => LuaType::ExistField(LuaExistField::new((**key).clone(), source).into()),
        }
    }
}

fn remove_nil_and_not_false(t: LuaType) -> LuaType {
    match t {
        LuaType::Nil => LuaType::Unknown,
        LuaType::Union(types) => {
            let mut new_types = Vec::new();
            for t in types.get_types() {
                let t = remove_nil_and_not_false(t.clone());
                if t != LuaType::Unknown {
                    new_types.push(t);
                }
            }
            if new_types.len() == 1 {
                new_types.pop().unwrap()
            } else {
                LuaType::Union(LuaUnionType::new(new_types).into())
            }
        }
        LuaType::Nullable(t) => remove_nil_and_not_false(*t),
        t => t,
    }
}

fn force_type(target: LuaType, source: LuaType) -> LuaType {
    match &source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            match target {
                LuaType::Number => {
                    types.retain(|t| t.is_number());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::String => {
                    types.retain(|t| t.is_string());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Boolean => {
                    types.retain(|t| t.is_boolean());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Table => {
                    types.retain(|t| t.is_table());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Function => {
                    types.retain(|t| t.is_function());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Thread => {
                    types.retain(|t| t.is_thread());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Userdata => {
                    types.retain(|t| t.is_userdata());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Nil => {
                    types.retain(|t| t.is_nil());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                _ => target
            }
        }
        LuaType::Nullable(inner) => {
            if !target.is_nullable() {
                force_type(target, *inner.clone())
            } else {
                LuaType::Nil
            }
        }
        _ => target,
    }
}
