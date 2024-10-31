mod decl;
mod scope;

use std::collections::HashMap;

pub use decl::{LocalAttribute, LuaDecl, LuaDeclId};
use emmylua_parser::LuaSyntaxId;
use rowan::{TextRange, TextSize};
use scope::ScopeOrDeclId;
pub use scope::{LuaScope, LuaScopeId};

use crate::FileId;

#[allow(unused)]
#[derive(Debug)]
pub struct LuaDeclarationTree {
    file_id: FileId,
    decls: Vec<LuaDecl>,
    scopes: Vec<LuaScope>,
}

impl LuaDeclarationTree {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            decls: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn find_decl(&self, name: &str, position: TextSize) -> Option<&LuaDecl> {
        let mut scope = self.find_scope(position)?;

        loop {
            for decl_id in scope.get_children().iter().filter_map(|child| match child {
                ScopeOrDeclId::Decl(decl_id) => Some(decl_id),
                ScopeOrDeclId::Scope(_) => None,
            }) {
                if let Some(decl) = self.get_decl(*decl_id) {
                    if decl.get_position() <= position && decl.get_name() == name {
                        return Some(decl);
                    }
                }
            }

            scope = match scope.get_parent() {
                Some(parent_id) => self.scopes.get(parent_id.id as usize).unwrap(),
                None => break,
            };
        }

        None
    }

    fn find_scope(&self, position: TextSize) -> Option<&LuaScope> {
        if self.scopes.is_empty() {
            return None;
        }
        let mut scope = self.scopes.get(0).unwrap();
        loop {
            let child_scope = scope
                .get_children()
                .iter()
                .filter_map(|child| match child {
                    ScopeOrDeclId::Scope(child_id) => {
                        let child_scope = self.scopes.get(child_id.id as usize).unwrap();
                        Some(child_scope)
                    }
                    ScopeOrDeclId::Decl(_) => None,
                })
                .find(|child_scope| child_scope.get_range().contains(position));
            if child_scope.is_none() {
                break;
            }
            scope = child_scope.unwrap();
        }

        Some(scope)
    }

    pub fn add_decl(&mut self, decl: LuaDecl) -> LuaDeclId {
        let mut decl = decl;
        let id = self.decls.len() as u32;
        let decl_id = LuaDeclId {
            file_id: self.file_id,
            id,
        };

        decl.set_id(decl_id);

        self.decls.push(decl);
        decl_id
    }

    pub fn get_decl_mut(&mut self, decl_id: LuaDeclId) -> Option<&mut LuaDecl> {
        self.decls.get_mut(decl_id.id as usize)
    }

    pub fn get_decl(&self, decl_id: LuaDeclId) -> Option<&LuaDecl> {
        self.decls.get(decl_id.id as usize)
    }

    pub fn create_scope(&mut self, range: TextRange) -> LuaScopeId {
        let id = self.scopes.len() as u32;
        let scope_id = LuaScopeId {
            file_id: self.file_id,
            id,
        };

        let scope = LuaScope::new(range);
        self.scopes.push(scope);
        scope_id
    }

    pub fn add_decl_to_scope(&mut self, scope_id: LuaScopeId, decl_id: LuaDeclId) {
        if let Some(scope) = self.scopes.get_mut(scope_id.id as usize) {
            scope.add_decl(decl_id);
        }
    }

    pub fn add_child_scope(&mut self, parent_id: LuaScopeId, child_id: LuaScopeId) {
        if let Some(parent) = self.scopes.get_mut(parent_id.id as usize) {
            parent.add_child(child_id);
        }
        if let Some(child) = self.scopes.get_mut(child_id.id as usize) {
            child.set_parent(Some(parent_id));
        }
    }
}
