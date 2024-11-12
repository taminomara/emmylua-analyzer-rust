use emmylua_parser::VisibilityKind;

use crate::db_index::{member::LuaMemberId, LuaDeclId, LuaSignatureId, LuaTypeDeclId};

use super::version::LuaVersionCond;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaProperty {
    pub id: LuaPropertyId,
    pub owner: LuaPropertyOwnerId,
    pub description: Option<Box<String>>,
    pub visibility: Option<VisibilityKind>,
    pub source: Option<Box<String>>,
    pub nodiscard: bool,
    pub deprecated: bool,
    pub deprecated_message: Option<Box<String>>,
    pub version_conds: Option<Box<Vec<LuaVersionCond>>>,
}

impl LuaProperty {
    pub fn new(owner: LuaPropertyOwnerId, id: LuaPropertyId) -> Self {
        Self {
            id,
            owner,
            description: None,
            visibility: None,
            source: None,
            nodiscard: false,
            deprecated: false,
            deprecated_message: None,
            version_conds: None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct LuaPropertyId {
    id: u32,
}

impl LuaPropertyId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LuaPropertyOwnerId {
    TypeDecl(LuaTypeDeclId),
    Member(LuaMemberId),
    LuaDecl(LuaDeclId),
    Signature(LuaSignatureId),
}
