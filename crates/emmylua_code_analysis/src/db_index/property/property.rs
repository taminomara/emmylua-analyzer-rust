use emmylua_parser::{LuaVersionCondition, VisibilityKind};
use serde::{Deserialize, Serialize};

use crate::{
    db_index::{member::LuaMemberId, LuaDeclId, LuaSignatureId, LuaTypeDeclId},
    FileId,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaProperty {
    pub id: LuaPropertyId,
    pub description: Option<Box<String>>,
    pub visibility: Option<VisibilityKind>,
    pub source: Option<Box<String>>,
    pub is_deprecated: bool,
    pub deprecated_message: Option<Box<String>>,
    pub version_conds: Option<Box<Vec<LuaVersionCondition>>>,
    pub see_content: Option<Box<String>>,
}

impl LuaProperty {
    pub fn new(id: LuaPropertyId) -> Self {
        Self {
            id,
            description: None,
            visibility: None,
            source: None,
            is_deprecated: false,
            deprecated_message: None,
            version_conds: None,
            see_content: None,
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuaPropertyOwnerId {
    TypeDecl(LuaTypeDeclId),
    Member(LuaMemberId),
    LuaDecl(LuaDeclId),
    Signature(LuaSignatureId),
}

impl From<LuaDeclId> for LuaPropertyOwnerId {
    fn from(id: LuaDeclId) -> Self {
        LuaPropertyOwnerId::LuaDecl(id)
    }
}

impl From<LuaTypeDeclId> for LuaPropertyOwnerId {
    fn from(id: LuaTypeDeclId) -> Self {
        LuaPropertyOwnerId::TypeDecl(id)
    }
}

impl From<LuaMemberId> for LuaPropertyOwnerId {
    fn from(id: LuaMemberId) -> Self {
        LuaPropertyOwnerId::Member(id)
    }
}

impl From<LuaSignatureId> for LuaPropertyOwnerId {
    fn from(id: LuaSignatureId) -> Self {
        LuaPropertyOwnerId::Signature(id)
    }
}

impl LuaPropertyOwnerId {
    pub fn get_file_id(&self) -> Option<FileId> {
        match self {
            LuaPropertyOwnerId::TypeDecl(_) => None,
            LuaPropertyOwnerId::Member(id) => Some(id.file_id),
            LuaPropertyOwnerId::LuaDecl(id) => Some(id.file_id),
            LuaPropertyOwnerId::Signature(id) => Some(id.get_file_id()),
        }
    }
}
