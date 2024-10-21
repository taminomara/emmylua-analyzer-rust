use crate::{kind::LuaTokenKind, syntax::traits::LuaToken, LuaSyntaxNode};


#[derive(Debug, Clone, PartialEq, )]
pub struct LuaNameToken {
    node: LuaSyntaxNode,
}

impl LuaToken for LuaNameToken {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.node
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaTokenKind::Name
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaTokenKind::Name.into() {
            Some(Self { node: syntax })
        } else {
            None
        }
    }
}