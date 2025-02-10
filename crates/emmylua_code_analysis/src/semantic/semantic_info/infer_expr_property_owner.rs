use emmylua_parser::{LuaAstNode, LuaExpr, LuaIndexExpr, LuaNameExpr};

use crate::{
    semantic::{
        member::{get_buildin_type_map_type_id, without_members},
        InferGuard,
    },
    DbIndex, LuaDeclId, LuaDeclOrMemberId, LuaInferConfig, LuaInstanceType, LuaMemberId,
    LuaMemberKey, LuaMemberOwner, LuaPropertyOwnerId, LuaType, LuaTypeDeclId, LuaUnionType,
};

use super::infer_expr;

pub fn infer_expr_property_owner(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    expr: LuaExpr,
) -> Option<LuaPropertyOwnerId> {
    let file_id = infer_config.get_file_id();
    let maybe_decl_id = LuaDeclId::new(file_id, expr.get_position());
    if let Some(_) = db.get_decl_index().get_decl(&maybe_decl_id) {
        return Some(LuaPropertyOwnerId::LuaDecl(maybe_decl_id));
    };

    match expr {
        LuaExpr::NameExpr(name_expr) => infer_name_expr_property_owner(db, infer_config, name_expr),
        LuaExpr::IndexExpr(index_expr) => {
            infer_index_expr_property_owner(db, infer_config, index_expr)
        }
        _ => {
            let member_id = LuaMemberId::new(expr.get_syntax_id(), file_id);
            if let Some(_) = db.get_member_index().get_member(&member_id) {
                return Some(LuaPropertyOwnerId::Member(member_id));
            };

            None
        }
    }
}

fn infer_name_expr_property_owner(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    name_expr: LuaNameExpr,
) -> Option<LuaPropertyOwnerId> {
    let name_token = name_expr.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    if name == "self" {
        return infer_self_property_owner(db, infer_config, name_expr);
    }

    let file_id = infer_config.get_file_id();
    let references_index = db.get_reference_index();
    let range = name_expr.get_range();
    let local_ref = references_index.get_local_reference(&file_id)?;
    let decl_id = local_ref.get_decl_id(&range);

    if let Some(decl_id) = decl_id {
        let decl = db.get_decl_index().get_decl(&decl_id)?;
        if decl.is_local() {
            return Some(LuaPropertyOwnerId::LuaDecl(decl_id));
        }
    }

    let decl_id = db
        .get_decl_index()
        .get_global_decl_id(&LuaMemberKey::Name(name.into()));

    if decl_id.is_some() {
        Some(LuaPropertyOwnerId::LuaDecl(decl_id.unwrap()))
    } else {
        None
    }
}

fn infer_self_property_owner(
    db: &DbIndex,
    config: &LuaInferConfig,
    name_expr: LuaNameExpr,
) -> Option<LuaPropertyOwnerId> {
    let file_id = config.get_file_id();
    let tree = db.get_decl_index().get_decl_tree(&file_id)?;
    let id = tree.find_self_decl(db, name_expr)?;
    match id {
        LuaDeclOrMemberId::Decl(decl_id) => {
            return Some(LuaPropertyOwnerId::LuaDecl(decl_id));
        }
        LuaDeclOrMemberId::Member(member_id) => {
            return Some(LuaPropertyOwnerId::Member(member_id));
        }
    }
}

fn infer_index_expr_property_owner(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    index_expr: LuaIndexExpr,
) -> Option<LuaPropertyOwnerId> {
    let prefix_expr = index_expr.get_prefix_expr()?;
    let prefix_type = infer_expr(db, infer_config, prefix_expr.into())?;

    let member_key = index_expr.get_index_key()?.into();
    if let Some(member_info) = infer_member_property_owner_by_member_key(
        db,
        infer_config,
        &prefix_type,
        &member_key,
        &mut InferGuard::new(),
    ) {
        return Some(member_info);
    }

    None
}

