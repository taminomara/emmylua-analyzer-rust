use crate::{
    kind::LuaSyntaxKind,
    syntax::{
        comment_trait::LuaCommentOwner,
        traits::{LuaAstChildren, LuaAstNode},
    },
    LuaSyntaxNode,
};

use super::{expr::LuaExpr, LuaBlock, LuaLocalName};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaStat {}

impl LuaAstNode for LuaStat {
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
    #[allow(unused)]
    pub fn get_parent_block(&self) -> Option<LuaBlock> {
        LuaBlock::cast(self.syntax().parent()?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LocalStat {
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
    #[allow(unused)]
    pub fn get_local_name_list(&self) -> LuaAstChildren<LuaLocalName> {
        self.children()
    }

    #[allow(unused)]
    pub fn get_value_exprs(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }
}
