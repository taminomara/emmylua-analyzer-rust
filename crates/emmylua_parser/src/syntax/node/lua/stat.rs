use crate::{
    kind::{LuaSyntaxKind, LuaTokenKind},
    syntax::{
        comment_trait::LuaCommentOwner,
        node::LuaNameToken,
        traits::{LuaAstChildren, LuaAstNode, LuaAstTokenChildren},
    },
    LuaSyntaxNode,
};

use super::{
    expr::{LuaCallExpr, LuaClosureExpr, LuaExpr, LuaVarExpr},
    LuaBlock, LuaLocalName,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaStat {
    LocalStat(LuaLocalStat),
    AssignStat(LuaAssignStat),
    CallExprStat(LuaCallExprStat),
    FuncStat(LuaFuncStat),
    LocalFuncStat(LuaLocalFuncStat),
    IfStat(LuaIfStat),
    WhileStat(LuaWhileStat),
    DoStat(LuaDoStat),
    ForStat(LuaForStat),
    ForRangeStat(LuaForRangeStat),
    RepeatStat(LuaRepeatStat),
    BreakStat(LuaBreakStat),
    ReturnStat(LuaReturnStat),
    GotoStat(LuaGotoStat),
    LabelStat(LuaLabelStat),
    EmptyStat(LuaEmptyStat),
}

impl LuaAstNode for LuaStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaStat::LocalStat(node) => node.syntax(),
            LuaStat::AssignStat(node) => node.syntax(),
            LuaStat::CallExprStat(node) => node.syntax(),
            LuaStat::FuncStat(node) => node.syntax(),
            LuaStat::LocalFuncStat(node) => node.syntax(),
            LuaStat::IfStat(node) => node.syntax(),
            LuaStat::WhileStat(node) => node.syntax(),
            LuaStat::DoStat(node) => node.syntax(),
            LuaStat::ForStat(node) => node.syntax(),
            LuaStat::ForRangeStat(node) => node.syntax(),
            LuaStat::RepeatStat(node) => node.syntax(),
            LuaStat::BreakStat(node) => node.syntax(),
            LuaStat::ReturnStat(node) => node.syntax(),
            LuaStat::GotoStat(node) => node.syntax(),
            LuaStat::LabelStat(node) => node.syntax(),
            LuaStat::EmptyStat(node) => node.syntax(),
        }
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            LuaSyntaxKind::LocalStat => true,
            LuaSyntaxKind::AssignStat => true,
            LuaSyntaxKind::CallExprStat => true,
            LuaSyntaxKind::FuncStat => true,
            LuaSyntaxKind::LocalFuncStat => true,
            LuaSyntaxKind::IfStat => true,
            LuaSyntaxKind::WhileStat => true,
            LuaSyntaxKind::DoStat => true,
            LuaSyntaxKind::ForStat => true,
            LuaSyntaxKind::ForRangeStat => true,
            LuaSyntaxKind::RepeatStat => true,
            LuaSyntaxKind::BreakStat => true,
            LuaSyntaxKind::ReturnStat => true,
            LuaSyntaxKind::GotoStat => true,
            LuaSyntaxKind::LabelStat => true,
            LuaSyntaxKind::EmptyStat => true,
            _ => false,
        }
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if LuaLocalStat::can_cast(syntax.kind().into()) {
            LuaLocalStat::cast(syntax).map(LuaStat::LocalStat)
        } else if LuaAssignStat::can_cast(syntax.kind().into()) {
            LuaAssignStat::cast(syntax).map(LuaStat::AssignStat)
        } else if LuaCallExprStat::can_cast(syntax.kind().into()) {
            LuaCallExprStat::cast(syntax).map(LuaStat::CallExprStat)
        } else if LuaFuncStat::can_cast(syntax.kind().into()) {
            LuaFuncStat::cast(syntax).map(LuaStat::FuncStat)
        } else if LuaLocalFuncStat::can_cast(syntax.kind().into()) {
            LuaLocalFuncStat::cast(syntax).map(LuaStat::LocalFuncStat)
        } else if LuaIfStat::can_cast(syntax.kind().into()) {
            LuaIfStat::cast(syntax).map(LuaStat::IfStat)
        } else if LuaWhileStat::can_cast(syntax.kind().into()) {
            LuaWhileStat::cast(syntax).map(LuaStat::WhileStat)
        } else if LuaDoStat::can_cast(syntax.kind().into()) {
            LuaDoStat::cast(syntax).map(LuaStat::DoStat)
        } else if LuaForStat::can_cast(syntax.kind().into()) {
            LuaForStat::cast(syntax).map(LuaStat::ForStat)
        } else if LuaForRangeStat::can_cast(syntax.kind().into()) {
            LuaForRangeStat::cast(syntax).map(LuaStat::ForRangeStat)
        } else if LuaRepeatStat::can_cast(syntax.kind().into()) {
            LuaRepeatStat::cast(syntax).map(LuaStat::RepeatStat)
        } else if LuaBreakStat::can_cast(syntax.kind().into()) {
            LuaBreakStat::cast(syntax).map(LuaStat::BreakStat)
        } else if LuaReturnStat::can_cast(syntax.kind().into()) {
            LuaReturnStat::cast(syntax).map(LuaStat::ReturnStat)
        } else if LuaGotoStat::can_cast(syntax.kind().into()) {
            LuaGotoStat::cast(syntax).map(LuaStat::GotoStat)
        } else if LuaLabelStat::can_cast(syntax.kind().into()) {
            LuaLabelStat::cast(syntax).map(LuaStat::LabelStat)
        } else if LuaEmptyStat::can_cast(syntax.kind().into()) {
            LuaEmptyStat::cast(syntax).map(LuaStat::EmptyStat)
        } else {
            None
        }
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
pub struct LuaLocalStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalStat {
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

impl LuaCommentOwner for LuaLocalStat {}

impl LuaLocalStat {
    #[allow(unused)]
    pub fn get_local_name_list(&self) -> LuaAstChildren<LuaLocalName> {
        self.children()
    }

