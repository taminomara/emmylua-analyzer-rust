mod find_index;
mod find_members;
mod get_member_map;
mod infer_raw_member;

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    LuaMemberFeature, LuaMemberKey, LuaSemanticDeclId,
};
pub use find_index::find_index_operations;
pub use find_members::find_members;
pub use get_member_map::get_member_map;
pub use infer_raw_member::infer_raw_member_type;

use super::InferFailReason;

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

type FindMembersResult = Option<Vec<LuaMemberInfo>>;
type RawGetMemberTypeResult = Result<LuaType, InferFailReason>;
