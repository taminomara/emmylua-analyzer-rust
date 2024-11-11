use std::collections::HashMap;

use rowan::TextSize;

use crate::{db_index::LuaType, FileId};

#[derive(Debug)]
pub struct LuaSignature {
    pub generic_params: Vec<(String, Option<LuaType>)>,
    pub visibility: Option<String>,
    pub overloads: Vec<LuaType>,
    pub param_docs: HashMap<String, LuaDocParamInfo>,
    pub params: Vec<String>,
    pub return_docs: Vec<LuaDocReturnInfo>,
}

impl LuaSignature {
    pub fn new() -> Self {
        Self {
            generic_params: Vec::new(),
            visibility: None,
            overloads: Vec::new(),
            param_docs: HashMap::new(),
            params: Vec::new(),
            return_docs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LuaDocParamInfo {
    pub name: String,
    pub type_ref: LuaType,
    pub nullable: bool,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct LuaDocReturnInfo {
    pub name: Option<String>,
    pub type_ref: LuaType,
    pub description: Option<String>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct LuaSignatureId {
    pub file_id: FileId,
    pub position: TextSize,
}

impl LuaSignatureId {
    pub fn new(file_id: FileId, position: TextSize) -> Self {
        Self { file_id, position }
    }
}