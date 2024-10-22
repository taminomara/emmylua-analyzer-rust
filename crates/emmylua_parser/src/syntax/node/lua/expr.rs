use crate::{kind::LuaSyntaxKind, syntax::traits::LuaAstNode, LuaSyntaxNode};


#[derive(Debug, Clone, PartialEq)]
pub enum LuaExpr {
    
}

impl LuaAstNode for LuaExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        unimplemented!()
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
