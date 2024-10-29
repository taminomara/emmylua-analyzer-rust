mod scope;
mod decl;

pub use decl::{LuaDecl, LuaDeclId};
use emmylua_parser::LuaSyntaxNodePtr;
use rowan::{TextRange, TextSize};
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
        None
    }

    pub fn create_decl(&mut self, name: String, position: TextSize) -> LuaDeclId {
        let id = self.decls.len() as u32;
        let decl_id = LuaDeclId {
            file_id: self.file_id,
            id,
        };

        let decl = LuaDecl::new(name, decl_id, position);
        self.decls.push(decl);
        decl_id
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