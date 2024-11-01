use flagset::{flags, FlagSet};
use rowan::TextRange;

use crate::FileId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum LuaDeclTypeKind {
    Class,
    Enum,
    Alias
}

flags! {
    pub enum LuaTypeAttribute: u8 {
        None,
        Key,
        Local,
        Global,
        Partial,
        Exact
    }
}

#[derive(Debug)]
pub struct LuaTypeDecl {
    file_id: FileId,
    range: TextRange,
    name: String,
    kind: LuaDeclTypeKind,
    attrib: Option<FlagSet<LuaTypeAttribute>>,
}

impl LuaTypeDecl {
    pub fn new(file_id: FileId, range: TextRange, name: String, kind: LuaDeclTypeKind, attrib: Option<FlagSet<LuaTypeAttribute>>) -> Self {
        Self {
            file_id,
            range,
            name,
            kind,
            attrib,
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_kind(&self) -> LuaDeclTypeKind {
        self.kind
    }

    pub fn get_attrib(&self) -> Option<FlagSet<LuaTypeAttribute>> {
        self.attrib
    }

    pub fn get_id(&self) -> Option<LuaTypeDeclId> {
        Some(LuaTypeDeclId::new(self.name.clone(), self.file_id))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct LuaTypeDeclId {
    pub name: String,
    pub file_id: FileId,
}

impl LuaTypeDeclId {
    pub fn new(name: String, file_id: FileId) -> Self {
        Self { name, file_id }
    }
}
