use crate::{
    compilation::analyzer::lua::analyze_return_point,
    db_index::{DbIndex, LuaMemberOwner, LuaType},
    semantic::{infer_expr, LuaInferCache},
    LuaSemanticDeclId, SignatureReturnStatus,
};

use super::{
    merge_decl_expr_type, merge_member_type, UnResolveDecl, UnResolveIterVar, UnResolveMember,
    UnResolveModule, UnResolveModuleRef, UnResolveReturn,
};

pub fn try_resolve_decl(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    decl: &UnResolveDecl,
) -> Option<bool> {
    let expr = decl.expr.clone();
    let expr_type = infer_expr(db, cache, expr).ok()?;
    let decl_id = decl.decl_id;
    let expr_type = match &expr_type {
        LuaType::MuliReturn(multi) => multi
            .get_type(decl.ret_idx)
            .cloned()
            .unwrap_or(LuaType::Unknown),
        _ => expr_type,
    };

    merge_decl_expr_type(db, cache, decl_id, expr_type);
    Some(true)
}

pub fn try_resolve_member(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    unresolve_member: &mut UnResolveMember,
) -> Option<bool> {
    if let Some(prefix_expr) = &unresolve_member.prefix {
        let prefix_type = infer_expr(db, cache, prefix_expr.clone()).ok()?;
        let member_owner = match prefix_type {
            LuaType::TableConst(in_file_range) => LuaMemberOwner::Element(in_file_range),
            LuaType::Def(def_id) => {
                let type_decl = db.get_type_index().get_type_decl(&def_id)?;
                // if is exact type, no need to extend field
                if type_decl.is_exact() {
                    return None;
                }
                LuaMemberOwner::Type(def_id)
            }
            LuaType::Instance(instance) => LuaMemberOwner::Element(instance.get_range().clone()),
            // is ref need extend field?
            _ => {
                return None;
            }
        };
        let member_id = unresolve_member.member_id.clone();
        db.get_member_index_mut().set_member_owner(
            member_owner.clone(),
            cache.get_file_id(),
            member_id,
        );
        db.get_member_index_mut()
            .add_member_to_owner(member_owner, member_id);
        unresolve_member.prefix = None;
    }

    if let Some(expr) = unresolve_member.expr.clone() {
        let expr_type = infer_expr(db, cache, expr).ok()?;
        let expr_type = match &expr_type {
            LuaType::MuliReturn(multi) => multi
                .get_type(unresolve_member.ret_idx)
                .cloned()
                .unwrap_or(LuaType::Unknown),
            _ => expr_type,
        };

        let member_id = unresolve_member.member_id;
        merge_member_type(db, cache, member_id, expr_type);
    }
    Some(true)
}

pub fn try_resolve_module(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    module: &UnResolveModule,
) -> Option<bool> {
    let expr = module.expr.clone();
    let expr_type = infer_expr(db, cache, expr).ok()?;
    let expr_type = match &expr_type {
        LuaType::MuliReturn(multi) => multi.get_type(0).cloned().unwrap_or(LuaType::Unknown),
        _ => expr_type,
    };
    let module_info = db.get_module_index_mut().get_module_mut(module.file_id)?;
    module_info.export_type = Some(expr_type);
    Some(true)
}

pub fn try_resolve_return_point(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    return_: &UnResolveReturn,
) -> Option<bool> {
    let return_docs = analyze_return_point(&db, cache, &return_.return_points).ok()?;
    let signature = db
        .get_signature_index_mut()
        .get_mut(&return_.signature_id)?;
    signature.resolve_return = SignatureReturnStatus::InferResolve;
    signature.return_docs = return_docs;
    Some(true)
}

pub fn try_resolve_iter_var(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    iter_var: &UnResolveIterVar,
) -> Option<bool> {
    let expr_type = infer_expr(db, cache, iter_var.iter_expr.clone()).ok()?;
    let func = match expr_type {
        LuaType::DocFunction(func) => func,
        _ => return Some(true),
    };

    let multi_return = func.get_multi_return();
    let iter_type = multi_return
        .get_type(iter_var.ret_idx)
        .unwrap_or(&LuaType::Nil);
    let decl_id = iter_var.decl_id;
    merge_decl_expr_type(db, cache, decl_id, iter_type.clone());
    Some(true)
}

pub fn try_resolve_module_ref(
    db: &mut DbIndex,
    _: &mut LuaInferCache,
    module_ref: &UnResolveModuleRef,
) -> Option<bool> {
    let module_index = db.get_module_index();
    let module = module_index.get_module(module_ref.module_file_id)?;
    let export_type = module.export_type.clone()?;
    match &module_ref.owner_id {
        LuaSemanticDeclId::LuaDecl(decl_id) => {
            let decl = db.get_decl_index_mut().get_decl_mut(decl_id)?;
            decl.set_decl_type(export_type);
        }
        LuaSemanticDeclId::Member(member_id) => {
            let member = db.get_member_index_mut().get_member_mut(member_id)?;
            member.set_decl_type(export_type);
        }
        _ => {}
    }

    Some(true)
}
