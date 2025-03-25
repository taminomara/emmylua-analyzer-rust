use std::sync::Arc;

use emmylua_parser::LuaExpr;

use crate::{LuaDeclId, LuaMemberId, LuaMemberKey, LuaMemberOwner, LuaSignatureId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferFailReason {
    None,
    RecursiveInfer,
    UnResolveExpr(LuaExpr),
    UnResolveSignatureReturn(LuaSignatureId),
    FieldDotFound(Arc<(LuaMemberOwner, LuaMemberKey)>),
    UnResolveDeclType(LuaDeclId),
    UnResolveMemberType(LuaMemberId),
}

impl InferFailReason {
    pub fn is_need_resolve(&self) -> bool {
        match self {
            InferFailReason::UnResolveExpr(_)
            | InferFailReason::UnResolveSignatureReturn(_)
            | InferFailReason::FieldDotFound(_)
            | InferFailReason::UnResolveDeclType(_)
            | InferFailReason::UnResolveMemberType(_) => true,
            _ => false,
        }
    }
}
