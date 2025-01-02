use std::fmt;

use crate::LuaSignatureId;
use crate::{db_index::LuaType, FileId};
use emmylua_parser::{LuaKind, LuaSyntaxId, LuaSyntaxKind};
use rowan::{TextRange, TextSize};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    Param {
        name: String,
        file_id: FileId,
        range: TextRange,
        signature_id: LuaSignatureId,
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
            LuaDecl::Param { file_id, .. } => *file_id,
            LuaDecl::Global { file_id, .. } => *file_id,
        }
    }

    pub fn get_id(&self) -> LuaDeclId {
        match self {
            LuaDecl::Local { file_id, .. } => LuaDeclId::new(*file_id, self.get_position()),
            LuaDecl::Param { file_id, .. } => LuaDeclId::new(*file_id, self.get_position()),
            LuaDecl::Global { file_id, .. } => LuaDeclId::new(*file_id, self.get_position()),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            LuaDecl::Local { name, .. } => name,
            LuaDecl::Param { name, .. } => name,
            LuaDecl::Global { name, .. } => name,
        }
    }

    pub fn get_position(&self) -> TextSize {
        match self {
            LuaDecl::Local { range, .. } => range.start(),
            LuaDecl::Param { range, .. } => range.start(),
            LuaDecl::Global { range, .. } => range.start(),
        }
    }
    #[allow(unused)]
    pub fn get_range(&self) -> TextRange {
        match self {
            LuaDecl::Local { range, .. } => *range,
            LuaDecl::Param { range, .. } => *range,
            LuaDecl::Global { range, .. } => *range,
        }
    }

    pub fn get_type(&self) -> Option<&LuaType> {
        match self {
            LuaDecl::Local { decl_type, .. } => decl_type.as_ref(),
            LuaDecl::Global { decl_type, .. } => decl_type.as_ref(),
            LuaDecl::Param { .. } => None,
        }
    }

    pub(crate) fn set_decl_type(&mut self, decl_type: LuaType) {
        match self {
            LuaDecl::Local { decl_type: dt, .. } => *dt = Some(decl_type),
            LuaDecl::Global { decl_type: dt, .. } => *dt = Some(decl_type),
            LuaDecl::Param { .. } => {}
        }
    }

    pub fn get_syntax_id(&self) -> LuaSyntaxId {
        match self {
            LuaDecl::Local { kind, range, .. } => LuaSyntaxId::new(*kind, *range),
            LuaDecl::Param { range, .. } => {
                LuaSyntaxId::new(LuaSyntaxKind::ParamName.into(), *range)
            }
            LuaDecl::Global { range, .. } => {
                LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), *range)
            }
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, LuaDecl::Local { .. } | LuaDecl::Param { .. })
    }

    pub fn is_param(&self) -> bool {
        matches!(self, LuaDecl::Param { .. })
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

impl Serialize for LuaDeclId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = format!("{}|{}", self.file_id.id, u32::from(self.position));
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for LuaDeclId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LuaDeclIdVisitor;

        impl<'de> Visitor<'de> for LuaDeclIdVisitor {
            type Value = LuaDeclId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with format 'file_id:position'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split('|').collect();
                if parts.len() != 2 {
                    return Err(E::custom("expected format 'file_id:position'"));
                }

                let file_id = FileId {
                    id: parts[0]
                        .parse()
                        .map_err(|e| E::custom(format!("invalid file_id: {}", e)))?,
                };
                let position = TextSize::new(
                    parts[1]
                        .parse()
                        .map_err(|e| E::custom(format!("invalid position: {}", e)))?,
                );

                Ok(LuaDeclId { file_id, position })
            }
        }

        deserializer.deserialize_str(LuaDeclIdVisitor)
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
