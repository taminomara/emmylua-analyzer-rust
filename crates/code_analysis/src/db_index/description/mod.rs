mod description;

use std::collections::HashMap;

pub use description::{LuaDescription, LuaDescriptionId, LuaDescriptionOwnerId};

use crate::FileId;

use super::traits::LuaIndex;



#[derive(Debug)]
pub struct LuaDescriptionIndex {
    descriptions: HashMap<LuaDescriptionId, LuaDescription>,
    id_count: u32,
    in_filed_descriptions: HashMap<FileId, LuaDescriptionId>,
    description_map: HashMap<LuaDescriptionOwnerId, LuaDescriptionId>,
}

impl LuaDescriptionIndex {
    pub fn new() -> Self {
        Self {
            descriptions: HashMap::new(),
            id_count: 0,
            in_filed_descriptions: HashMap::new(),
            description_map: HashMap::new(),
        }
    }

    pub fn add_description(
        &mut self,
        file_id: FileId,
        description: LuaDescription,
    ) -> LuaDescriptionId {
        let id = LuaDescriptionId::new(self.id_count);
        self.id_count += 1;
        let owner = description.owner_id.clone();
        self.description_map.insert(owner, id);
        self.in_filed_descriptions.insert(file_id, id);
        self.descriptions.insert(id, description);
        id
    }

    pub fn get_description(&self, id: &LuaDescriptionId) -> Option<&LuaDescription> {
        self.descriptions.get(id)
    }
}

impl LuaIndex for LuaDescriptionIndex {
    fn remove(&mut self, file_id: FileId) {
        if let Some(id) = self.in_filed_descriptions.remove(&file_id) {
            if let Some(description) = self.descriptions.remove(&id) {
                self.description_map.remove(&description.owner_id);
            }
        }
    }
}
