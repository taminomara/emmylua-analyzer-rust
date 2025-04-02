use rowan::TextRange;

use crate::{
    db_index::{DbIndex, LuaMemberOwner, LuaType, LuaTypeDeclId},
    InFiled, LuaInferCache, LuaMemberId, LuaTypeCache, LuaTypeOwner,
};

use super::migrate_member::migrate_member_when_type_resolve;

pub fn merge_type_owner_expr_type(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    type_owner: LuaTypeOwner,
    expr_type: LuaType,
) -> Option<()> {
    let decl_type_cache = db.get_type_index().get_type_cache(&type_owner);

    if decl_type_cache.is_none() {
        db.get_type_index_mut()
            .bind_type(type_owner.clone(), LuaTypeCache::InferType(expr_type));
    } else {
        let decl_type = decl_type_cache.unwrap().as_type();
        merge_def_type(db, cache, decl_type.clone(), expr_type);
    }

    match &type_owner {
        LuaTypeOwner::Member(member_id) => {
            migrate_member_when_type_resolve(db, member_id.clone());
        }
        _ => {}
    }

    Some(())
}

fn merge_def_type(
    db: &mut DbIndex,
    cache: &mut LuaInferCache,
    decl_type: LuaType,
    expr_type: LuaType,
) {
    match &decl_type {
        LuaType::Def(def) => match &expr_type {
            LuaType::TableConst(in_filed_range) => {
                merge_def_type_with_table(db, cache, def.clone(), in_filed_range.clone());
            }
            LuaType::Instance(instance) => {
                let base_ref = instance.get_base();
                merge_def_type(db, cache, base_ref.clone(), expr_type);
            }
            _ => {}
        },
        _ => {}
    }
}

fn merge_def_type_with_table(
    db: &mut DbIndex,
    _: &mut LuaInferCache,
    def_id: LuaTypeDeclId,
    table_range: InFiled<TextRange>,
) -> Option<()> {
    let expr_member_owner = LuaMemberOwner::Element(table_range);
    let member_index = db.get_member_index_mut();
    let expr_member_ids = member_index
        .get_members(&expr_member_owner)?
        .iter()
        .map(|member| member.get_id())
        .collect::<Vec<_>>();
    let def_owner = LuaMemberOwner::Type(def_id);
    for table_member_id in expr_member_ids {
        set_owner_and_add_member(db, def_owner.clone(), table_member_id);
    }

    Some(())
}

pub fn set_owner_and_add_member(
    db: &mut DbIndex,
    owner: LuaMemberOwner,
    member_id: LuaMemberId,
) -> Option<()> {
    db.get_member_index_mut()
        .set_member_owner(owner.clone(), member_id.file_id, member_id);
    db.get_member_index_mut()
        .add_member_to_owner(owner.clone(), member_id);

    Some(())
}
