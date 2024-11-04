use flagset::{flags, FlagSet};
use internment::ArcIntern;
use rowan::TextRange;

use crate::FileId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum LuaDeclTypeKind {
    Class,
    Enum,
    Alias,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LuaTypeDecl {
    name: String,
    kind: LuaDeclTypeKind,
    pub(crate) attrib: Option<FlagSet<LuaTypeAttribute>>,
    pub(crate) defined_locations: Vec<LuaDeclLocation>,
    id: LuaTypeDeclId,
}

impl LuaTypeDecl {
    pub fn new(
        file_id: FileId,
        range: TextRange,
        name: String,
        kind: LuaDeclTypeKind,
        attrib: Option<FlagSet<LuaTypeAttribute>>,
        id: LuaTypeDeclId,
    ) -> Self {
        Self {
            name,
            kind,
            attrib,
            defined_locations: vec![LuaDeclLocation { file_id, range }],
            id,
        }
    }

    #[allow(unused)]
    pub fn get_file_ids(&self) -> Vec<FileId> {
        self.defined_locations
            .iter()
            .map(|loc| loc.file_id)
            .collect()
    }

    pub fn get_locations(&self) -> &[LuaDeclLocation] {
        &self.defined_locations
    }

    pub fn get_mut_locations(&mut self) -> &mut Vec<LuaDeclLocation> {
        &mut self.defined_locations
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

    pub fn get_id(&self) -> LuaTypeDeclId {
        self.id.clone()
    }

    pub fn get_full_name(&self) -> &str {
        self.id.get_name()
    }

    pub fn get_namespace(&self) -> Option<&str> {
        self.id.get_name().rfind('.').map(|idx| &self.id.get_name()[..idx])
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct LuaTypeDeclId {
    id: ArcIntern<String>,
}

impl LuaTypeDeclId {
    #[allow(unused)]
    pub fn new_by_id(id: ArcIntern<String>) -> Self {
        Self { id }
    }

    pub fn new(str: &str) -> Self {
        Self {
            id: ArcIntern::new(str.to_string()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuaDeclLocation {
    pub file_id: FileId,
    pub range: TextRange,
}
