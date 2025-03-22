use crate::{LuaType, LuaUnionType};

pub fn remove_type(source: LuaType, removed_type: LuaType) -> LuaType {
    if removed_type.is_nil() {
        return remove_nil_and_not_false(source);
    }

    match source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            types.retain(|t| t != &removed_type);
            if types.len() == 1 {
                types.pop().unwrap()
            } else {
                LuaType::Union(LuaUnionType::new(types).into())
            }
        }
        _ => source,
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
        LuaType::Boolean | LuaType::BooleanConst(_) => LuaType::BooleanConst(true),
        t => t,
    }
}
