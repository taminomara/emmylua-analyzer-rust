mod lua;
mod doc;
mod token;

#[allow(unused)]
pub use lua::*;
#[allow(unused)]
pub use doc::*;
#[allow(unused)]
pub use token::*;


#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaNode {
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

    // lua struct other

    // comment
    LuaComment(LuaComment),


    // doc tag



    // doc type
}