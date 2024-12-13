use emmylua_parser::{LuaAstNode, LuaExpr};

use crate::{DbIndex, LuaDeclId, LuaInferConfig, LuaMemberId, LuaPropertyOwnerId};

use super::{infer_expr, SemanticInfo};

pub fn get_expr_semantic_info(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    expr: LuaExpr,
) -> Option<SemanticInfo> {
    let typ = infer_expr(db, infer_config, expr.clone())?;
    let file_id = infer_config.get_file_id();
    let maybe_decl_id = LuaDeclId::new(file_id, expr.get_position());
    if let Some(_) = db.get_decl_index().get_decl(&maybe_decl_id) {
        return Some(SemanticInfo {
            typ,
            property_owner: Some(LuaPropertyOwnerId::LuaDecl(maybe_decl_id)),
        });
    };

    let member_id = LuaMemberId::new(expr.get_syntax_id(), file_id);
    if let Some(_) = db.get_member_index().get_member(&member_id) {
        return Some(SemanticInfo {
            typ,
            property_owner: Some(LuaPropertyOwnerId::Member(member_id)),
        });
    };

    Some(SemanticInfo {
        typ,
        property_owner: None,
    })
}
