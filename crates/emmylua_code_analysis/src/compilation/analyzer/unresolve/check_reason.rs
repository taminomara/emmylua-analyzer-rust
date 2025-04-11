use emmylua_parser::LuaAstNode;

use crate::{
    infer_expr, DbIndex, FileId, InFiled, InferFailReason, LuaDeclExtra, LuaDeclId,
    LuaDocParamInfo, LuaDocReturnInfo, LuaInferCache, LuaSemanticDeclId, LuaType, LuaTypeCache,
    SignatureReturnStatus,
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

            Some(
                db.get_type_index()
                    .get_type_cache(&decl_id.clone().into())
                    .is_some(),
            )
        }
        InferFailReason::UnResolveMemberType(member_id) => {
            let member = db.get_member_index().get_member(member_id)?;
            let key = member.get_key();
            let owner = db.get_member_index().get_current_owner(member_id)?;
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

pub fn resolve_all_reason<F>(
    db: &mut DbIndex,
    infer_manager: &mut InferCacheManager,
    unresolves: &mut Vec<UnResolve>,
    resolve_fn: F,
) where
    F: Fn(&mut DbIndex, &mut LuaInferCache, &mut InferFailReason) -> Option<()>,
{
    for unresolve in unresolves.iter_mut() {
        let file_id = unresolve.get_file_id().unwrap_or(FileId { id: 0 });
        let cache = infer_manager.get_infer_cache(file_id);
        match unresolve {
            UnResolve::Decl(un_resolve_decl) => {
                resolve_fn(db, cache, &mut un_resolve_decl.reason);
            }
            UnResolve::Member(ref mut un_resolve_member) => {
                resolve_fn(db, cache, &mut un_resolve_member.reason);
            }
            UnResolve::Module(un_resolve_module) => {
                resolve_fn(db, cache, &mut un_resolve_module.reason);
            }
            UnResolve::Return(un_resolve_return) => {
                resolve_fn(db, cache, &mut un_resolve_return.reason);
            }
            UnResolve::IterDecl(un_resolve_iter_var) => {
                resolve_fn(db, cache, &mut un_resolve_iter_var.reason);
            }
            _ => continue,
        };
    }
}

pub fn resolve_as_any(
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
            if decl.is_param() {
                return set_param_decl_type(db, decl_id, LuaType::Any);
            }

            db.get_type_index_mut().bind_type(
                decl_id.clone().into(),
                LuaTypeCache::InferType(LuaType::Any),
            );
        }
        InferFailReason::UnResolveMemberType(member_id) => {
            let member = db.get_member_index().get_member(member_id)?;
            let key = member.get_key();
            let owner = db.get_member_index().get_current_owner(&member_id)?;
            let member_item = db.get_member_index().get_member_item(&owner, key)?;
            let opt_type = member_item.resolve_type(db).ok();
            if opt_type.is_none() {
                let semantic_member_id = member_item.resolve_semantic_decl(db)?;
                if let LuaSemanticDeclId::Member(member_id) = semantic_member_id {
                    db.get_type_index_mut()
                        .bind_type(member_id.into(), LuaTypeCache::InferType(LuaType::Any));
                }
            }
        }
        InferFailReason::UnResolveExpr(expr) => {
            let file_id = cache.get_file_id();
            let key = InFiled::new(file_id, expr.get_syntax_id());
            db.get_type_index_mut()
                .bind_type(key.into(), LuaTypeCache::InferType(LuaType::Any));
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

fn set_param_decl_type(db: &mut DbIndex, decl_id: &LuaDeclId, typ: LuaType) -> Option<()> {
    let decl = db.get_decl_index_mut().get_decl_mut(decl_id)?;

    let (param_idx, signature_id) = match &decl.extra {
        LuaDeclExtra::Param {
            idx, signature_id, ..
        } => (*idx, *signature_id),
        _ => unreachable!(),
    };

    // find local annotation
    if let Some(signature) = db.get_signature_index_mut().get_mut(&signature_id) {
        if signature.param_docs.get(&param_idx).is_none() {
            let name = signature.params.get(param_idx)?;
            signature.param_docs.insert(
                param_idx,
                LuaDocParamInfo {
                    name: name.clone(),
                    type_ref: typ,
                    description: None,
                    nullable: false,
                },
            );
        }
    }
    Some(())
}
