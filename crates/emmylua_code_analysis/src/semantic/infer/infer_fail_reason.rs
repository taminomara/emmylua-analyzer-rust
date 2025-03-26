use emmylua_parser::LuaExpr;

use crate::{LuaDeclId, LuaMemberId, LuaSignatureId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferFailReason {
    None,
    RecursiveInfer,
    UnResolveExpr(LuaExpr),
    UnResolveSignatureReturn(LuaSignatureId),
    FieldDotFound,
    UnResolveDeclType(LuaDeclId),
    UnResolveMemberType(LuaMemberId),
}

impl InferFailReason {
    pub fn is_need_resolve(&self) -> bool {
        match self {
            InferFailReason::UnResolveExpr(_)
            | InferFailReason::UnResolveSignatureReturn(_)
            | InferFailReason::FieldDotFound
            | InferFailReason::UnResolveDeclType(_)
            | InferFailReason::UnResolveMemberType(_) => true,
            _ => false,
        }
    }
}
