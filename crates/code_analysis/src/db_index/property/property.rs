use std::str::FromStr;

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
    pub is_nodiscard: bool,
    pub is_deprecated: bool,
    pub deprecated_message: Option<Box<String>>,
    pub version_conds: Option<Box<Vec<LuaVersionCond>>>,
    pub is_async: bool,
}

impl LuaProperty {
    pub fn new(owner: LuaPropertyOwnerId, id: LuaPropertyId) -> Self {
        Self {
            id,
            owner,
            description: None,
            visibility: None,
            source: None,
            is_nodiscard: false,
            is_deprecated: false,
            deprecated_message: None,
            version_conds: None,
            is_async: false,
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

impl FromStr for LuaPropertyOwnerId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(());
        }

        match parts[0] {
            "TypeDecl" => parts[1].parse().map(LuaPropertyOwnerId::TypeDecl).map_err(|_| ()),
            "Member" => parts[1].parse().map(LuaPropertyOwnerId::Member).map_err(|_| ()),
            "LuaDecl" => parts[1].parse().map(LuaPropertyOwnerId::LuaDecl).map_err(|_| ()),
            "Signature" => parts[1].parse().map(LuaPropertyOwnerId::Signature).map_err(|_| ()),
            _ => Err(()),
        }
    }
}

impl ToString for LuaPropertyOwnerId {
    fn to_string(&self) -> String {
        match self {
            LuaPropertyOwnerId::TypeDecl(id) => format!("TypeDecl:{}", id.to_string()),
            LuaPropertyOwnerId::Member(id) => format!("Member:{}", id.to_string()),
            LuaPropertyOwnerId::LuaDecl(id) => format!("LuaDecl:{}", id.to_string()),
            LuaPropertyOwnerId::Signature(id) => format!("Signature:{}", id.to_string()),
        }
    }
}