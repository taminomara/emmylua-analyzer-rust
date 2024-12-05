mod module_info;
mod module_node;
mod test;

use log::error;
use module_info::ModuleInfo;
use module_node::{ModuleNode, ModuleNodeId};
use regex::Regex;

use super::traits::LuaIndex;
use crate::{Emmyrc, FileId};
use std::{
    collections::HashMap,
    path::{Path, PathBuf}, sync::Arc,
};

#[derive(Debug)]
pub struct LuaModuleIndex {
    module_patterns: Vec<Regex>,
    module_root_id: ModuleNodeId,
    module_nodes: HashMap<ModuleNodeId, ModuleNode>,
    file_module_map: HashMap<FileId, ModuleInfo>,
    workspace_root: Vec<PathBuf>,
    id_counter: u32,
}

impl LuaModuleIndex {
    pub fn new() -> Self {
        let mut index = Self {
            module_patterns: Vec::new(),
            module_root_id: ModuleNodeId { id: 0 },
            module_nodes: HashMap::new(),
            file_module_map: HashMap::new(),
            workspace_root: Vec::new(),
            id_counter: 1,
        };

        let root_node = ModuleNode::default();
        index.module_nodes.insert(index.module_root_id, root_node);

        index
    }

    // patterns like "?.lua" and "?/init.lua"
    pub fn set_module_patterns(&mut self, patterns: Vec<String>) {
        let mut patterns = patterns;
        patterns.sort_by(|a, b| b.len().cmp(&a.len()));
        self.module_patterns.clear();
        for item in patterns {
            let regex_str = format!(
                "^{}$",
                regex::escape(&item.replace('\\', "/")).replace("\\?", "(.*)")
            );
            match Regex::new(&regex_str) {
                Ok(re) => self.module_patterns.push(re),
                Err(e) => {
                    error!("Invalid module pattern: {}, error: {}", item, e);
                    return;
                }
            };
        }
    }

    pub fn add_module_by_path(&mut self, file_id: FileId, path: &str) -> Option<()> {
        if self.file_module_map.contains_key(&file_id) {
            self.remove(file_id);
        }

        let module_path = self.extract_module_path(&path)?;
        let module_path = module_path.replace(['\\', '/'], ".");
        self.add_module_by_module_path(file_id, module_path)
    }

    pub fn add_module_by_module_path(
        &mut self,
        file_id: FileId,
        module_path: String,
    ) -> Option<()> {
        if self.file_module_map.contains_key(&file_id) {
            self.remove(file_id);
        }

        let module_parts: Vec<&str> = module_path.split('.').collect();
        if module_parts.is_empty() {
            return None;
        }

        let mut parent_node_id = self.module_root_id;
        for part in &module_parts {
            // I had to struggle with Rust's ownership rules, making the code look like this.
            let child_id = {
                let parent_node = self.module_nodes.get_mut(&parent_node_id).unwrap();
                let node_id = parent_node.children.get(*part);
                match node_id {
                    Some(id) => *id,
                    None => {
                        let new_id = ModuleNodeId {
                            id: self.id_counter,
                        };
                        parent_node.children.insert(part.to_string(), new_id);
                        new_id
                    }
                }
            };
            if !self.module_nodes.contains_key(&child_id) {
                let new_node = ModuleNode {
                    children: HashMap::new(),
                    file_ids: Vec::new(),
                    parent: Some(parent_node_id),
                };

                self.module_nodes.insert(child_id, new_node);
                self.id_counter += 1;
            }

            parent_node_id = child_id;
        }

        let node = self.module_nodes.get_mut(&parent_node_id).unwrap();
        node.file_ids.push(file_id);

        let module_info = ModuleInfo {
            file_id,
            full_module_name: module_parts.join("."),
            name: module_parts.last().unwrap().to_string(),
            module_id: parent_node_id,
            visible: true,
            export_type: None,
        };

        self.file_module_map.insert(file_id, module_info);

        Some(())
    }

    pub fn get_module(&self, file_id: FileId) -> Option<&ModuleInfo> {
        self.file_module_map.get(&file_id)
    }

    pub fn get_module_mut(&mut self, file_id: FileId) -> Option<&mut ModuleInfo> {
        self.file_module_map.get_mut(&file_id)
    }

