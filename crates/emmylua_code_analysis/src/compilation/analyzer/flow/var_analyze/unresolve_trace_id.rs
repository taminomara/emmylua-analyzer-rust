use emmylua_parser::{LuaBlock, LuaExpr};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum UnResolveTraceId {
    Expr(LuaExpr),
    OutsideBlock(LuaBlock),
}