fn infer_member_property_owner_by_member_key(
    db: &DbIndex,
    config: &LuaInferConfig,
    prefix_type: &LuaType,
    member_key: &LuaMemberKey,
    infer_guard: &mut InferGuard,
) -> Option<LuaPropertyOwnerId> {
    if without_members(prefix_type) {
        return None;
    }

    match &prefix_type {
        LuaType::TableConst(id) => {
            let member_owner = LuaMemberOwner::Element(id.clone());
            infer_table_member_property_owner(db, member_owner, member_key)
        }
        LuaType::String | LuaType::Io | LuaType::StringConst(_) => {
            let decl_id = get_buildin_type_map_type_id(&prefix_type)?;
            infer_custom_type_member_property_owner(db, config, decl_id, member_key, infer_guard)
        }
        LuaType::Ref(decl_id) => infer_custom_type_member_property_owner(
            db,
            config,
            decl_id.clone(),
            member_key,
            infer_guard,
        ),
        LuaType::Def(decl_id) => infer_custom_type_member_property_owner(
            db,
            config,
            decl_id.clone(),
            member_key,
            infer_guard,
        ),
        // LuaType::Module(_) => todo!(),
        LuaType::KeyOf(_) => {
            let decl_id = LuaTypeDeclId::new("string");
            infer_custom_type_member_property_owner(db, config, decl_id, member_key, infer_guard)
        }
        LuaType::Nullable(inner_type) => infer_member_property_owner_by_member_key(
            db,
            config,
            &inner_type,
            member_key,
            infer_guard,
        ),
        LuaType::Union(union_type) => {
            infer_union_member_semantic_info(db, config, &union_type, member_key)
        }
        LuaType::Generic(generic_type) => infer_custom_type_member_property_owner(
            db,
            config,
            generic_type.get_base_type_id(),
            member_key,
            infer_guard,
        ),
        LuaType::MemberPathExist(exist_field) => infer_member_property_owner_by_member_key(
            db,
            config,
            exist_field.get_origin(),
            member_key,
            infer_guard,
        ),
        LuaType::Instance(inst) => {
            infer_instance_member_property_by_member_key(db, config, inst, member_key, infer_guard)
        }
        LuaType::Global => {
            infer_global_member_property_by_member_key(db, config, member_key, infer_guard)
        }
        _ => None,
    }
}

fn infer_table_member_property_owner(
    db: &DbIndex,
    member_owner: LuaMemberOwner,
    member_key: &LuaMemberKey,
) -> Option<LuaPropertyOwnerId> {
    let member_index = db.get_member_index();
    let member_map = member_index.get_member_map(member_owner)?;
    let member_id = member_map.get(&member_key)?;

    Some(LuaPropertyOwnerId::Member(member_id.clone()))
}

fn infer_custom_type_member_property_owner(
    db: &DbIndex,
    config: &LuaInferConfig,
    prefix_type_id: LuaTypeDeclId,
    member_key: &LuaMemberKey,
    infer_guard: &mut InferGuard,
) -> Option<LuaPropertyOwnerId> {
    infer_guard.check(&prefix_type_id)?;
    let type_index = db.get_type_index();
    let type_decl = type_index.get_type_decl(&prefix_type_id)?;
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin(db, None) {
            return infer_member_property_owner_by_member_key(
                db,
                config,
                &origin_type,
                member_key,
                infer_guard,
            );
        } else {
            return infer_member_property_owner_by_member_key(
                db,
                config,
                &LuaType::String,
                member_key,
                infer_guard,
            );
        }
    }

    let member_owner = LuaMemberOwner::Type(prefix_type_id.clone());
    let member_index = db.get_member_index();
    // find member by key in self
    if let Some(member_map) = member_index.get_member_map(member_owner) {
        if let Some(member_id) = member_map.get(&member_key) {
            return Some(LuaPropertyOwnerId::Member(member_id.clone()));
        }
    }

    // find member by key in super
    if type_decl.is_class() {
        let super_types = type_index.get_super_types(&prefix_type_id)?;
        for super_type in super_types {
            let member_info = infer_member_property_owner_by_member_key(
                db,
                config,
                &super_type,
                member_key,
                infer_guard,
            );
            if member_info.is_some() {
                return member_info;
            }
        }
    }

    None
}

fn infer_union_member_semantic_info(
    db: &DbIndex,
    config: &LuaInferConfig,
    union_type: &LuaUnionType,
    member_key: &LuaMemberKey,
) -> Option<LuaPropertyOwnerId> {
    for typ in union_type.get_types() {
        if let Some(property_owner_id) = infer_member_property_owner_by_member_key(
            db,
            config,
            typ,
            member_key,
            &mut InferGuard::new(),
        ) {
            return Some(property_owner_id);
        }
    }

    None
}

fn infer_instance_member_property_by_member_key(
    db: &DbIndex,
    config: &LuaInferConfig,
    inst: &LuaInstanceType,
    member_key: &LuaMemberKey,
    infer_guard: &mut InferGuard,
) -> Option<LuaPropertyOwnerId> {
    let range = inst.get_range();

    let origin_type = inst.get_base();
    if let Some(result) =
        infer_member_property_owner_by_member_key(db, config, origin_type, member_key, infer_guard)
    {
        return Some(result);
    }

    let member_owner = LuaMemberOwner::Element(range.clone());
    if let Some(result) = infer_table_member_property_owner(db, member_owner, member_key) {
        return Some(result);
    }

    None
}

fn infer_global_member_property_by_member_key(
    db: &DbIndex,
    _: &LuaInferConfig,
    member_key: &LuaMemberKey,
    _: &mut InferGuard,
) -> Option<LuaPropertyOwnerId> {
    let decl_id = db.get_decl_index().get_global_decl_id(member_key);

    if decl_id.is_some() {
        Some(LuaPropertyOwnerId::LuaDecl(decl_id.unwrap()))
    } else {
        None
    }
}
