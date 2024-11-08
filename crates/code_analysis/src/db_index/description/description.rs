use crate::db_index::{member::LuaMemberId, LuaDeclId, LuaTypeDeclId};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaDescription {
    pub owner_id: LuaDescriptionOwnerId,
    pub text: String,
}

impl LuaDescription {
    pub fn new(owner_id: LuaDescriptionOwnerId, text: String) -> Self {
        Self { owner_id, text }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct LuaDescriptionId {
    id: u32
}

impl LuaDescriptionId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LuaDescriptionOwnerId {
    TypeDecl(LuaTypeDeclId),
    Member(LuaMemberId),
    Variable(LuaDeclId)
}