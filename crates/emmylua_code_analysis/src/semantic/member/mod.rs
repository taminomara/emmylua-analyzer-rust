mod infer_member_map;
mod infer_members;

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    LuaMemberFeature, LuaMemberKey, LuaSemanticDeclId,
};
pub use infer_member_map::infer_member_map;
pub use infer_members::infer_members;

pub fn without_index_operator(type_: &LuaType) -> bool {
    match type_ {
        LuaType::Nil
        | LuaType::Boolean
        | LuaType::BooleanConst(_)
        | LuaType::Integer
        | LuaType::IntegerConst(_)
        | LuaType::Number
        | LuaType::FloatConst(_)
        | LuaType::Function
        | LuaType::DocFunction(_)
        | LuaType::Table
        | LuaType::Userdata
        | LuaType::Thread
        | LuaType::Unknown
        | LuaType::String
        | LuaType::StringConst(_)
        | LuaType::Io
        | LuaType::Any
        | LuaType::StrTplRef(_)
        | LuaType::TplRef(_)
        | LuaType::MuliReturn(_) => true,
        _ => false,
    }
}

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
