use std::collections::HashMap;

use crate::{GenericTplId, LuaType};

#[derive(Debug, Clone)]
pub struct TypeSubstitutor {
    tpl_replace_map: HashMap<GenericTplId, LuaType>,
}

impl TypeSubstitutor {
    pub fn new() -> Self {
        Self {
            tpl_replace_map: HashMap::new(),
        }
    }

    pub fn from_type_array(type_array: Vec<LuaType>) -> Self {
        let mut tpl_replace_map = HashMap::new();
        for (i, ty) in type_array.into_iter().enumerate() {
            tpl_replace_map.insert(GenericTplId::Type(i as u32), ty);
        }
        Self { tpl_replace_map }
    }

    pub fn insert(&mut self, tpl_id: GenericTplId, replace_type: LuaType) {
        self.tpl_replace_map.insert(tpl_id, replace_type);
    }

    pub fn get(&self, tpl_id: GenericTplId) -> Option<&LuaType> {
        self.tpl_replace_map.get(&tpl_id)
    }
}
