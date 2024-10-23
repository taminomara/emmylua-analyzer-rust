mod doc;
mod lua;
mod token;
mod test;

#[allow(unused)]
pub use doc::*;
#[allow(unused)]
pub use lua::*;
#[allow(unused)]
pub use token::*;

use crate::kind::LuaSyntaxKind;

use super::{traits::LuaAstNode, LuaSyntaxNode};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaAst {
    LuaChunk(LuaChunk),
    LuaBlock(LuaBlock),
    // stats
    LuaAssignStat(LuaAssignStat),
    LuaLocalStat(LuaLocalStat),
    LuaCallExprStat(LuaCallExprStat),
    LuaLabelStat(LuaLabelStat),
    LuaBreakStat(LuaBreakStat),
    LuaGotoStat(LuaGotoStat),
    LuaDoStat(LuaDoStat),
    LuaWhileStat(LuaWhileStat),
    LuaRepeatStat(LuaRepeatStat),
    LuaIfStat(LuaIfStat),
    LuaForStat(LuaForStat),
    LuaForRangeStat(LuaForRangeStat),
    LuaFuncStat(LuaFuncStat),
    LuaLocalFuncStat(LuaLocalFuncStat),
    LuaReturnStat(LuaReturnStat),

    // exprs
    LuaNameExpr(LuaNameExpr),
    LuaIndexExpr(LuaIndexExpr),
    LuaTableExpr(LuaTableExpr),
    LuaBinaryExpr(LuaBinaryExpr),
    LuaUnaryExpr(LuaUnaryExpr),
    LuaParenExpr(LuaParenExpr),
    LuaCallExpr(LuaCallExpr),
    LuaLiteralExpr(LuaLiteralExpr),
    LuaClosureExpr(LuaClosureExpr),

    // lua structure other
    LuaParamList(LuaParamList),
    LuaCallArgList(LuaCallArgList),
    LuaLocalName(LuaLocalName),
    LuaTableField(LuaTableField),
    LuaParamName(LuaParamName),
    LuaLocalAttribute(LuaLocalAttribute),

    // comment
    LuaComment(LuaComment),
    // doc tag

    // doc type
}