    #[allow(unused)]
    pub fn get_value_exprs(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaAssignStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaAssignStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::AssignStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::AssignStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaAssignStat {}

impl LuaAssignStat {
    #[allow(unused)]
    pub fn get_expr_list(&self) -> (Vec<LuaVarExpr>, Vec<LuaExpr>) {
        let mut vars = Vec::new();
        let mut exprs = Vec::new();
        let mut meet_assign = false;
        for child in self.syntax.children_with_tokens() {
            if child.kind() == LuaTokenKind::TkAssign.into() {
                meet_assign = true;
            }

            if let Some(node) = child.into_node() {
                if meet_assign {
                    if let Some(var) = LuaVarExpr::cast(node) {
                        vars.push(var);
                    }
                } else {
                    if let Some(var) = LuaVarExpr::cast(node) {
                        vars.push(var);
                    }
                }
            }
        }

        (vars, exprs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaCallExprStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaCallExprStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::CallExprStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::CallExprStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaCallExprStat {}

impl LuaCallExprStat {
    #[allow(unused)]
    pub fn get_call_expr(&self) -> Option<LuaCallExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaFuncStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaFuncStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::FuncStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::FuncStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaFuncStat {}

impl LuaFuncStat {
    #[allow(unused)]
    pub fn get_func_name(&self) -> Option<LuaVarExpr> {
        self.child()
    }

    #[allow(unused)]
    pub fn get_closure(&self) -> Option<LuaClosureExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalFuncStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalFuncStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LocalFuncStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::LocalFuncStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaLocalFuncStat {}

impl LuaLocalFuncStat {
    #[allow(unused)]
    pub fn get_local_name(&self) -> Option<LuaLocalName> {
        self.child()
    }

    #[allow(unused)]
    pub fn get_closure(&self) -> Option<LuaClosureExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaIfStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaIfStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::IfStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::IfStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaIfStat {}

impl LuaIfStat {
    #[allow(unused)]
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }

    // #[allow(unused)]
    // pub fn get_else_if_clause_list(&self) -> LuaAstChildren<LuaElseIfClauseStat> {
    //     self.children()
    // }

    // #[allow(unused)]
    // pub fn get_else_clause(&self) -> Option<LuaElseClauseStat> {
    //     self.child()
    // }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaWhileStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaWhileStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::WhileStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::WhileStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaWhileStat {}

impl LuaWhileStat {
    #[allow(unused)]
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDoStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDoStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::DoStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::DoStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaDoStat {}

impl LuaDoStat {
    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaForStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaForStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ForStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::ForStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaForStat {}

impl LuaForStat {
    #[allow(unused)]
    pub fn get_var_name(&self) -> Option<LuaNameToken> {
        self.token()
    }

    #[allow(unused)]
    pub fn get_iter_expr(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaForRangeStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaForRangeStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ForRangeStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::ForRangeStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaForRangeStat {}

impl LuaForRangeStat {
    #[allow(unused)]
    pub fn get_var_name_list(&self) -> LuaAstTokenChildren<LuaNameToken> {
        self.tokens()
    }

    #[allow(unused)]
    pub fn get_expr_list(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaRepeatStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaRepeatStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::RepeatStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::RepeatStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaRepeatStat {}

impl LuaRepeatStat {
    #[allow(unused)]
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }

    #[allow(unused)]
    pub fn get_condition_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBreakStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaBreakStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::BreakStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::BreakStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaBreakStat {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaReturnStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaReturnStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ReturnStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::ReturnStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaReturnStat {}

impl LuaReturnStat {
    #[allow(unused)]
    pub fn get_expr_list(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaGotoStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaGotoStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::GotoStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::GotoStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaGotoStat {}

impl LuaGotoStat {
    #[allow(unused)]
    pub fn get_label_name(&self) -> Option<LuaNameToken> {
        self.token()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLabelStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLabelStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LabelStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::LabelStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaLabelStat {}

impl LuaLabelStat {
    #[allow(unused)]
    pub fn get_label_name(&self) -> Option<LuaNameToken> {
        self.token()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaEmptyStat {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaEmptyStat {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::EmptyStat
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::EmptyStat.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCommentOwner for LuaEmptyStat {}
