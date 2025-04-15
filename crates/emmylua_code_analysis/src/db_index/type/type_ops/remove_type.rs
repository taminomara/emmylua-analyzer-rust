use crate::{LuaType, LuaUnionType};

pub fn remove_type(source: LuaType, removed_type: LuaType) -> Option<LuaType> {
    if source == removed_type {
        match source {
            LuaType::IntegerConst(_) => return Some(LuaType::Integer),
            LuaType::FloatConst(_) => return Some(LuaType::Number),
            _ => return None,
        }
    }

    match &removed_type {
        LuaType::Nil => match &source {
            LuaType::Boolean => return Some(LuaType::BooleanConst(true)),
            LuaType::BooleanConst(b) => {
                if *b {
                    return Some(LuaType::BooleanConst(true));
                } else {
                    return None;
                }
            }
            LuaType::DocBooleanConst(b) => {
                if *b {
                    return Some(LuaType::DocBooleanConst(true));
                } else {
                    return None;
                }
            }
            _ => {}
        },
        LuaType::Boolean => {
            if source.is_boolean() {
                return None;
            }
        }
        LuaType::Integer => {
            if source.is_integer() {
                return None;
            }
        }
        LuaType::Number => {
            if source.is_number() {
                return None;
            }
        }
        LuaType::String => {
            if source.is_string() {
                return None;
            }
        }
        LuaType::Io => {
            if source.is_io() {
                return None;
            }
        }
        LuaType::Function => {
            if source.is_function() {
                return None;
            }
        }
        LuaType::Thread => {
            if source.is_thread() {
                return None;
            }
        }
        LuaType::Userdata => {
            if source.is_userdata() {
                return None;
            }
        }
        LuaType::Table => match &source {
            LuaType::TableConst(_)
            | LuaType::Table
            | LuaType::Userdata
            | LuaType::Ref(_)
            | LuaType::Def(_)
            | LuaType::Global
            | LuaType::Array(_)
            | LuaType::Tuple(_)
            | LuaType::Generic(_)
            | LuaType::TableGeneric(_) => return None,
            _ => {}
        },
        LuaType::DocStringConst(s) => match &source {
            LuaType::DocStringConst(s2) => {
                if s == s2 {
                    return None;
                }
            }
            LuaType::StringConst(s2) => {
                if s == s2 {
                    return None;
                }
            }
            _ => {}
        },
        LuaType::StringConst(s) => match &source {
            LuaType::DocStringConst(s2) => {
                if s == s2 {
                    return None;
                }
            }
            LuaType::StringConst(s2) => {
                if s == s2 {
                    return None;
                }
            }
            _ => {}
        },
        LuaType::DocIntegerConst(i) => match &source {
            LuaType::DocIntegerConst(i2) => {
                if i == i2 {
                    return None;
                }
            }
            LuaType::IntegerConst(i2) => {
                if i == i2 {
                    return None;
                }
            }
            _ => {}
        },
        LuaType::IntegerConst(i) => match &source {
            LuaType::DocIntegerConst(i2) => {
                if i == i2 {
                    return None;
                }
            }
            LuaType::IntegerConst(i2) => {
                if i == i2 {
                    return None;
                }
            }
            _ => {}
        },
        _ => {}
    }

    if let LuaType::Union(u) = &source {
        let mut types = u
            .get_types()
            .iter()
            .filter_map(|t| remove_type(t.clone(), removed_type.clone()))
            .collect::<Vec<_>>();
        types.dedup();
        if types.len() == 1 {
            return Some(types.pop().unwrap());
        }
        return Some(LuaType::Union(LuaUnionType::new(types).into()));
    } else if let LuaType::Union(u) = &removed_type {
        let mut types = u
            .get_types()
            .iter()
            .filter_map(|t| remove_type(source.clone(), t.clone()))
            .collect::<Vec<_>>();
        types.dedup();
        if types.len() == 1 {
            return Some(types.pop().unwrap());
        }
        return Some(LuaType::Union(LuaUnionType::new(types).into()));
    }

    Some(source)
}
