use rowan::{TextRange, TextSize};

use crate::FileId;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LuaDecl {
    Local {
        name: String,
        id: Option<LuaDeclId>,
        range: TextRange,
        attrib: Option<LocalAttribute>,
    },
    Global {
        name: String,
        id: Option<LuaDeclId>,
        range: TextRange,
    },
}

impl LuaDecl {
    #[allow(unused)]
    pub fn get_file_id(&self) -> Option<FileId> {
        match self {
            LuaDecl::Local { id, .. } => id.map(|id| id.file_id),
            LuaDecl::Global { id, .. } => id.map(|id| id.file_id),
        }
    }

    pub fn get_id(&self) -> LuaDeclId {
        match self {
            LuaDecl::Local { id, .. } => (*id).unwrap(),
            LuaDecl::Global { id, .. } => (*id).unwrap(),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            LuaDecl::Local { name, .. } => name,
            LuaDecl::Global { name, .. } => name,
        }
    }

    pub fn get_position(&self) -> TextSize {
        match self {
            LuaDecl::Local { range, .. } => range.start(),
            LuaDecl::Global { range, .. } => range.start(),
        }
    }
    #[allow(unused)]
    pub fn get_range(&self) -> TextRange {
        match self {
            LuaDecl::Local { range, .. } => *range,
            LuaDecl::Global { range, .. } => *range,
        }
    }

    pub fn set_id(&mut self, id: LuaDeclId) {
        match self {
            LuaDecl::Local { id: id_ref, .. } => *id_ref = Some(id),
            LuaDecl::Global { id: id_ref, .. } => *id_ref = Some(id),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct LuaDeclId {
    pub file_id: FileId,
    pub id: u32,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LocalAttribute {
    Const,
    Close,
    IterConst,
}
