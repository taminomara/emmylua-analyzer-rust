use emmylua_parser::LuaKind;
use rowan::{TextRange, TextSize};

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    FileId,
};

#[derive(Debug)]
pub struct LuaMember {
    owner: LuaMemberOwner,
    key: LuaMemberKey,
    file_id: FileId,
    range: TextRange,
    kind: LuaKind,
    decl_type: LuaType,
}

impl LuaMember {
    pub fn new(
        owner: LuaMemberOwner,
        key: LuaMemberKey,
        file_id: FileId,
        kind: LuaKind,
        range: TextRange,
        decl_type: Option<LuaType>,
    ) -> Self {
        Self {
            owner,
            key,
            file_id,
            range,
            kind,
            decl_type: if let Some(decl_type) = decl_type {
                decl_type
            } else {
                LuaType::Unknown
            },
        }
    }
    pub fn get_owner(&self) -> LuaMemberOwner {
        self.owner.clone()
    }

    pub fn get_key(&self) -> &LuaMemberKey {
        &self.key
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn get_decl_type(&self) -> &LuaType {
        &self.decl_type
    }

    pub fn get_id(&self) -> LuaMemberId {
        LuaMemberId::new(self.range.start(), self.file_id)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct LuaMemberId {
    file_id: FileId,
    position: TextSize,
}

impl LuaMemberId {
    pub fn new(position: TextSize, file_id: FileId) -> Self {
        Self { position, file_id }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum LuaMemberOwner {
    Type(LuaTypeDeclId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaMemberKey {
    Integer(i64),
    Name(String),
}
