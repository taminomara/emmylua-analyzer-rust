use std::collections::HashMap;

use crate::{GenericTplId, LuaType, LuaTypeDeclId};

#[derive(Debug, Clone)]
pub struct TypeSubstitutor {
    tpl_replace_map: HashMap<GenericTplId, LuaType>,
    alias_type_id: Option<LuaTypeDeclId>,
}

impl TypeSubstitutor {
    pub fn new() -> Self {
        Self {
            tpl_replace_map: HashMap::new(),
            alias_type_id: None,
        }
    }

    pub fn from_type_array(type_array: Vec<LuaType>) -> Self {
        let mut tpl_replace_map = HashMap::new();
        for (i, ty) in type_array.into_iter().enumerate() {
            tpl_replace_map.insert(GenericTplId::Type(i as u32), ty);
        }
        Self {
            tpl_replace_map,
            alias_type_id: None,
        }
    }

    pub fn from_alias(type_array: Vec<LuaType>, alias_type_id: LuaTypeDeclId) -> Self {
        let mut tpl_replace_map = HashMap::new();
        for (i, ty) in type_array.into_iter().enumerate() {
            tpl_replace_map.insert(GenericTplId::Type(i as u32), ty);
        }
        Self {
            tpl_replace_map,
            alias_type_id: Some(alias_type_id),
        }
    }

    pub fn insert(&mut self, tpl_id: GenericTplId, replace_type: LuaType) {
        self.tpl_replace_map.insert(tpl_id, replace_type);
    }

    pub fn get(&self, tpl_id: GenericTplId) -> Option<&LuaType> {
        self.tpl_replace_map.get(&tpl_id)
    }

    pub fn check_recursion(&self, type_id: &LuaTypeDeclId) -> bool {
        if let Some(alias_type_id) = &self.alias_type_id {
            if alias_type_id == type_id {
                return true;
            }
        }
        
        false
    }
}
