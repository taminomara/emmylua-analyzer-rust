use emmylua_parser::{LuaAstNode, LuaNameExpr};

use crate::db_index::{DbIndex, LuaDeclOrMemberId, LuaMemberKey};

use super::{InferResult, LuaInferConfig};

pub fn infer_name_expr(
    db: &DbIndex,
    config: &LuaInferConfig,
    name_expr: LuaNameExpr,
) -> InferResult {
    let name_token = name_expr.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    if name == "self" {
        return infer_self(db, config, name_expr);
    }

    let file_id = config.get_file_id();
    let references_index = db.get_reference_index();
    let range = name_expr.get_range();
    let file_ref = references_index.get_local_reference(&file_id)?;
    let decl_id = file_ref.get_decl_id(&range);
    if let Some(decl_id) = decl_id {
        let decl = db.get_decl_index().get_decl(&decl_id)?;
        let mut decl_type = if decl.is_global() {
            db.get_decl_index()
                .get_global_decl_type(&LuaMemberKey::Name(name.into()))?
                .clone()
        } else {
            decl.get_type()?.clone()
        };
        let flow_chain = db.get_flow_index().get_flow_chain(file_id, decl_id);
        if let Some(flow_chain) = flow_chain {
            for type_assert in flow_chain.get_type_asserts(name_expr.get_position()) {
                decl_type = type_assert.tighten_type(decl_type);
            }
        }

        Some(decl_type)
    } else {
        let decl_type = db
            .get_decl_index()
            .get_global_decl_type(&LuaMemberKey::Name(name.into()))?
            .clone();
        Some(decl_type)
    }
}

fn infer_self(db: &DbIndex, config: &LuaInferConfig, name_expr: LuaNameExpr) -> InferResult {
    let name = name_expr.get_name_token()?.get_name_text().to_string();
    let file_id = config.get_file_id();
    let tree = db.get_decl_index().get_decl_tree(&file_id)?;
    let id = tree.find_self_decl(db, name_expr)?;
    match id {
        LuaDeclOrMemberId::Decl(decl_id) => {
            let decl = db.get_decl_index().get_decl(&decl_id)?;
            if decl.is_global() {
                return Some(
                    db.get_decl_index()
                        .get_global_decl_type(&name.into())?
                        .clone(),
                );
            }
            Some(decl.get_type()?.clone())
        }
        LuaDeclOrMemberId::Member(member_id) => {
            let member = db.get_member_index().get_member(&member_id)?;
            let ty = member.get_decl_type();
            if ty.is_unknown() {
                None
            } else {
                Some(ty.clone())
            }
        }
    }
}
