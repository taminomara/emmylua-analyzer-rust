mod gen;
mod lua_syntax_tree;

use rowan::Language;

use crate::kind::LuaKind;
pub use lua_syntax_tree::LuaSyntaxTree;

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

pub type LuSyntaxNode = rowan::SyntaxNode<LuaLanguage>;
pub type LuSyntaxToken = rowan::SyntaxToken<LuaLanguage>;
pub type LuSyntaxElement = rowan::NodeOrToken<LuSyntaxNode, LuSyntaxToken>;
pub type LuSyntaxElementChildren = rowan::SyntaxElementChildren<LuaLanguage>;
