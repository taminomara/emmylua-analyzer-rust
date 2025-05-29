use emmylua_code_analysis::{
    LuaDeclId, LuaMemberId, LuaSemanticDeclId, LuaType, SemanticDeclLevel, SemanticModel,
};
use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaSyntaxKind, LuaTableExpr, LuaTableField};

pub fn find_function_decl_origin_owner(
    semantic_model: &SemanticModel,
    decl_id: LuaDeclId,
) -> Option<LuaSemanticDeclId> {
    let root = semantic_model
        .get_db()
        .get_vfs()
        .get_syntax_tree(&decl_id.file_id)?
        .get_red_root();
    let node = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)?
        .get_value_syntax_id()?
        .to_node_from_root(&root)?;
    let semantic_decl = semantic_model.find_decl(node.into(), SemanticDeclLevel::default());
    match semantic_decl {
        Some(LuaSemanticDeclId::Member(member_id)) => {
            find_function_member_origin_owner(semantic_model, member_id).or(semantic_decl)
        }
        Some(LuaSemanticDeclId::LuaDecl(_)) => semantic_decl,
        _ => None,
    }
}
pub fn find_function_member_origin_owner(
    semantic_model: &SemanticModel,
    member_id: LuaMemberId,
) -> Option<LuaSemanticDeclId> {
    let mut current_owner = resolve_member_owner(semantic_model, &member_id);
    let mut final_owner = current_owner.clone();

    while let Some(LuaSemanticDeclId::Member(current_member_id)) = &current_owner {
        match resolve_member_owner(semantic_model, current_member_id) {
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
    semantic_model: &SemanticModel,
    member_id: &LuaMemberId,
) -> Option<LuaSemanticDeclId> {
    let root = semantic_model
        .get_db()
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
                    resolve_table_field_through_type_inference(semantic_model, &table_field)
                {
                    return Some(owner_id);
                }
                // 非类, 那么通过右值推断
                let value_expr = table_field.get_value_expr()?;
                let value_node = value_expr.get_syntax_id().to_node_from_root(&root)?;
                semantic_model.find_decl(value_node.into(), SemanticDeclLevel::default())
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
                    return semantic_model
                        .find_decl(expr_node.into(), SemanticDeclLevel::default());
                }
            }
            None
        }
        _ => None,
    }
}

fn resolve_table_field_through_type_inference(
    semantic_model: &SemanticModel,
    table_field: &LuaTableField,
) -> Option<LuaSemanticDeclId> {
    let parent = table_field.syntax().parent()?;
    let table_expr = LuaTableExpr::cast(parent)?;
    let table_type = semantic_model.infer_table_should_be(table_expr)?;

    if !matches!(table_type, LuaType::Ref(_) | LuaType::Def(_)) {
        return None;
    }

    let field_key = table_field.get_field_key()?;
    let key = semantic_model.get_member_key(&field_key)?;
    let member_infos = semantic_model.get_member_infos(&table_type)?;

    member_infos
        .iter()
        .find(|m| m.key == key)?
        .property_owner_id
        .clone()
}

pub fn is_function(typ: &LuaType) -> bool {
    typ.is_function()
        || match &typ {
            LuaType::Union(union) => union
                .get_types()
                .iter()
                .all(|t| matches!(t, LuaType::DocFunction(_) | LuaType::Signature(_))),
            _ => false,
        }
}
