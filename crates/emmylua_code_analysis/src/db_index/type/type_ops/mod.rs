mod union_type;

use super::LuaType;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeOps {
    /// Add a type to the source type
    Union,
    /// Remove a type from the source type
    Remove,
    /// Force a type to the source type
    Narrow,
}

impl TypeOps {
    pub fn apply(&self, source: &LuaType, target: &LuaType) -> LuaType {
        match self {
            TypeOps::Union => union_type::union_type(source.clone(), target.clone()),
            // TypeOps::Remove => remove_type(source, target),
            // TypeOps::Narrow => narrow_down_type(source, target),
            _ => source.clone(),
        }
    }
}