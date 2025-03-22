use crate::{LuaType, LuaUnionType};

pub fn remove_type(source: LuaType, removed_type: LuaType) -> Option<LuaType> {
    if source == removed_type {
        return None;
    }

    match (&source, &removed_type) {
        (
            LuaType::Boolean | LuaType::BooleanConst(_) | LuaType::DocBooleanConst(_),
            LuaType::Nil,
        ) => Some(LuaType::BooleanConst(true)),
        (LuaType::TableConst(_) | LuaType::TableGeneric(_), LuaType::Table) => None,
        (left, LuaType::Boolean) if left.is_boolean() => None,
        (left, LuaType::Integer) if left.is_integer() => None,
        (left, LuaType::Number) if left.is_number() => None,
        (left, LuaType::String) if left.is_string() => None,
        (left, LuaType::Unknown) if left.is_unknown() => Some(left.clone()),
        (LuaType::Union(u), right) => {
            let mut types = u
                .get_types()
                .iter()
                .filter_map(|t| remove_type(t.clone(), right.clone()))
                .collect::<Vec<_>>();
            types.dedup();
            if types.len() == 1 {
                return Some(types.pop().unwrap());
            }
            Some(LuaType::Union(LuaUnionType::new(types).into()))
        }
        (_, _) => Some(source),
    }
}
