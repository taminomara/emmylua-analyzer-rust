use std::collections::HashSet;

use emmylua_parser::{LuaAst, LuaAstNode, LuaIndexExpr, LuaIndexKey, LuaVarExpr};

use crate::{DiagnosticCode, LuaType, SemanticModel};

use super::{humanize_lint_type, Checker, DiagnosticContext};

pub struct CheckFieldChecker;

impl Checker for CheckFieldChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::InjectField, DiagnosticCode::UndefinedField];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
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
    }
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
    match &prefix_typ {
        LuaType::Any
        | LuaType::Unknown
        | LuaType::Table
        | LuaType::TplRef(_)
        | LuaType::StrTplRef(_)
        | LuaType::TableConst(_) => return Some(()),
        _ => {}
    }

    let index_key = index_expr.get_index_key()?;
    if let LuaIndexKey::Expr(expr) = &index_key {
        let expr_typ = semantic_model.infer_expr(expr.clone())?;
        match &expr_typ {
            LuaType::Any
            | LuaType::Unknown
            | LuaType::Table
            | LuaType::TplRef(_)
            | LuaType::StrTplRef(_) => {
                return Some(());
            }
            _ => {}
        }
    }

    let index_name = index_key.get_path_part();
    let member_info =
        semantic_model.get_semantic_info(rowan::NodeOrToken::Node(index_expr.syntax().clone()));

    let mut need_add_diagnostic = false;
    if let Some(member_info) = member_info {
        if member_info.semantic_decl.is_none() && member_info.typ.is_unknown() {
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
                        class = humanize_lint_type(&db, &prefix_typ),
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
                    t!("Undefined field `%{field}`. ", field = index_name,).to_string(),
                    None,
                );
            }
            _ => {}
        }
    }

    Some(())
}
