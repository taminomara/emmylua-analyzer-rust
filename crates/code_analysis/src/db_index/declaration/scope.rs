use rowan::TextRange;

use crate::FileId;

use super::decl::LuaDeclId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct LuaScope {
    parent: Option<LuaScopeId>,
    children: Vec<ScopeOrDeclId>,
    range: TextRange,
}

impl LuaScope {
    pub fn new(range: TextRange) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            range,
        }
    }

    pub fn add_decl(&mut self, decl: LuaDeclId) {
        self.children.push(ScopeOrDeclId::Decl(decl));
    }

    pub fn add_child(&mut self, child: LuaScopeId) {
        self.children.push(ScopeOrDeclId::Scope(child));
    }

    pub fn get_parent(&self) -> Option<LuaScopeId> {
        self.parent
    }

    pub(crate) fn set_parent(&mut self, parent: Option<LuaScopeId>) {
        self.parent = parent;
    }

    pub fn get_children(&self) -> &[ScopeOrDeclId] {
        &self.children
    }

    pub fn get_range(&self) -> TextRange {
        self.range
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