use rowan::TextSize;

use crate::{db_index::LuaType, FileId};

#[derive(Debug)]
pub struct LuaSignature {
    pub generic_params: Vec<(String, Option<LuaType>)>,
    pub visibility: Option<String>,
    pub overloads: Vec<LuaType>,
    pub params: Vec<LuaDocParamInfo>,
    pub return_info: LuaDocReturnInfo,
}

impl LuaSignature {
    pub fn new() -> Self {
        Self {
            generic_params: Vec::new(),
            visibility: None,
            overloads: Vec::new(),
            params: Vec::new(),
            return_info: LuaDocReturnInfo {
                type_ref_and_name_list: Vec::new(),
            },
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
    pub type_ref_and_name_list: Vec<(Option<String>, LuaType)>,
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