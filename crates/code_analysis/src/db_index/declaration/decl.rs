use rowan::TextSize;

use crate::FileId;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct LuaDecl {
    name: String,
    id: LuaDeclId,
    position: TextSize
}

impl LuaDecl {
    pub fn new(name: String, id: LuaDeclId, position: TextSize) -> Self {
        Self {
            name,
            id,
            position
        }
    }

    pub fn file_id(&self) -> FileId {
        self.id.file_id
    }

    pub fn id(&self) -> LuaDeclId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> TextSize {
        self.position
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct LuaDeclId {
    pub file_id: FileId,
    pub id: u32,
}