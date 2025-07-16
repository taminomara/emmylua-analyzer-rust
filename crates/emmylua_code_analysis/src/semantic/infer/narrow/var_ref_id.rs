use std::ops::Deref;

use emmylua_parser::{LuaAstNode, LuaExpr};
use internment::ArcIntern;
use rowan::TextSize;
use smol_str::SmolStr;

use crate::{
    semantic::infer::{
        infer_index::get_index_expr_var_ref_id, infer_name::get_name_expr_var_ref_id,
    },
    DbIndex, LuaDeclId, LuaDeclOrMemberId, LuaInferCache, LuaMemberId,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarRefId {
    VarRef(LuaDeclId),
    SelfRef(LuaDeclOrMemberId),
    IndexRef(LuaDeclOrMemberId, ArcIntern<SmolStr>),
}

impl VarRefId {
    pub fn get_decl_id_ref(&self) -> Option<LuaDeclId> {
        match self {
            VarRefId::VarRef(decl_id) => Some(*decl_id),
            VarRefId::SelfRef(decl_or_member_id) => decl_or_member_id.as_decl_id(),
            _ => None,
        }
    }

    pub fn get_member_id_ref(&self) -> Option<LuaMemberId> {
        match self {
            VarRefId::SelfRef(decl_or_member_id) => decl_or_member_id.as_member_id(),
            _ => None,
        }
    }

    pub fn get_position(&self) -> TextSize {
        match self {
            VarRefId::VarRef(decl_id) => decl_id.position,
            VarRefId::SelfRef(decl_or_member_id) => decl_or_member_id.get_position(),
            VarRefId::IndexRef(decl_or_member_id, _) => decl_or_member_id.get_position(),
        }
    }

    pub fn start_with(&self, prefix: &VarRefId) -> bool {
        let (decl_or_member_id, path) = match self {
            VarRefId::IndexRef(decl_or_member_id, path) => {
                (decl_or_member_id.clone(), path.clone())
            }
            _ => return false,
        };

        match prefix {
            VarRefId::VarRef(decl_id) => decl_or_member_id.as_decl_id() == Some(*decl_id),
            VarRefId::SelfRef(decl_or_member_id) => decl_or_member_id == decl_or_member_id,
            VarRefId::IndexRef(decl_or_member_id, prefix_path) => {
                decl_or_member_id == decl_or_member_id
                    && path.starts_with(prefix_path.deref().as_str())
            }
        }
    }
}

pub fn get_var_expr_var_ref_id(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    var_expr: LuaExpr,
) -> Option<VarRefId> {
    if let Some(var_ref_id) = cache.expr_var_ref_id_cache.get(&var_expr.get_syntax_id()) {
        return Some(var_ref_id.clone());
    }

    let ref_id = match &var_expr {
        LuaExpr::NameExpr(name_expr) => get_name_expr_var_ref_id(db, cache, name_expr),
        LuaExpr::IndexExpr(index_expr) => get_index_expr_var_ref_id(db, cache, index_expr),
        _ => None,
    }?;

    cache
        .expr_var_ref_id_cache
        .insert(var_expr.get_syntax_id(), ref_id.clone());
    Some(ref_id)
}
