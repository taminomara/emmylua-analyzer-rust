use crate::{LuaType, LuaUnionType};

// need to be optimized
pub fn narrow_down_type(source: LuaType, target: LuaType) -> LuaType {
    match &source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            match target {
                LuaType::Number | LuaType::FloatConst(_) => {
                    types.retain(|t| t.is_number());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Integer | LuaType::IntegerConst(_) => {
                    types.retain(|t| t.is_integer());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::String | LuaType::StringConst(_) => {
                    types.retain(|t| t.is_string());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Boolean | LuaType::BooleanConst(_) => {
                    types.retain(|t| t.is_boolean());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Table | LuaType::TableConst(_) => {
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
                _ => target,
            }
        }
        LuaType::Nullable(inner) => {
            if !target.is_nullable() {
                narrow_down_type(target, (**inner).clone())
            } else {
                LuaType::Nil
            }
        }
        LuaType::BooleanConst(_) => {
            if target.is_boolean() {
                return LuaType::Boolean;
            }

            target
        }
        LuaType::FloatConst(_) => {
            if target.is_number() {
                return LuaType::Number;
            }

            target
        }
        LuaType::IntegerConst(_) => match target {
            LuaType::Number => LuaType::Number,
            LuaType::Integer | LuaType::IntegerConst(_) => LuaType::Integer,
            _ => target,
        },
        LuaType::Number => {
            if target.is_number() {
                return LuaType::Number;
            }

            target
        }
        LuaType::StringConst(_) => {
            if target.is_string() {
                return LuaType::String;
            }

            target
        }
        LuaType::Array(_) => {
            if target.is_table() {
                return source;
            }

            target
        }
        LuaType::Tuple(_) => {
            if target.is_table() {
                return source;
            }

            target
        }
        _ => target,
    }
}
