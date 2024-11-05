mod test;
mod type_decl;
mod types;

use std::collections::HashMap;

use flagset::FlagSet;
use rowan::TextRange;
use type_decl::LuaDeclLocation;
pub use type_decl::{LuaDeclTypeKind, LuaTypeAttribute, LuaTypeDecl, LuaTypeDeclId};
use crate::FileId;
use super::traits::LuaIndex;
pub use types::*;

#[derive(Debug)]
pub struct LuaTypeIndex {
    file_namespace: HashMap<FileId, String>,
    file_using_namespace: HashMap<FileId, Vec<String>>,
    file_types: HashMap<FileId, Vec<LuaTypeDeclId>>,
    full_name_type_map: HashMap<LuaTypeDeclId, LuaTypeDecl>,
}

impl LuaTypeIndex {
    pub fn new() -> Self {
        Self {
            file_namespace: HashMap::new(),
            file_using_namespace: HashMap::new(),
            file_types: HashMap::new(),
            full_name_type_map: HashMap::new(),
        }
    }

    pub fn add_file_namespace(&mut self, file_id: FileId, namespace: String) {
        self.file_namespace.insert(file_id, namespace);
    }

    pub fn get_file_namespace(&self, file_id: &FileId) -> Option<&String> {
        self.file_namespace.get(file_id)
    }

    pub fn add_file_using_namespace(&mut self, file_id: FileId, namespace: String) {
        self.file_using_namespace
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(namespace);
    }

    pub fn get_file_using_namespace(&self, file_id: &FileId) -> Option<&Vec<String>> {
        self.file_using_namespace.get(file_id)
    }

    pub fn add_type_decl(
        &mut self,
        file_id: FileId,
        range: TextRange,
        name: String,
        kind: LuaDeclTypeKind,
        attrib: Option<FlagSet<LuaTypeAttribute>>,
    ) {
        let basic_name = name;
        let ns = self.get_file_namespace(&file_id);
        let full_name = ns
            .map(|ns| format!("{}.{}", ns, basic_name))
            .unwrap_or(basic_name.to_string());
        let id = LuaTypeDeclId::new(&full_name);
        self.file_types
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(id.clone());

        if let Some(decls) = self.full_name_type_map.get_mut(&id) {
            let can_add = match (decls.get_attrib(), attrib) {
                (Some(a), Some(b)) => {
                    if a.contains(LuaTypeAttribute::Partial)
                        && b.contains(LuaTypeAttribute::Partial)
                    {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };

            if can_add {
                if let Some(decl_attrib) = &mut decls.attrib {
                    *decl_attrib |= attrib.unwrap();
                }

                let location = LuaDeclLocation { file_id, range };
                decls.defined_locations.push(location);
            }
        } else {
            let just_name = if let Some(i) = basic_name.rfind('.') {
                basic_name[i + 1..].to_string()
            } else {
                basic_name.clone()
            };

            let decl = LuaTypeDecl::new(file_id, range, just_name, kind, attrib, id.clone());
            self.full_name_type_map.insert(id, decl);
        }
    }

    #[allow(unused)]
    pub fn find_type_decl(&self, file_id: FileId, name: &str) -> Option<&LuaTypeDecl> {
        if let Some(ns) = self.get_file_namespace(&file_id) {
            let full_name = LuaTypeDeclId::new(&format!("{}.{}", ns, name));
            if let Some(decl) = self.full_name_type_map.get(&full_name) {
                return Some(decl);
            }
        }
        if let Some(usings) = self.get_file_using_namespace(&file_id) {
            for ns in usings {
                let full_name = LuaTypeDeclId::new(&format!("{}.{}", ns, name));
                if let Some(decl) = self.full_name_type_map.get(&full_name) {
                    return Some(decl);
                }
            }
        }

        let id = LuaTypeDeclId::new(name);
        self.full_name_type_map.get(&id)
    }
}

impl LuaIndex for LuaTypeIndex {
    fn remove(&mut self, file_id: FileId) {
        self.file_namespace.remove(&file_id);
        self.file_using_namespace.remove(&file_id);
        let name_list = self.file_types.remove(&file_id);
        if let Some(id_list) = name_list {
            for id in id_list {
                if let Some(decl) = self.full_name_type_map.get_mut(&id) {
                    decl.get_mut_locations()
                        .retain(|loc| loc.file_id != file_id);
                    if decl.get_mut_locations().is_empty() {
                        self.full_name_type_map.remove(&id);
                    }
                }
            }
        }
    }
}
