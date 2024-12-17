use std::str::FromStr;

use emmylua_parser::{LuaKind, LuaSyntaxId, LuaSyntaxKind};
use rowan::{TextRange, TextSize};

use crate::{db_index::LuaType, FileId};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LuaDecl {
    Local {
        name: String,
        file_id: FileId,
        range: TextRange,
        kind: LuaKind,
        attrib: Option<LocalAttribute>,
        decl_type: Option<LuaType>,
    },
    Global {
        name: String,
        file_id: FileId,
        range: TextRange,
        decl_type: Option<LuaType>,
    },
}

impl LuaDecl {
    pub fn get_file_id(&self) -> FileId {
        match self {
            LuaDecl::Local { file_id, .. } => *file_id,
            LuaDecl::Global { file_id, .. } => *file_id,
        }
    }

    pub fn get_id(&self) -> LuaDeclId {
        match self {
            LuaDecl::Local { file_id, .. } => LuaDeclId::new(*file_id, self.get_position()),
            LuaDecl::Global { file_id, .. } => LuaDeclId::new(*file_id, self.get_position()),
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

    pub fn get_type(&self) -> Option<&LuaType> {
        match self {
            LuaDecl::Local { decl_type, .. } => decl_type.as_ref(),
            LuaDecl::Global { decl_type, .. } => decl_type.as_ref(),
        }
    }

    pub (crate) fn set_decl_type(&mut self, decl_type: LuaType) {
        match self {
            LuaDecl::Local { decl_type: dt, .. } => *dt = Some(decl_type),
            LuaDecl::Global { decl_type: dt, .. } => *dt = Some(decl_type),
        }
    }

    pub fn get_syntax_id(&self) -> LuaSyntaxId {
        match self {
            LuaDecl::Local { kind, range, .. } => LuaSyntaxId::new(*kind, *range),
            LuaDecl::Global { range, .. } => LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), *range),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, LuaDecl::Local { .. })
    }

    pub fn is_global(&self) -> bool {
        matches!(self, LuaDecl::Global { .. })
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct LuaDeclId {
    pub file_id: FileId,
    pub position: TextSize,
}

impl FromStr for LuaDeclId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let file_id = parts[0].parse().map_err(|_| ())?;
        let position = parts[1].parse::<u32>().map_err(|_| ())?;
        Ok(Self { file_id, position: position.into() })
    }
}

impl ToString for LuaDeclId {
    fn to_string(&self) -> String {
        format!("{}:{}", self.file_id.to_string(), u32::from(self.position))
    }
}

impl LuaDeclId {
    pub fn new(file_id: FileId, position: TextSize) -> Self {
        Self { file_id, position }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LocalAttribute {
    Const,
    Close,
    IterConst,
}
