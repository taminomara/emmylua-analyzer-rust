use std::ops::Deref;

use emmylua_parser::{LuaAstNode, LuaNameExpr};

use crate::{
    db_index::{DbIndex, LuaDeclOrMemberId, LuaMemberKey},
    LuaDecl, LuaDeclExtra, LuaFlowId, LuaInferCache, LuaMemberId, LuaType, TypeOps,
};

use super::InferResult;

pub fn infer_name_expr(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    name_expr: LuaNameExpr,
) -> InferResult {
    let name_token = name_expr.get_name_token()?;
    let name = name_token.get_name_text();
    match name {
        "self" => return infer_self(db, cache, name_expr),
        "_G" => return Some(LuaType::Global),
        _ => {}
    }

    let file_id = cache.get_file_id();
    let references_index = db.get_reference_index();
    let range = name_expr.get_range();
    let file_ref = references_index.get_local_reference(&file_id)?;
    let decl_id = file_ref.get_decl_id(&range);
    if let Some(decl_id) = decl_id {
        let decl = db.get_decl_index().get_decl(&decl_id)?;
        let opt_decl_type = get_decl_type(db, decl);
        let mut decl_type = if let Some(decl_type) = opt_decl_type {
            decl_type
        } else if cache.get_config().analysis_phase.is_force() {
            LuaType::Unknown
        } else {
            return None;
        };

        let flow_id = LuaFlowId::from_node(name_expr.syntax());
        let flow_chain = db.get_flow_index().get_flow_chain(file_id, flow_id);
        let root = name_expr.get_root();
        if let Some(flow_chain) = flow_chain {
            for type_assert in
                flow_chain.get_type_asserts(name, name_expr.get_position(), Some(decl_id.position))
            {
                decl_type = type_assert.tighten_type(db, cache, &root, decl_type)?;
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

fn get_decl_type(db: &DbIndex, decl: &LuaDecl) -> Option<LuaType> {
    if decl.is_global() {
        let name = decl.get_name();
        return db
            .get_decl_index()
            .get_global_decl_type(&LuaMemberKey::Name(name.into()));
    }

    if let Some(typ) = decl.get_type() {
        return Some(typ.clone());
    }

    if decl.is_param() {
        return infer_param(db, decl);
    }

    None
}

fn infer_self(db: &DbIndex, cache: &mut LuaInferCache, name_expr: LuaNameExpr) -> InferResult {
    let file_id = cache.get_file_id();
    let tree = db.get_decl_index().get_decl_tree(&file_id)?;
    let id = tree.find_self_decl(db, name_expr.clone())?;
    match id {
        LuaDeclOrMemberId::Decl(decl_id) => {
            let decl = db.get_decl_index().get_decl(&decl_id)?;
            let opt_decl_type = get_decl_type(db, decl);
            let mut decl_type = if let Some(decl_type) = opt_decl_type {
                decl_type
            } else if cache.get_config().analysis_phase.is_force() {
                LuaType::Unknown
            } else {
                return None;
            };

            if let LuaType::Ref(id) = decl_type {
                decl_type = LuaType::Def(id);
            }

            let flow_id = LuaFlowId::from_node(name_expr.syntax());
            let flow_chain = db.get_flow_index().get_flow_chain(file_id, flow_id);
            let root = name_expr.get_root();
            if let Some(flow_chain) = flow_chain {
                for type_assert in flow_chain.get_type_asserts(
                    "self",
                    name_expr.get_position(),
                    Some(decl_id.position),
                ) {
                    decl_type = type_assert.tighten_type(db, cache, &root, decl_type)?;
                }
            }

            Some(decl_type)
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

pub fn infer_param(db: &DbIndex, decl: &LuaDecl) -> InferResult {
    let (param_idx, signature_id, member_id) = match &decl.extra {
        LuaDeclExtra::Param {
            idx,
            signature_id,
            owner_member_id: closure_owner_syntax_id,
        } => (*idx, *signature_id, *closure_owner_syntax_id),
        _ => return None,
    };

    let mut colon_define = false;
    // find local annotation
    if let Some(signature) = db.get_signature_index().get(&signature_id) {
        colon_define = signature.is_colon_define;
        if let Some(param_info) = signature.get_param_info_by_id(param_idx) {
            let mut typ = param_info.type_ref.clone();
            if param_info.nullable && !typ.is_nullable() {
                typ = LuaType::Nullable(typ.into());
            }

            return Some(typ);
        }
    }

    let current_member_id = member_id?;
    let member_decl_type = find_decl_member_type(db, current_member_id)?;
    let param_type = find_param_type_from_type(db, member_decl_type, param_idx, colon_define);
    if let Some(param_type) = param_type {
        return Some(param_type);
    }

    // todo inherit from super
    None
}

fn find_decl_member_type(db: &DbIndex, member_id: LuaMemberId) -> Option<LuaType> {
    let member = db.get_member_index().get_member(&member_id)?;
    let key = member.get_key();
    let owner = member.get_owner();
    let member_item = db.get_member_index().get_member_item(&owner, key)?;
    member_item.resolve_type(db)
}

fn find_param_type_from_type(
    db: &DbIndex,
    source_type: LuaType,
    param_idx: usize,
    current_colon_define: bool,
) -> Option<LuaType> {
    match source_type {
        LuaType::Signature(signature_id) => {
            let signature = db.get_signature_index().get(&signature_id)?;
            let decl_colon_defined = signature.is_colon_define;
            let mut param_idx = param_idx;
            match (current_colon_define, decl_colon_defined) {
                (true, false) => {
                    param_idx += 1;
                }
                (false, true) => {
                    if param_idx > 0 {
                        param_idx -= 1;
                    }
                }
                _ => {}
            }

            if let Some(param_info) = signature.get_param_info_by_id(param_idx) {
                let mut typ = param_info.type_ref.clone();
                if param_info.nullable && !typ.is_nullable() {
                    typ = TypeOps::Union.apply(&typ, &LuaType::Nil);
                }

                return Some(typ);
            }
        }
        LuaType::DocFunction(f) => {
            let mut param_idx = param_idx;
            let decl_colon_defined = f.is_colon_define();
            match (current_colon_define, decl_colon_defined) {
                (true, false) => {
                    param_idx += 1;
                }
                (false, true) => {
                    if param_idx > 0 {
                        param_idx -= 1;
                    }
                }
                _ => {}
            }

            if let Some((_, typ)) = f.get_params().get(param_idx) {
                return typ.clone();
            }
        }
        LuaType::Nullable(base) => {
            return find_param_type_from_type(
                db,
                base.deref().clone(),
                param_idx,
                current_colon_define,
            );
        }
        LuaType::Union(union_types) => {
            for ty in union_types.get_types() {
                if let Some(ty) =
                    find_param_type_from_type(db, ty.clone(), param_idx, current_colon_define)
                {
                    return Some(ty);
                }
            }
        }
        _ => {}
    }

    None
}
