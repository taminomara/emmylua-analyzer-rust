use smol_str::SmolStr;

use crate::LuaDeclId;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum VarRefId {
    DeclId(LuaDeclId),
    Name(SmolStr)
}