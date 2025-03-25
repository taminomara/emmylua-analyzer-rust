mod infer_member_map;
mod infer_members;

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    LuaMemberFeature, LuaMemberKey, LuaSemanticDeclId,
};
pub use infer_member_map::infer_member_map;
pub use infer_members::infer_members;

pub fn get_buildin_type_map_type_id(type_: &LuaType) -> Option<LuaTypeDeclId> {
    match type_ {
        LuaType::String | LuaType::StringConst(_) | LuaType::DocStringConst(_) => {
            Some(LuaTypeDeclId::new("string"))
        }
        LuaType::Io => Some(LuaTypeDeclId::new("io")),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaMemberInfo {
    pub property_owner_id: Option<LuaSemanticDeclId>,
    pub key: LuaMemberKey,
    pub typ: LuaType,
    pub feature: Option<LuaMemberFeature>,
    pub overload_index: Option<usize>,
}

type InferMembersResult = Option<Vec<LuaMemberInfo>>;
