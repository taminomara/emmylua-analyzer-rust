use crate::{LuaType, LuaUnionType};

// need to be optimized
pub fn narrow_down_type(source: LuaType, target: LuaType) -> Option<LuaType> {
    if source == target {
        return Some(source);
    }

    match &target {
        LuaType::Number => {
            if source.is_number() {
                return Some(source);
            }
        }
        LuaType::Integer => {
            if source.is_integer() {
                return Some(source);
            }
        }
        LuaType::String => {
            if source.is_string() {
                return Some(source);
            }
        }
        LuaType::Boolean => {
            if source.is_boolean() {
                return Some(source);
            }
        }
        LuaType::Table => match &source {
            LuaType::TableConst(_) => {
                return Some(source);
            }
            LuaType::Table | LuaType::Userdata | LuaType::Any | LuaType::Unknown => {
                return Some(LuaType::Table);
            }
            LuaType::Ref(_)
            | LuaType::Def(_)
            | LuaType::Global
            | LuaType::Array(_)
            | LuaType::Tuple(_)
            | LuaType::Generic(_)
            | LuaType::TableGeneric(_) => return Some(source),
            _ => {}
        },
        LuaType::Function => {
            if source.is_function() {
                return Some(source);
            }
        }
        LuaType::Thread => {
            if source.is_thread() {
                return Some(source);
            }
        }
        LuaType::Userdata => {
            if source.is_userdata() {
                return Some(source);
            }
        }
        LuaType::Nil => {
            if source.is_nil() {
                return Some(source);
            }
        }
        LuaType::Any => {
            return Some(source);
        }
        LuaType::FloatConst(f) => {
            if source.is_number() {
                return Some(LuaType::Number);
            } else if source.is_unknown() {
                return Some(LuaType::FloatConst(*f));
            }
        }
        LuaType::IntegerConst(i) => match &source {
            LuaType::DocIntegerConst(i2) => {
                if i == i2 {
                    return Some(LuaType::IntegerConst(*i));
                }
            }
            LuaType::Number | LuaType::Integer | LuaType::Any | LuaType::Unknown => {
                return Some(LuaType::IntegerConst(*i));
            }
            LuaType::IntegerConst(_) => {
                return Some(LuaType::Integer);
            }
            _ => {}
        },
        LuaType::StringConst(s) => match &source {
            LuaType::DocStringConst(s2) => {
                if s == s2 {
                    return Some(LuaType::DocStringConst(s.clone()));
                }
            }
            LuaType::String | LuaType::Any | LuaType::Unknown => {
                return Some(LuaType::StringConst(s.clone()));
            }
            LuaType::StringConst(_) => {
                return Some(LuaType::String);
            }
            _ => {}
        },
        LuaType::TableConst(t) => match &source {
            LuaType::TableConst(s) => {
                return Some(LuaType::TableConst(s.clone()));
            }
            LuaType::Table | LuaType::Userdata | LuaType::Any | LuaType::Unknown => {
                return Some(LuaType::TableConst(t.clone()));
            }
            LuaType::Ref(_)
            | LuaType::Def(_)
            | LuaType::Global
            | LuaType::Array(_)
            | LuaType::Tuple(_)
            | LuaType::Generic(_)
            | LuaType::TableGeneric(_) => return Some(source),
            _ => {}
        },
        LuaType::Instance(base) => return narrow_down_type(source, base.get_base().clone()),
        LuaType::Unknown => return Some(source),
        _ => {
            if target.is_unknown() {
                return Some(source);
            }

            return Some(target);
        }
    }

    match &source {
        LuaType::Union(union) => {
            let mut union_types = union
                .get_types()
                .iter()
                .filter_map(|t| narrow_down_type(t.clone(), target.clone()))
                .collect::<Vec<_>>();

            union_types.dedup();
            return match union_types.len() {
                0 => Some(target),
                1 => Some(union_types[0].clone()),
                _ => Some(LuaType::Union(LuaUnionType::new(union_types).into())),
            };
        }
        _ => {}
    }

    None
}
