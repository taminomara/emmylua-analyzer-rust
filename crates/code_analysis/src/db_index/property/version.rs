use emmylua_parser::LuaVersionNumber;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaVersionCond {
    pub version: LuaVersionNumber,
    pub op: LuaVersionCondOp
}

impl LuaVersionCond {
    pub fn new(version: LuaVersionNumber, op: LuaVersionCondOp) -> Self {
        Self {
            version,
            op
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaVersionCondOp {
    Eq, // ==
    Gt, // >
    Lt  // <
}