    pub fn set_module_visibility(&mut self, file_id: FileId, visible: bool) {
        if let Some(module_info) = self.file_module_map.get_mut(&file_id) {
            module_info.visible = visible;
        }
    }

    pub fn find_module(&self, module_path: &str) -> Option<&ModuleInfo> {
        let module_path = module_path.replace(['\\', '/'], ".");
        let module_parts: Vec<&str> = module_path.split('.').collect();
        if module_parts.is_empty() {
            return None;
        }

        let mut parent_node_id = self.module_root_id;
        for part in &module_parts {
            let parent_node = self.module_nodes.get(&parent_node_id)?;
            let child_id = parent_node.children.get(*part)?;
            parent_node_id = *child_id;
        }

        let node = self.module_nodes.get(&parent_node_id)?;
        let file_id = node.file_ids.first()?;
        self.file_module_map.get(file_id)
    }

    /// Find a module node by module path.
    /// The module path is a string separated by dots.
    /// For example, "a.b.c" represents the module "c" in the module "b" in the module "a".
    pub fn find_module_node(&self, module_path: &str) -> Option<&ModuleNode> {
        let module_path = module_path.replace(['\\', '/'], ".");
        let module_parts: Vec<&str> = module_path.split('.').collect();
        if module_parts.is_empty() {
            return None;
        }

        let mut parent_node_id = self.module_root_id;
        for part in &module_parts {
            let parent_node = self.module_nodes.get(&parent_node_id)?;
            let child_id = parent_node.children.get(*part)?;
            parent_node_id = *child_id;
        }

        self.module_nodes.get(&parent_node_id)
    }

    fn extract_module_path(&self, path: &str) -> Option<String> {
        let path = Path::new(path);
        for root in &self.workspace_root {
            if let Ok(relative_path) = path.strip_prefix(root) {
                let relative_path_str = relative_path.to_str().unwrap_or("");
                let module_path = self.match_pattern(relative_path_str);
                return module_path;
            }
        }

        None
    }

    fn match_pattern(&self, path: &str) -> Option<String> {
        for pattern in &self.module_patterns {
            if let Some(captures) = pattern.captures(path) {
                if let Some(matched) = captures.get(1) {
                    return Some(matched.as_str().to_string());
                }
            }
        }

        None
    }

    pub fn add_workspace_root(&mut self, root: PathBuf) {
        self.workspace_root.push(root);
    }

    #[allow(unused)]
    pub fn remove_workspace_root(&mut self, root: &Path) {
        self.workspace_root.retain(|r| r != root);
    }

    pub fn update_config(&mut self, config: Arc<Emmyrc>) {
        let extension = if let Some(runtime) = &config.runtime {
            let mut extension_names = Vec::new();
            if let Some(extensions) = &runtime.extensions {
                for extension in extensions {
                    if extension.starts_with(".") {
                        extension_names.push(extension[1..].to_string());
                    } else if extension.starts_with("*.") {
                        extension_names.push(extension[2..].to_string());
                    } else {
                        extension_names.push(extension.clone());
                    }
                }
            }
            extension_names
        } else {
            vec!["lua".to_string()]
        };

        let mut patterns = Vec::new();
        for extension in extension {
            patterns.push(format!("?.{}", extension));
            patterns.push(format!("?/init.{}", extension));
        }

        self.set_module_patterns(patterns);
    }
}

impl LuaIndex for LuaModuleIndex {
    fn remove(&mut self, file_id: FileId) {
        let (mut parent_id, mut child_id) =
            if let Some(module_info) = self.file_module_map.remove(&file_id) {
                let module_id = module_info.module_id;
                let node = self.module_nodes.get_mut(&module_id).unwrap();
                node.file_ids.retain(|id| *id != file_id);
                if node.file_ids.is_empty() && node.children.is_empty() {
                    (node.parent, Some(module_id))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

        if parent_id.is_none() || child_id.is_none() {
            return;
        }

        while let Some(id) = parent_id {
            let child_module_id = child_id.unwrap();
            let node = self.module_nodes.get_mut(&id).unwrap();
            node.children
                .retain(|_, node_child_idid| *node_child_idid != child_module_id);

            if id == self.module_root_id {
                return;
            }

            if node.file_ids.is_empty() && node.children.is_empty() {
                child_id = Some(id);
                parent_id = node.parent;
                self.module_nodes.remove(&id);
            } else {
                break;
            }
        }
    }
}
