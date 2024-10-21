use crate::{kind::LuaSyntaxKind, syntax::traits::{LuaCommentOwner, LuaNode, LuaNodeChildren}, LuaSyntaxNode};

use super::{LuaBlock, LuaLocalName};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaStat {

}

impl LuaNode for LuaStat {
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

impl LuaCommentOwner for LuaStat {}

impl LuaStat {
    pub fn get_parent_block(&self) -> Option<LuaBlock> {
        LuaBlock::cast(self.syntax().parent()?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalStat {
    syntax: LuaSyntaxNode,
}

impl LuaNode for LocalStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LocalStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::LocalStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LocalStat {}

impl LocalStat {
    pub fn get_local_name_list(&self) -> LuaNodeChildren<LuaLocalName> {
        self.children()
    }
}