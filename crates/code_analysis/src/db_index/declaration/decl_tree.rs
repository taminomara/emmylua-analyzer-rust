use std::collections::HashMap;

use super::{decl, scope};
use crate::FileId;
use decl::{LuaDecl, LuaDeclId};
use rowan::{TextRange, TextSize};
use scope::{LuaScope, LuaScopeId, LuaScopeKind, ScopeOrDeclId};

#[derive(Debug)]
pub struct LuaDeclarationTree {
    file_id: FileId,
    decls: HashMap<LuaDeclId, LuaDecl>,
    scopes: Vec<LuaScope>,
}

impl LuaDeclarationTree {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            decls: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn find_local_decl(&self, name: &str, position: TextSize) -> Option<&LuaDecl> {
        let scope = self.find_scope(position)?;
        let mut result: Option<&LuaDecl> = None;
        self.walk_up(scope, position, 0, &mut |decl_id| {
            let decl = self.get_decl(decl_id).unwrap();
            if decl.get_name() == name {
                result = Some(decl);
                return true;
            }
            false
        });
        result
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

    fn base_walk_up<F>(&self, scope: &LuaScope, start_pos: TextSize, level: usize, f: &mut F)
    where
        F: FnMut(LuaDeclId) -> bool,
    {
        let cur_index = scope.get_children().iter().rposition(|child| match child {
            ScopeOrDeclId::Decl(decl_id) => {
                decl_id.position <= start_pos
            }
            ScopeOrDeclId::Scope(scope_id) => {
                let child_scope = self.scopes.get(scope_id.id as usize).unwrap();
                child_scope.get_position() <= start_pos
            }
        });

        if let Some(cur_index) = cur_index {
            for i in (0..=cur_index).rev() {
                match scope.get_children().get(i).unwrap() {
                    ScopeOrDeclId::Decl(decl_id) => {
                        if f(*decl_id) {
                            return;
                        }
                    }
                    ScopeOrDeclId::Scope(scope_id) => {
                        let child_scope = self.scopes.get(scope_id.id as usize).unwrap();
                        if self.walk_over(child_scope, f) {
                            return;
                        }
                    }
                }
            }
        }

        if let Some(parent_id) = scope.get_parent() {
            let parent_scope = self.scopes.get(parent_id.id as usize).unwrap();
            self.walk_up(parent_scope, start_pos, level + 1, f);
        }
    }

    /// Walks up the scope tree and calls `f` for each declaration.
    fn walk_up<F>(&self, scope: &LuaScope, start_pos: TextSize, level: usize, f: &mut F)
    where
        F: FnMut(LuaDeclId) -> bool,
    {
        match scope.get_kind() {
            LuaScopeKind::LocalStat => {
                let parent = scope.get_parent();
                if let Some(parent) = parent {
                    let parent_scope = self.scopes.get(parent.id as usize).unwrap();
                    self.walk_up(parent_scope, start_pos, level, f);
                }
            }
            LuaScopeKind::Repeat => {
                if level == 0 {
                    if let Some(ScopeOrDeclId::Scope(scope_id)) = scope.get_children().get(0) {
                        let scope = self.scopes.get(scope_id.id as usize).unwrap();
                        self.walk_up(scope, start_pos, level, f);
                        return;
                    }
                }

                self.base_walk_up(scope, start_pos, level, f);
            }
            LuaScopeKind::ForRange => {
                if level == 0 {
                    let parent = scope.get_parent();
                    if let Some(parent) = parent {
                        let parent_scope = self.scopes.get(parent.id as usize).unwrap();
                        self.walk_up(parent_scope, start_pos, level, f);
                    }
                } else {
                    self.base_walk_up(scope, start_pos, level, f);
                }
            }
            _ => {
                self.base_walk_up(scope, start_pos, level, f);
            }
        }
    }

    fn walk_over<F>(&self, scope: &LuaScope, f: &mut F) -> bool
    where
        F: FnMut(LuaDeclId) -> bool,
    {
        match scope.get_kind() {
            LuaScopeKind::LocalStat => {
                for child in scope.get_children() {
                    if let ScopeOrDeclId::Decl(decl_id) = child {
                        if f(*decl_id) {
                            return true;
                        }
                    }
                }
            }
            _ => {}
        }

        false
    }

    pub fn add_decl(&mut self, decl: LuaDecl) -> LuaDeclId {
        let decl_id = decl.get_id();
        self.decls.insert(decl_id, decl);
        decl_id
    }

    #[allow(unused)]
    pub fn get_decl_mut(&mut self, decl_id: LuaDeclId) -> Option<&mut LuaDecl> {
        self.decls.get_mut(&decl_id)
    }

    pub fn get_decl(&self, decl_id: LuaDeclId) -> Option<&LuaDecl> {
        self.decls.get(&decl_id)
    }

    pub fn create_scope(&mut self, range: TextRange, kind: LuaScopeKind) -> LuaScopeId {
        let id = self.scopes.len() as u32;
        let scope_id = LuaScopeId {
            file_id: self.file_id,
            id,
        };

        let scope = LuaScope::new(range, kind);
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
