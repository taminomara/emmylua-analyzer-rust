use std::{collections::HashMap, str::FromStr, sync::Arc};

use emmylua_parser::{LuaAstNode, LuaClosureExpr};
use rowan::TextSize;

use crate::{db_index::{LuaFunctionType, LuaType}, FileId};

#[derive(Debug)]
pub struct LuaSignature {
    pub generic_params: Vec<(String, Option<LuaType>)>,
    pub overloads: Vec<Arc<LuaFunctionType>>,
    pub param_docs: HashMap<String, LuaDocParamInfo>,
    pub params: Vec<String>,
    pub return_docs: Vec<LuaDocReturnInfo>,
    pub(crate) resolve_return: bool,
    pub is_colon_define: bool,
}

impl LuaSignature {
    pub fn new() -> Self {
        Self {
            generic_params: Vec::new(),
            overloads: Vec::new(),
            param_docs: HashMap::new(),
            params: Vec::new(),
            return_docs: Vec::new(),
            resolve_return: false,
            is_colon_define: false,
        }
    }

    pub fn is_generic(&self) -> bool {
        !self.generic_params.is_empty()
    }

    pub fn is_resolve_return(&self) -> bool {
        self.resolve_return || !self.return_docs.is_empty()
    }

    pub fn get_type_params(&self) -> Vec<(String, Option<LuaType>)> {
        let mut type_params = Vec::new();
        for param_name in &self.params {
            if let Some(param_info) = self.param_docs.get(param_name) {
                type_params.push((param_name.clone(), Some(param_info.type_ref.clone())));
            } else {
                type_params.push((param_name.clone(), None));
            }
        }

        type_params
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
    file_id: FileId,
    position: TextSize,
}

impl FromStr for LuaSignatureId {
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

impl ToString for LuaSignatureId {
    fn to_string(&self) -> String {
        format!("{}|{}", self.file_id.to_string(), u32::from(self.position))
    }
}


impl LuaSignatureId {
    pub fn new(file_id: FileId, closure: &LuaClosureExpr) -> Self {
        Self {
            file_id,
            position: closure.get_position(),
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_position(&self) -> TextSize {
        self.position
    }
}
