mod test;
mod type_decl;
mod types;
mod type_assert;
mod total_f64;

use std::collections::HashMap;

use super::traits::LuaIndex;
use crate::{FileId, InFiled};
use flagset::FlagSet;
use rowan::TextRange;
use type_decl::LuaDeclLocation;
pub use type_decl::{LuaDeclTypeKind, LuaTypeAttribute, LuaTypeDecl, LuaTypeDeclId};
pub use types::*;
pub use type_assert::TypeAssertion;

#[derive(Debug)]
pub struct LuaTypeIndex {
    file_namespace: HashMap<FileId, String>,
    file_using_namespace: HashMap<FileId, Vec<String>>,
    file_types: HashMap<FileId, Vec<LuaTypeDeclId>>,
    full_name_type_map: HashMap<LuaTypeDeclId, LuaTypeDecl>,
    generic_params: HashMap<LuaTypeDeclId, Vec<(String, Option<LuaType>)>>,
    supers: HashMap<LuaTypeDeclId, Vec<InFiled<LuaType>>>,
}

impl LuaTypeIndex {
    pub fn new() -> Self {
        Self {
            file_namespace: HashMap::new(),
            file_using_namespace: HashMap::new(),
            file_types: HashMap::new(),
            full_name_type_map: HashMap::new(),
            generic_params: HashMap::new(),
            supers: HashMap::new(),
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
    ) -> Result<(), String> {
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

            if !can_add {
                return Err(format!("Type {} already defined", full_name));
            }

            if let Some(decl_attrib) = &mut decls.attrib {
                *decl_attrib |= attrib.unwrap();
            }

            let location = LuaDeclLocation { file_id, range };
            decls.defined_locations.push(location);
        } else {
            let just_name = if let Some(i) = basic_name.rfind('.') {
                basic_name[i + 1..].to_string()
            } else {
                basic_name.clone()
            };

            let decl = LuaTypeDecl::new(file_id, range, just_name, kind, attrib, id.clone());
            self.full_name_type_map.insert(id, decl);
        }

        Ok(())
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

    pub fn add_generic_params(
        &mut self,
        decl_id: LuaTypeDeclId,
        params: Vec<(String, Option<LuaType>)>,
    ) {
        self.generic_params.insert(decl_id, params);
    }

    pub fn get_generic_params(
        &self,
        decl_id: &LuaTypeDeclId,
    ) -> Option<&Vec<(String, Option<LuaType>)>> {
        self.generic_params.get(decl_id)
    }

    pub fn add_super_type(&mut self, decl_id: LuaTypeDeclId, file_id: FileId, super_type: LuaType) {
        self.supers
            .entry(decl_id)
            .or_insert_with(Vec::new)
            .push(InFiled::new(file_id, super_type));
    }

    pub fn get_super_types(&self, decl_id: &LuaTypeDeclId) -> Option<Vec<LuaType>> {
        if let Some(supers) = self.supers.get(decl_id) {
            Some(supers.iter().map(|s| s.value.clone()).collect())
        } else {
            None
        }
    }

    pub fn get_type_decl(&self, decl_id: &LuaTypeDeclId) -> Option<&LuaTypeDecl> {
        self.full_name_type_map.get(decl_id)
    }
}

impl LuaIndex for LuaTypeIndex {
    fn remove(&mut self, file_id: FileId) {
        self.file_namespace.remove(&file_id);
        self.file_using_namespace.remove(&file_id);
        if let Some(type_id_list) = self.file_types.remove(&file_id) {
            for id in type_id_list {
                let mut remove_type = false;
                if let Some(decl) = self.full_name_type_map.get_mut(&id) {
                    decl.get_mut_locations()
                        .retain(|loc| loc.file_id != file_id);
                    if decl.get_mut_locations().is_empty() {
                        self.full_name_type_map.remove(&id);
                        remove_type = true;
                    }
                }

                if let Some(supers) = self.supers.get_mut(&id) {
                    supers.retain(|s| s.file_id != file_id);
                    if supers.is_empty() {
                        self.supers.remove(&id);
                    }
                }

                if remove_type {
                    self.generic_params.remove(&id);
                }
            }
        }
    }
}
