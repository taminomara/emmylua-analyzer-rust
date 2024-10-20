use crate::{kind::LuaSyntaxKind, syntax::traits::LuaNode, LuaSyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaComment {
    syntax: LuaSyntaxNode,
}

impl LuaNode for LuaComment {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Comment
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::Comment.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}
