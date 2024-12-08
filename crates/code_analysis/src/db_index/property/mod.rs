mod property;
mod version;

use std::collections::{HashMap, HashSet};

use emmylua_parser::VisibilityKind;
use property::LuaProperty;
pub use property::{LuaPropertyId, LuaPropertyOwnerId};
pub use version::{LuaVersionCond, LuaVersionCondOp};

use crate::FileId;

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct LuaPropertyIndex {
    properties: HashMap<LuaPropertyId, LuaProperty>,
    properties_map: HashMap<LuaPropertyOwnerId, LuaPropertyId>,

    id_count: u32,
    in_filed_descriptions: HashMap<FileId, HashSet<LuaPropertyId>>,
}

impl LuaPropertyIndex {
    pub fn new() -> Self {
        Self {
            id_count: 0,
            in_filed_descriptions: HashMap::new(),
            properties: HashMap::new(),
            properties_map: HashMap::new(),
        }
    }

    fn get_or_create_property(&mut self, owner_id: LuaPropertyOwnerId) -> &mut LuaProperty {
        if let Some(property_id) = self.properties_map.get(&owner_id) {
            self.properties.get_mut(property_id).unwrap()
        } else {
            let id = LuaPropertyId::new(self.id_count);
            self.id_count += 1;
            self.properties_map.insert(owner_id.clone(), id);
            self.properties
                .insert(id, LuaProperty::new(owner_id.clone(), id.clone()));
            self.properties.get_mut(&id).unwrap()
        }
    }

    pub fn add_description(
        &mut self,
        file_id: FileId,
        owner_id: LuaPropertyOwnerId,
        description: String,
    ) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.description = Some(Box::new(description));
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_visibility(
        &mut self,
        file_id: FileId,
        owner_id: LuaPropertyOwnerId,
        visibility: VisibilityKind,
    ) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.visibility = Some(visibility);
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_source(&mut self, file_id: FileId, owner_id: LuaPropertyOwnerId, source: String) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.source = Some(Box::new(source));
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_nodiscard(&mut self, file_id: FileId, owner_id: LuaPropertyOwnerId) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.is_nodiscard = true;
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_deprecated(
        &mut self,
        file_id: FileId,
        owner_id: LuaPropertyOwnerId,
        message: Option<String>,
    ) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.is_deprecated = true;
            property.deprecated_message = message.map(Box::new);
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_version(
        &mut self,
        file_id: FileId,
        owner_id: LuaPropertyOwnerId,
        version_conds: Vec<LuaVersionCond>,
    ) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.version_conds = Some(Box::new(version_conds));
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn add_async(&mut self, file_id: FileId, owner_id: LuaPropertyOwnerId) {
        let id = {
            let property = self.get_or_create_property(owner_id);
            property.is_async = true;
            property.id.clone()
        };
        self.in_filed_descriptions
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn get_property(&self, owner_id: LuaPropertyOwnerId) -> Option<&LuaProperty> {
        self.properties_map
            .get(&owner_id)
            .and_then(|id| self.properties.get(id))
    }
}

impl LuaIndex for LuaPropertyIndex {
    fn remove(&mut self, file_id: FileId) {
        if let Some(properties) = self.in_filed_descriptions.remove(&file_id) {
            for property_id in properties {
                let property = self.properties.remove(&property_id);
                if let Some(property) = property {
                    self.properties_map.remove(&property.owner);
                }
            }
        }
    }
}
