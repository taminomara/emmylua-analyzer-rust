use rowan::{TextRange, TextSize};

use crate::FileId;

use super::decl::LuaDeclId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum LuaScopeKind {
    Normal,
    Repeat,
    LocalStat,
    ForRange
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct LuaScope {
    parent: Option<LuaScopeId>,
    children: Vec<ScopeOrDeclId>,
    range: TextRange,
    kind: LuaScopeKind,
}

impl LuaScope {
    pub fn new(range: TextRange, kind: LuaScopeKind) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            range,
            kind,
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

    pub fn get_parent_scope(&self) -> Option<LuaScopeId> {
        self.parent
    }

    pub fn get_kind(&self) -> LuaScopeKind {
        self.kind
    }

    pub fn get_position(&self) -> TextSize {
        self.range.start()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct LuaScopeId {
    pub file_id: FileId,
    pub id: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ScopeOrDeclId {
    Scope(LuaScopeId),
    Decl(LuaDeclId),
}
