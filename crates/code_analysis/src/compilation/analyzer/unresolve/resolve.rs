use crate::{
    db_index::DbIndex,
    semantic::{infer_expr, LuaInferConfig},
};

use super::UnResolveDecl;

pub fn try_resolve_decl(
    db: &mut DbIndex,
    config: &mut LuaInferConfig,
    decl: &UnResolveDecl,
) -> Option<bool> {
    // infer_expr(db, config, expr);
    Some(true)
}