impl LuaAstNode for LuaAst {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaAst::LuaChunk(node) => node.syntax(),
            LuaAst::LuaBlock(node) => node.syntax(),
            LuaAst::LuaAssignStat(node) => node.syntax(),
            LuaAst::LuaLocalStat(node) => node.syntax(),
            LuaAst::LuaCallExprStat(node) => node.syntax(),
            LuaAst::LuaLabelStat(node) => node.syntax(),
            LuaAst::LuaBreakStat(node) => node.syntax(),
            LuaAst::LuaGotoStat(node) => node.syntax(),
            LuaAst::LuaDoStat(node) => node.syntax(),
            LuaAst::LuaWhileStat(node) => node.syntax(),
            LuaAst::LuaRepeatStat(node) => node.syntax(),
            LuaAst::LuaIfStat(node) => node.syntax(),
            LuaAst::LuaForStat(node) => node.syntax(),
            LuaAst::LuaForRangeStat(node) => node.syntax(),
            LuaAst::LuaFuncStat(node) => node.syntax(),
            LuaAst::LuaLocalFuncStat(node) => node.syntax(),
            LuaAst::LuaReturnStat(node) => node.syntax(),
            LuaAst::LuaNameExpr(node) => node.syntax(),
            LuaAst::LuaIndexExpr(node) => node.syntax(),
            LuaAst::LuaTableExpr(node) => node.syntax(),
            LuaAst::LuaBinaryExpr(node) => node.syntax(),
            LuaAst::LuaUnaryExpr(node) => node.syntax(),
            LuaAst::LuaParenExpr(node) => node.syntax(),
            LuaAst::LuaCallExpr(node) => node.syntax(),
            LuaAst::LuaLiteralExpr(node) => node.syntax(),
            LuaAst::LuaClosureExpr(node) => node.syntax(),
            LuaAst::LuaParamList(node) => node.syntax(),
            LuaAst::LuaCallArgList(node) => node.syntax(),
            LuaAst::LuaLocalName(node) => node.syntax(),
            LuaAst::LuaTableField(node) => node.syntax(),
            LuaAst::LuaParamName(node) => node.syntax(),
            LuaAst::LuaLocalAttribute(node) => node.syntax(),
            LuaAst::LuaComment(node) => node.syntax(),
        }
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            LuaSyntaxKind::Chunk => true,
            LuaSyntaxKind::Block => true,
            LuaSyntaxKind::AssignStat => true,
            LuaSyntaxKind::LocalStat => true,
            LuaSyntaxKind::CallExprStat => true,
            LuaSyntaxKind::LabelStat => true,
            LuaSyntaxKind::BreakStat => true,
            LuaSyntaxKind::GotoStat => true,
            LuaSyntaxKind::DoStat => true,
            LuaSyntaxKind::WhileStat => true,
            LuaSyntaxKind::RepeatStat => true,
            LuaSyntaxKind::IfStat => true,
            LuaSyntaxKind::ForStat => true,
            LuaSyntaxKind::ForRangeStat => true,
            LuaSyntaxKind::FuncStat => true,
            LuaSyntaxKind::LocalFuncStat => true,
            LuaSyntaxKind::ReturnStat => true,
            LuaSyntaxKind::NameExpr => true,
            LuaSyntaxKind::IndexExpr => true,
            LuaSyntaxKind::TableEmptyExpr
            | LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr => true,
            LuaSyntaxKind::BinaryExpr => true,
            LuaSyntaxKind::UnaryExpr => true,
            LuaSyntaxKind::ParenExpr => true,
            LuaSyntaxKind::CallExpr => true,
            LuaSyntaxKind::LiteralExpr => true,
            LuaSyntaxKind::ClosureExpr => true,
            LuaSyntaxKind::ParamList => true,
            LuaSyntaxKind::CallArgList => true,
            LuaSyntaxKind::LocalName => true,
            LuaSyntaxKind::TableFieldAssign | LuaSyntaxKind::TableFieldValue => true,
            LuaSyntaxKind::ParamName => true,
            LuaSyntaxKind::Attribute => true,
            LuaSyntaxKind::Comment => true,
            _ => false,
        }
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            LuaSyntaxKind::Chunk => LuaChunk::cast(syntax).map(LuaAst::LuaChunk),
            LuaSyntaxKind::Block => LuaBlock::cast(syntax).map(LuaAst::LuaBlock),
            LuaSyntaxKind::AssignStat => LuaAssignStat::cast(syntax).map(LuaAst::LuaAssignStat),
            LuaSyntaxKind::LocalStat => LuaLocalStat::cast(syntax).map(LuaAst::LuaLocalStat),
            LuaSyntaxKind::CallExprStat => {
                LuaCallExprStat::cast(syntax).map(LuaAst::LuaCallExprStat)
            }
            LuaSyntaxKind::LabelStat => LuaLabelStat::cast(syntax).map(LuaAst::LuaLabelStat),
            LuaSyntaxKind::BreakStat => LuaBreakStat::cast(syntax).map(LuaAst::LuaBreakStat),
            LuaSyntaxKind::GotoStat => LuaGotoStat::cast(syntax).map(LuaAst::LuaGotoStat),
            LuaSyntaxKind::DoStat => LuaDoStat::cast(syntax).map(LuaAst::LuaDoStat),
            LuaSyntaxKind::WhileStat => LuaWhileStat::cast(syntax).map(LuaAst::LuaWhileStat),
            LuaSyntaxKind::RepeatStat => LuaRepeatStat::cast(syntax).map(LuaAst::LuaRepeatStat),
            LuaSyntaxKind::IfStat => LuaIfStat::cast(syntax).map(LuaAst::LuaIfStat),
            LuaSyntaxKind::ForStat => LuaForStat::cast(syntax).map(LuaAst::LuaForStat),
            LuaSyntaxKind::ForRangeStat => {
                LuaForRangeStat::cast(syntax).map(LuaAst::LuaForRangeStat)
            }
            LuaSyntaxKind::FuncStat => LuaFuncStat::cast(syntax).map(LuaAst::LuaFuncStat),
            LuaSyntaxKind::LocalFuncStat => {
                LuaLocalFuncStat::cast(syntax).map(LuaAst::LuaLocalFuncStat)
            }
            LuaSyntaxKind::ReturnStat => LuaReturnStat::cast(syntax).map(LuaAst::LuaReturnStat),
            LuaSyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaAst::LuaNameExpr),
            LuaSyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaAst::LuaIndexExpr),
            LuaSyntaxKind::TableEmptyExpr
            | LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr => {
                LuaTableExpr::cast(syntax).map(LuaAst::LuaTableExpr)
            }
            LuaSyntaxKind::BinaryExpr => LuaBinaryExpr::cast(syntax).map(LuaAst::LuaBinaryExpr),
            LuaSyntaxKind::UnaryExpr => LuaUnaryExpr::cast(syntax).map(LuaAst::LuaUnaryExpr),
            LuaSyntaxKind::ParenExpr => LuaParenExpr::cast(syntax).map(LuaAst::LuaParenExpr),
            LuaSyntaxKind::CallExpr => LuaCallExpr::cast(syntax).map(LuaAst::LuaCallExpr),
            LuaSyntaxKind::LiteralExpr => LuaLiteralExpr::cast(syntax).map(LuaAst::LuaLiteralExpr),
            LuaSyntaxKind::ClosureExpr => LuaClosureExpr::cast(syntax).map(LuaAst::LuaClosureExpr),
            LuaSyntaxKind::ParamList => LuaParamList::cast(syntax).map(LuaAst::LuaParamList),
            LuaSyntaxKind::CallArgList => LuaCallArgList::cast(syntax).map(LuaAst::LuaCallArgList),
            LuaSyntaxKind::LocalName => LuaLocalName::cast(syntax).map(LuaAst::LuaLocalName),
            LuaSyntaxKind::TableFieldAssign | LuaSyntaxKind::TableFieldValue => {
                LuaTableField::cast(syntax).map(LuaAst::LuaTableField)
            }
            LuaSyntaxKind::ParamName => LuaParamName::cast(syntax).map(LuaAst::LuaParamName),
            LuaSyntaxKind::Attribute => {
                LuaLocalAttribute::cast(syntax).map(LuaAst::LuaLocalAttribute)
            }
            LuaSyntaxKind::Comment => LuaComment::cast(syntax).map(LuaAst::LuaComment),
            _ => None,
        }
    }
}
