use emmylua_parser::LuaKind;
use rowan::TextRange;

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    FileId,
};

#[derive(Debug)]
pub struct LuaMember {
    owner: LuaMemberOwner,
    name: String,
    file_id: FileId,
    range: TextRange,
    kind: LuaKind,
    decl_type: LuaType,
}

impl LuaMember {
    pub fn new(
        owner: LuaMemberOwner,
        name: String,
        file_id: FileId,
        kind: LuaKind,
        range: TextRange,
        decl_type: Option<LuaType>,
    ) -> Self {
        Self {
            owner,
            name,
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

    pub fn get_name(&self) -> &str {
        &self.name
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
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct LuaMemberId {
    id: usize,
}

impl LuaMemberId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum LuaMemberOwner {
    Type(LuaTypeDeclId),
}
