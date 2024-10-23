use crate::{LuaAstNode, LuaSyntaxKind, LuaSyntaxNode};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaDocType {

}

impl LuaAstNode for LuaDocType {
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