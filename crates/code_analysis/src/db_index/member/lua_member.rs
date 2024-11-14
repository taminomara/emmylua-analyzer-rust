use emmylua_parser::{LuaKind, LuaSyntaxId};
use internment::ArcIntern;
use rowan::{TextRange, TextSize};

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    FileId, InFiled,
};

#[derive(Debug)]
pub struct LuaMember {
    pub(super) owner: LuaMemberOwner,
    key: LuaMemberKey,
    file_id: FileId,
    syntax_id: LuaSyntaxId,
    pub(crate) decl_type: LuaType,
}

impl LuaMember {
    pub fn new(
        owner: LuaMemberOwner,
        key: LuaMemberKey,
        file_id: FileId,
        id: LuaSyntaxId,
        decl_type: Option<LuaType>,
    ) -> Self {
        Self {
            owner,
            key,
            file_id,
            syntax_id: id,
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
        self.syntax_id.get_range()
    }

    pub fn get_decl_type(&self) -> &LuaType {
        &self.decl_type
    }

    pub fn get_id(&self) -> LuaMemberId {
        LuaMemberId::new(self.syntax_id, self.file_id)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct LuaMemberId {
    file_id: FileId,
    id: LuaSyntaxId,
}

impl LuaMemberId {
    pub fn new(id: LuaSyntaxId, file_id: FileId) -> Self {
        Self { id, file_id }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum LuaMemberOwner {
    None,
    Type(LuaTypeDeclId),
    Table(InFiled<TextRange>)
}

impl LuaMemberOwner {
    pub fn get_type_id(&self) -> Option<&LuaTypeDeclId> {
        match self {
            LuaMemberOwner::Type(id) => Some(id),
            _ => None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, LuaMemberOwner::None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaMemberKey {
    Integer(i64),
    Name(ArcIntern<String>),
}
