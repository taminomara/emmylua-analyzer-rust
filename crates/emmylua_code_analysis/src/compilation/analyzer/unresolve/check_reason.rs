use emmylua_parser::LuaAstNode;

use crate::{
    infer_expr, DbIndex, FileId, InFiled, InferFailReason, LuaDocReturnInfo, LuaInferCache,
    LuaSemanticDeclId, LuaType, SignatureReturnStatus,
};

use super::{infer_manager::InferCacheManager, UnResolve};

pub fn check_reach_reason(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    reason: &InferFailReason,
) -> Option<bool> {
    match reason {
        InferFailReason::None
        | InferFailReason::FieldDotFound
        | InferFailReason::RecursiveInfer => Some(true),
        InferFailReason::UnResolveDeclType(decl_id) => {
            let decl = db.get_decl_index().get_decl(decl_id)?;
            if decl.is_param() {
                return Some(true);
            }

            Some(decl.get_type().is_some())
        }
        InferFailReason::UnResolveMemberType(member_id) => {
            let member = db.get_member_index().get_member(member_id)?;
            let key = member.get_key();
            let owner = member.get_owner();
            let member_item = db.get_member_index().get_member_item(&owner, key)?;
            Some(member_item.resolve_type(db).is_ok())
        }
        InferFailReason::UnResolveExpr(expr) => Some(infer_expr(db, cache, expr.clone()).is_ok()),
        InferFailReason::UnResolveSignatureReturn(signature_id) => {
            let signature = db.get_signature_index().get(signature_id)?;
            Some(signature.is_resolve_return())
        }
    }
}

pub fn resolve_all_reason(
    db: &mut DbIndex,
    infer_manager: &mut InferCacheManager,
    unresolves: &mut Vec<UnResolve>,
) {
    for unresolve in unresolves.iter_mut() {
        let file_id = unresolve.get_file_id().unwrap_or(FileId { id: 0 });
        let cache = infer_manager.get_infer_cache(file_id);
        match unresolve {
            UnResolve::Decl(un_resolve_decl) => {
                resolve_reason(db, cache, &mut un_resolve_decl.reason);
            }
            UnResolve::Member(ref mut un_resolve_member) => {
                resolve_reason(db, cache, &mut un_resolve_member.reason);
            }
            UnResolve::Module(un_resolve_module) => {
                resolve_reason(db, cache, &mut un_resolve_module.reason);
            }
            UnResolve::Return(un_resolve_return) => {
                resolve_reason(db, cache, &mut un_resolve_return.reason);
            }
            UnResolve::IterDecl(un_resolve_iter_var) => {
                resolve_reason(db, cache, &mut un_resolve_iter_var.reason);
            }
            _ => continue,
        };
    }
}

fn resolve_reason(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    reason: &mut InferFailReason,
) -> Option<()> {
    match reason {
        InferFailReason::None
        | InferFailReason::FieldDotFound
        | InferFailReason::RecursiveInfer => {}
        InferFailReason::UnResolveDeclType(decl_id) => {
            let decl = db.get_decl_index_mut().get_decl_mut(decl_id)?;
            if decl.get_type().is_none() {
                decl.set_decl_type(LuaType::Any);
            }
        }
        InferFailReason::UnResolveMemberType(member_id) => {
            let member = db.get_member_index().get_member(member_id)?;
            let key = member.get_key();
            let owner = member.get_owner();
            let member_item = db.get_member_index().get_member_item(&owner, key)?;
            let opt_type = member_item.resolve_type(db).ok();
            if opt_type.is_none() {
                let semantic_member_id = member_item.resolve_semantic_decl(db)?;
                if let LuaSemanticDeclId::Member(member_id) = semantic_member_id {
                    let member = db.get_member_index_mut().get_member_mut(&member_id)?;
                    if member.get_option_decl_type().is_none() {
                        member.set_decl_type(LuaType::Any);
                    }
                }
            }
        }
        InferFailReason::UnResolveExpr(expr) => {
            let file_id = cache.get_file_id();
            let key = InFiled::new(file_id, expr.get_syntax_id());
            db.get_type_index_mut().add_as_force_type(key, LuaType::Any);
        }
        InferFailReason::UnResolveSignatureReturn(signature_id) => {
            let signature = db.get_signature_index_mut().get_mut(signature_id)?;
            signature.return_docs = vec![LuaDocReturnInfo {
                name: None,
                type_ref: LuaType::Any,
                description: None,
            }];
            signature.resolve_return = SignatureReturnStatus::InferResolve;
        }
    }

    *reason = InferFailReason::None;
    Some(())
}
