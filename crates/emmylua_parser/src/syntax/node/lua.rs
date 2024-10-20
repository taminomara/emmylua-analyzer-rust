use crate::{kind::LuaSyntaxKind, syntax::traits::{LuaNode, LuaNodeChilren}, LuaSyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaChunk {
    syntax: LuaSyntaxNode,
}

impl LuaNode for LuaChunk {
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
    pub fn block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBlock {
    syntax: LuaSyntaxNode,
}

impl LuaNode for LuaBlock {
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
    pub fn stats(&self) -> LuaNodeChilren<LuaStat> {
        // self.children()
        todo!()
    }
}

pub enum LuaStat {
    // AssignStat(LuaAssignStat),
    // LocalAssignStat(LuaLocalAssignStat),
    // FuncCallStat(LuaFuncCallStat),
    // LabelStat(LuaLabelStat),
    // BreakStat(LuaBreakStat),
    // GotoStat(LuaGotoStat),
    // DoStat(LuaDoStat),
    // WhileStat(LuaWhileStat),
    // RepeatStat(LuaRepeatStat),
    // IfStat(LuaIfStat),
    // ForNumStat(LuaForNumStat),
    // ForInStat(LuaForInStat),
    // FuncDefStat(LuaFuncDefStat),
    // LocalFuncDefStat(LuaLocalFuncDefStat),
}

