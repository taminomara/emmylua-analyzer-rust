mod comment_trait;
mod node;
mod traits;
mod tree;

use std::iter::successors;

use rowan::{Language, TextRange};

use crate::kind::{LuaKind, LuaSyntaxKind, LuaTokenKind};
pub use comment_trait::*;
pub use node::*;
pub use traits::*;
pub use tree::{LuaSyntaxTree, LuaTreeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaLanguage;

impl Language for LuaLanguage {
    type Kind = LuaKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        LuaKind::from_raw(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.get_raw())
    }
}

pub type LuaSyntaxNode = rowan::SyntaxNode<LuaLanguage>;
pub type LuaSyntaxToken = rowan::SyntaxToken<LuaLanguage>;
pub type LuaSyntaxElement = rowan::NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>;
pub type LuaSyntaxElementChildren = rowan::SyntaxElementChildren<LuaLanguage>;
pub type LuaSyntaxNodeChildren = rowan::SyntaxNodeChildren<LuaLanguage>;
pub type LuaSyntaxNodePtr = rowan::ast::SyntaxNodePtr<LuaLanguage>;

impl From<LuaSyntaxKind> for rowan::SyntaxKind {
    fn from(kind: LuaSyntaxKind) -> Self {
        let lua_kind = LuaKind::from(kind);
        rowan::SyntaxKind(lua_kind.get_raw())
    }
}

impl From<rowan::SyntaxKind> for LuaSyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        LuaKind::from_raw(kind.0).into()
    }
}

impl From<LuaTokenKind> for rowan::SyntaxKind {
    fn from(kind: LuaTokenKind) -> Self {
        let lua_kind = LuaKind::from(kind);
        rowan::SyntaxKind(lua_kind.get_raw())
    }
}

impl From<rowan::SyntaxKind> for LuaTokenKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        LuaKind::from_raw(kind.0).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LuaSyntaxId {
    kind: LuaKind,
    range: TextRange,
}

impl LuaSyntaxId {
    pub fn new(kind: LuaSyntaxKind, range: TextRange) -> Self {
        LuaSyntaxId {
            kind: kind.into(),
            range,
        }
    }
    
    pub fn from_ptr(ptr: LuaSyntaxNodePtr) -> Self {
        LuaSyntaxId {
            kind: ptr.kind().into(),
            range: ptr.text_range(),
        }
    }

    pub fn from_node(node: &LuaSyntaxNode) -> Self {
        LuaSyntaxId {
            kind: node.kind().into(),
            range: node.text_range(),
        }
    }

    pub fn get_kind(&self) -> LuaSyntaxKind {
        self.kind.into()
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn to_node(&self, tree: &LuaSyntaxTree) -> Option<LuaSyntaxNode> {
        let root = tree.get_red_root();
        if root.parent().is_some() {
            return None;
        }
        self.to_node_from_root(&root)
    }

    pub fn to_node_from_root(&self, root: &LuaSyntaxNode) -> Option<LuaSyntaxNode> {
        successors(Some(root.clone()), |node| {
            node.child_or_token_at_range(self.range)?.into_node()
        })
        .find(|it| it.text_range() == self.range && it.kind() == self.kind)
    }
}
