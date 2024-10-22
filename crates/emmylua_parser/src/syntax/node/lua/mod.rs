mod stat;
mod expr;

use stat::LuaStat;

use crate::{kind::LuaSyntaxKind, syntax::traits::{LuaAstNode, LuaAstChildren}, LuaSyntaxNode};

use super::LuaNameToken;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaChunk {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaChunk {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Chunk
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::Chunk.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaChunk {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBlock {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaBlock {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Block
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::Block.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaBlock {
    pub fn get_stats(&self) -> LuaAstChildren<LuaStat> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalName {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalName {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LocalName
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaLocalName {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }
}