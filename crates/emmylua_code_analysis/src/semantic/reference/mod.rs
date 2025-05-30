use std::collections::HashSet;

use emmylua_parser::{
    LuaAssignStat, LuaAstNode, LuaSyntaxKind, LuaSyntaxNode, LuaTableExpr, LuaTableField,
};

use crate::{DbIndex, LuaMemberId, LuaMemberKey, LuaSemanticDeclId, LuaType};

use super::{
    infer_table_should_be, member::find_members, semantic_info::infer_node_semantic_decl,
    LuaInferCache, SemanticDeclLevel,
};

pub fn is_reference_to(
    db: &DbIndex,
    infer_config: &mut LuaInferCache,
    node: LuaSyntaxNode,
    semantic_decl: LuaSemanticDeclId,
    level: SemanticDeclLevel,
) -> Option<bool> {
    let node_semantic_decl_id = infer_node_semantic_decl(db, infer_config, node, level)?;
    if node_semantic_decl_id == semantic_decl {
        return Some(true);
    }

    match (node_semantic_decl_id, &semantic_decl) {
        (LuaSemanticDeclId::Member(node_member_id), LuaSemanticDeclId::Member(member_id)) => {
            if let Some(true) = is_member_reference_to(db, node_member_id, *member_id) {
                return Some(true);
            }

            is_member_origin_reference_to(db, infer_config, node_member_id, semantic_decl)
        }
        _ => Some(false),
    }
}

fn is_member_reference_to(
    db: &DbIndex,
    node_member_id: LuaMemberId,
    member_id: LuaMemberId,
) -> Option<bool> {
    let node_owner = db.get_member_index().get_current_owner(&node_member_id)?;
    let owner = db.get_member_index().get_current_owner(&member_id)?;

    Some(node_owner == owner)
}

fn is_member_origin_reference_to(
    db: &DbIndex,
    infer_config: &mut LuaInferCache,
    node_member_id: LuaMemberId,
    semantic_decl: LuaSemanticDeclId,
) -> Option<bool> {
    let node_origin = find_member_origin_owner(db, infer_config, node_member_id)
        .unwrap_or(LuaSemanticDeclId::Member(node_member_id));

    match (node_origin, semantic_decl) {
        (LuaSemanticDeclId::Member(node_owner), LuaSemanticDeclId::Member(member_owner)) => {
            is_member_reference_to(db, node_owner, member_owner)
        }
        (node_origin, member_origin) => Some(node_origin == member_origin),
    }
}

pub fn find_member_origin_owner(
    db: &DbIndex,
    infer_config: &mut LuaInferCache,
    member_id: LuaMemberId,
) -> Option<LuaSemanticDeclId> {
    const MAX_ITERATIONS: usize = 50;
    let mut visited_members = HashSet::new();

    let mut current_owner = resolve_member_owner(db, infer_config, &member_id);
    let mut final_owner = current_owner.clone();
    let mut iteration_count = 0;

    while let Some(LuaSemanticDeclId::Member(current_member_id)) = &current_owner {
        if visited_members.contains(current_member_id) || iteration_count >= MAX_ITERATIONS {
            break;
        }

        visited_members.insert(current_member_id.clone());
        iteration_count += 1;

        match resolve_member_owner(db, infer_config, current_member_id) {
            Some(next_owner) => {
                final_owner = Some(next_owner.clone());
                current_owner = Some(next_owner);
            }
            None => break,
        }
    }

    final_owner
}

fn resolve_member_owner(
    db: &DbIndex,
    infer_config: &mut LuaInferCache,
    member_id: &LuaMemberId,
) -> Option<LuaSemanticDeclId> {
    let root = db
        .get_vfs()
        .get_syntax_tree(&member_id.file_id)?
        .get_red_root();
    let current_node = member_id.get_syntax_id().to_node_from_root(&root)?;
    match member_id.get_syntax_id().get_kind() {
        LuaSyntaxKind::TableFieldAssign => {
            if LuaTableField::can_cast(current_node.kind().into()) {
                let table_field = LuaTableField::cast(current_node.clone())?;
                // 如果表是类, 那么通过类型推断获取 owner
                if let Some(owner_id) =
                    resolve_table_field_through_type_inference(db, infer_config, &table_field)
                {
                    return Some(owner_id);
                }
                // 非类, 那么通过右值推断
                let value_expr = table_field.get_value_expr()?;
                let value_node = value_expr.get_syntax_id().to_node_from_root(&root)?;
                infer_node_semantic_decl(db, infer_config, value_node, SemanticDeclLevel::default())
            } else {
                None
            }
        }
        LuaSyntaxKind::IndexExpr => {
            let assign_node = current_node.parent()?;
            let assign_stat = LuaAssignStat::cast(assign_node)?;
            let (vars, exprs) = assign_stat.get_var_and_expr_list();

            for (var, expr) in vars.iter().zip(exprs.iter()) {
                if var.syntax().text_range() == current_node.text_range() {
                    let expr_node = expr.get_syntax_id().to_node_from_root(&root)?;
                    return infer_node_semantic_decl(
                        db,
                        infer_config,
                        expr_node,
                        SemanticDeclLevel::default(),
                    );
                }
            }
            None
        }
        _ => None,
    }
}

fn resolve_table_field_through_type_inference(
    db: &DbIndex,
    infer_config: &mut LuaInferCache,
    table_field: &LuaTableField,
) -> Option<LuaSemanticDeclId> {
    let parent = table_field.syntax().parent()?;
    let table_expr = LuaTableExpr::cast(parent)?;
    let table_type = infer_table_should_be(db, infer_config, table_expr).ok()?;

    if !matches!(table_type, LuaType::Ref(_) | LuaType::Def(_)) {
        return None;
    }

    let field_key = table_field.get_field_key()?;
    let key = LuaMemberKey::from_index_key(db, infer_config, &field_key).ok()?;
    let member_infos = find_members(db, &table_type)?;

    member_infos
        .iter()
        .find(|m| m.key == key)?
        .property_owner_id
        .clone()
}
