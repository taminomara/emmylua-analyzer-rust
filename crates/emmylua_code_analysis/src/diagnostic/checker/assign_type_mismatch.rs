use emmylua_parser::{
    LuaAssignStat, LuaAst, LuaAstNode, LuaAstToken, LuaExpr, LuaIndexExpr, LuaLocalStat,
    LuaNameExpr, LuaTableExpr, LuaVarExpr,
};
use rowan::TextRange;

use crate::{
    DiagnosticCode, LuaDeclId, LuaPropertyOwnerId, LuaType, SemanticModel, TypeCheckResult,
};

use super::{humanize_lint_type, DiagnosticContext};

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::AssignTypeMismatch];

#[allow(unused)]
pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaAssignStat(assign) => {
                check_assign_stat(context, semantic_model, &assign);
            }
            LuaAst::LuaLocalStat(local) => {
                check_local_stat(context, semantic_model, &local);
            }
            _ => {}
        }
    }
    Some(())
}

fn check_assign_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    assign: &LuaAssignStat,
) -> Option<()> {
    let (vars, exprs) = assign.get_var_and_expr_list();
    for (var, expr) in vars.iter().zip(exprs.iter()) {
        match var {
            LuaVarExpr::IndexExpr(index_expr) => {
                check_index_expr(context, semantic_model, index_expr, expr.clone());
            }
            LuaVarExpr::NameExpr(name_expr) => {
                check_name_expr(context, semantic_model, name_expr, expr.clone());
            }
        }
        check_table_expr(context, semantic_model, expr);
    }
    Some(())
}

fn check_name_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    name_expr: &LuaNameExpr,
    expr: LuaExpr,
) -> Option<()> {
    let property_owner_id = semantic_model
        .get_property_owner_id(rowan::NodeOrToken::Node(name_expr.syntax().clone()))?;
    let origin_type = match property_owner_id {
        LuaPropertyOwnerId::LuaDecl(decl_id) => {
            let decl = semantic_model
                .get_db()
                .get_decl_index()
                .get_decl(&decl_id)?;
            decl.get_type().cloned()
        }
        _ => None,
    };
    let expr_type = semantic_model.infer_expr(expr);
    check_assign_type_mismatch(
        context,
        semantic_model,
        name_expr.get_range(),
        origin_type,
        expr_type,
        false,
    );
    Some(())
}

fn check_index_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    index_expr: &LuaIndexExpr,
    expr: LuaExpr,
) -> Option<()> {
    let member_info =
        semantic_model.get_semantic_info(rowan::NodeOrToken::Node(index_expr.syntax().clone()))?;
    let expr_type = semantic_model.infer_expr(expr);
    check_assign_type_mismatch(
        context,
        semantic_model,
        index_expr.get_range(),
        Some(member_info.typ),
        expr_type,
        true,
    );
    Some(())
}

fn check_local_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    local: &LuaLocalStat,
) -> Option<()> {
    let root = semantic_model.get_root().syntax();

    for name in local.get_local_name_list() {
        let name_token = name.get_name_token()?;
        let position = name_token.get_position();
        let file_id = semantic_model.get_file_id();
        let decl_id = LuaDeclId::new(file_id, position);
        let decl = semantic_model
            .get_db()
            .get_decl_index()
            .get_decl(&decl_id)?;
        let value_expr = LuaExpr::cast(decl.get_value_syntax_id()?.to_node_from_root(root)?)?;
        check_table_expr(context, semantic_model, &value_expr);
        check_assign_type_mismatch(
            context,
            semantic_model,
            decl.get_range(),
            decl.get_type().cloned(),
            semantic_model.infer_expr(value_expr),
            false,
        );
    }
    Some(())
}

// 处理 value_expr 是 TableExpr 的情况
fn check_table_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    value_expr: &LuaExpr,
) -> Option<()> {
    let table_expr = LuaTableExpr::cast(value_expr.syntax().clone())?;
    let table_type = semantic_model.infer_table_should_be(table_expr.clone())?;
    let member_infos = semantic_model.infer_member_infos(&table_type)?;
    table_expr.get_fields().for_each(|field| {
        let field_key = field.get_field_key();
        if let Some(field_key) = field_key {
            let field_path_part = field_key.get_path_part();
            let source_type = member_infos
                .iter()
                .find(|info| info.key.to_path() == field_path_part)
                .map(|info| info.typ.clone());
            let expr = field.get_value_expr();
            if let Some(expr) = expr {
                let expr_type = semantic_model.infer_expr(expr);

                let allow_nil = match table_type {
                    LuaType::Array(_) => true,
                    _ => false,
                };

                check_assign_type_mismatch(
                    context,
                    semantic_model,
                    field.get_range(),
                    source_type,
                    expr_type,
                    allow_nil,
                );
            }
        }
    });
    Some(())
}

fn check_assign_type_mismatch(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    range: TextRange,
    source_type: Option<LuaType>,
    value_type: Option<LuaType>,
    allow_nil: bool,
) -> Option<()> {
    let source_type = source_type.unwrap_or(LuaType::Any);
    let value_type = value_type.unwrap_or(LuaType::Any);

    // 某些情况下我们应允许可空, 例如: boolean[]
    if allow_nil && value_type.is_optional() {
        return Some(());
    }

    match (&source_type, &value_type) {
        // 如果源类型是定义类型, 则不进行类型检查, 除非源类型是定义类型
        (LuaType::Def(_), LuaType::Def(_)) => {}
        (LuaType::Def(_), _) => return Some(()),
        // 此时检查交给 table_field
        (LuaType::Ref(_) | LuaType::Tuple(_), LuaType::TableConst(_)) => return Some(()),
        // 如果源类型是nil, 则不进行类型检查
        (LuaType::Nil, _) => return Some(()),
        _ => {}
    }

    let result = semantic_model.type_check(&source_type, &value_type);
    if !result.is_ok() {
        add_type_check_diagnostic(
            context,
            semantic_model,
            range,
            &source_type,
            &value_type,
            result,
        );
    }
    Some(())
}

fn add_type_check_diagnostic(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    range: TextRange,
    source_type: &LuaType,
    value_type: &LuaType,
    result: TypeCheckResult,
) {
    match result {
        Ok(_) => return,
        Err(_) => {
            let db = semantic_model.get_db();
            context.add_diagnostic(
                DiagnosticCode::AssignTypeMismatch,
                range,
                t!(
                    "Cannot assign `%{value}` to `%{source}`.",
                    value = humanize_lint_type(db, &value_type),
                    source = humanize_lint_type(db, &source_type)
                )
                .to_string(),
                None,
            );
        }
    }
}
