mod tree;
mod node;
mod traits;
mod comment_trait;

use rowan::{Language, TextRange};

use crate::kind::{LuaKind, LuaSyntaxKind, LuaTokenKind};
pub use tree::{LuaSyntaxTree, LuaTreeBuilder};
pub use node::*;
pub use traits::*;
pub use comment_trait::*;

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
    ptr: LuaSyntaxNodePtr,
}

impl LuaSyntaxId {
    pub fn from_ptr(ptr: LuaSyntaxNodePtr) -> Self {
        LuaSyntaxId {
            ptr
        }
    }

    pub fn from_node(node: &LuaSyntaxNode) -> Self {
        LuaSyntaxId {
            ptr: LuaSyntaxNodePtr::new(node)
        }
    }

    pub fn get_kind(&self) -> LuaSyntaxKind {
        self.ptr.kind().into()
    }

    pub fn get_range(&self) -> TextRange {
        self.ptr.text_range()
    }

    pub fn to_node(&self, tree: &LuaSyntaxTree) -> LuaSyntaxNode {
        self.ptr.to_node(tree.get_red_root())
    }
}