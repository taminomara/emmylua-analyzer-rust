use std::collections::HashSet;

use emmylua_parser::{LuaAst, LuaAstNode, LuaIndexExpr, LuaVarExpr};

use crate::{DiagnosticCode, SemanticModel};

use super::{get_lint_type_name, DiagnosticContext};

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::InjectField, DiagnosticCode::UndefinedField];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    let mut checked_index_expr = HashSet::new();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaAssignStat(assign) => {
                let (vars, _) = assign.get_var_and_expr_list();
                for var in vars.iter() {
                    if let LuaVarExpr::IndexExpr(index_expr) = var {
                        checked_index_expr.insert(index_expr.syntax().clone());
                        check_index_expr(
                            context,
                            semantic_model,
                            index_expr,
                            DiagnosticCode::InjectField,
                        );
                    }
                }
            }
            LuaAst::LuaIndexExpr(index_expr) => {
                if checked_index_expr.contains(index_expr.syntax()) {
                    continue;
                }
                check_index_expr(
                    context,
                    semantic_model,
                    &index_expr,
                    DiagnosticCode::UndefinedField,
                );
            }
            _ => {}
        }
    }
    Some(())
}

fn check_index_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    index_expr: &LuaIndexExpr,
    code: DiagnosticCode,
) -> Option<()> {
    let db = context.db;
    let prefix = index_expr.get_prefix_expr()?;
    let prefix_typ = semantic_model.infer_expr(prefix)?;
    let index_key = index_expr.get_index_key()?;
    let index_name = index_key.get_path_part();
    let member_info =
        semantic_model.get_semantic_info(rowan::NodeOrToken::Node(index_expr.syntax().clone()));

    let mut need_add_diagnostic = false;
    if let Some(member_info) = member_info {
        if member_info.property_owner.is_none() && member_info.typ.is_unknown() {
            need_add_diagnostic = true;
        }
    } else {
        need_add_diagnostic = true;
    }

    if need_add_diagnostic {
        match code {
            DiagnosticCode::InjectField => {
                context.add_diagnostic(
                    DiagnosticCode::InjectField,
                    index_key.get_range()?,
                    t!(
                        "Fields cannot be injected into the reference of `%{class}` for `%{field}`. ",
                        class = get_lint_type_name(&db, &prefix_typ),
                        field = index_name,
                    )
                    .to_string(),
                    None,
                );
            }
            DiagnosticCode::UndefinedField => {
                context.add_diagnostic(
                    DiagnosticCode::UndefinedField,
                    index_key.get_range()?,
                    t!(
                        "Undefined field `%{field}` in the reference of `%{class}`. ",
                        field = index_name,
                        class = get_lint_type_name(&db, &prefix_typ),
                    )
                    .to_string(),
                    None,
                );
            }
            _ => {}
        }
    }

    Some(())
}
