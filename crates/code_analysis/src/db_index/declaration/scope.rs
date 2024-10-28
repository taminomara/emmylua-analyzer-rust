use rowan::{TextRange, TextSize};

use crate::FileId;

use super::decl::LuaDeclId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct LuaScope {
    parent: Option<LuaScopeId>,
    children: Vec<ScopeOrDeclId>,
}

impl LuaScope {
    pub fn new(parent: Option<LuaScopeId>) -> Self {
        Self {
            parent,
            children: Vec::new(),
        }
    }

    pub fn add_decl(&mut self, decl: LuaDeclId) {
        self.children.push(ScopeOrDeclId::Decl(decl));
    }

    pub fn add_child(&mut self, child: LuaScopeId) {
        self.children.push(ScopeOrDeclId::Scope(child));
    }

    pub fn parent(&self) -> Option<LuaScopeId> {
        self.parent
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct LuaScopeId {
    pub file_id: FileId,
    pub id: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum ScopeOrDeclId {
    Scope(LuaScopeId),
    Decl(LuaDeclId),
}