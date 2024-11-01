mod type_decl;
mod types;

use std::collections::HashMap;

pub use type_decl::{LuaTypeDecl, LuaTypeDeclId, LuaDeclTypeKind, LuaTypeAttribute};

use crate::FileId;

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct LuaTypeIndex {
    file_namespace: HashMap<FileId, String>,
    file_using_namespace: HashMap<FileId, Vec<String>>,
    file_types: HashMap<FileId, Vec<String>>,
    full_name_type_map: HashMap<String, LuaTypeDecl>,
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

    pub fn add_type_decl(&mut self, decl: LuaTypeDecl) {
        let basic_name = decl.get_name();
        let file_id = decl.get_file_id();
        let ns = self.get_file_namespace(&file_id);
        let full_name = ns
            .map(|ns| format!("{}.{}", ns, basic_name))
            .unwrap_or(basic_name.to_string());
        self.file_types
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(full_name.clone());
        self.full_name_type_map.insert(full_name, decl);
    }

    #[allow(unused)]
    pub fn find_type_decl(&self, id: LuaTypeDeclId) -> Option<&LuaTypeDecl> {
        if let Some(ns) = self.get_file_namespace(&id.file_id) {
            let full_name = format!("{}.{}", ns, id.name);
            if let Some(decl) = self.full_name_type_map.get(&full_name) {
                return Some(decl);
            }
        }
        if let Some(usings) = self.get_file_using_namespace(&id.file_id) {
            for ns in usings {
                let full_name = format!("{}.{}", ns, id.name);
                if let Some(decl) = self.full_name_type_map.get(&full_name) {
                    return Some(decl);
                }
            }
        }

        self.full_name_type_map.get(&id.name)
    }
}

impl LuaIndex for LuaTypeIndex {
    fn remove(&mut self, file_id: FileId) {
        self.file_namespace.remove(&file_id);
        self.file_using_namespace.remove(&file_id);
        let name_list = self.file_types.remove(&file_id);
        if let Some(name_list) = name_list {
            for name in name_list {
                self.full_name_type_map.remove(&name);
            }
        }
    }
}
