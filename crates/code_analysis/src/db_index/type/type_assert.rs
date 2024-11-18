
use std::sync::Arc;

use crate::db_index::LuaReferenceKey;

use super::{LuaType, LuaUnionType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeAssertion {
    Exist,
    IsLuaType(Arc<LuaType>),
    FieldExist(Arc<LuaReferenceKey>),
    NonExist,
    NonExistField(Arc<LuaReferenceKey>),
}

impl TypeAssertion {
    pub fn tighten_type(&self, source: LuaType) -> LuaType {
        match self {
            TypeAssertion::Exist => remove_nil_and_not_false(source),
            TypeAssertion::IsLuaType(t) => (**t).clone(),
            // TODO: check if the field is exist
            TypeAssertion::FieldExist(_) => source,
            TypeAssertion::NonExist => {
                if source.is_boolean() {
                    LuaType::BooleanConst(false)
                } else {
                    LuaType::Nil
                }
            },
            // TODO: check if the field is not exist
            TypeAssertion::NonExistField(_) => LuaType::Nil,
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
