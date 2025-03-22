use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaExpr, LuaLocalStat, LuaStat};

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::UnbalancedAssignments];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for stat in root.descendants::<LuaStat>() {
        match stat {
            LuaStat::LocalStat(local) => {
                check_local_stat(context, semantic_model, &local);
            }
            LuaStat::AssignStat(assign) => {
                check_assign_stat(context, semantic_model, &assign);
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
    let (vars, value_exprs) = assign.get_var_and_expr_list();
    check_unbalanced_assignment(context, semantic_model, &vars, &value_exprs)
}

fn check_local_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    local: &LuaLocalStat,
) -> Option<()> {
    let vars = local.get_local_name_list().collect::<Vec<_>>();
    let value_exprs = local.get_value_exprs().collect::<Vec<_>>();
    check_unbalanced_assignment(context, semantic_model, &vars, &value_exprs)
}

fn check_unbalanced_assignment(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    vars: &[impl LuaAstNode],
    value_exprs: &[LuaExpr],
) -> Option<()> {
    if value_exprs.is_empty() {
        return Some(());
    }

    let value_types = semantic_model
        .infer_multi_value_adjusted_expression_types(value_exprs, Some(vars.len()))?;

    let value_len = value_types.len();

    if vars.len() > value_len {
        for var in vars[value_len..].iter() {
            context.add_diagnostic(
                DiagnosticCode::UnbalancedAssignments,
                var.get_range(),
                t!("The value is assigned as `nil` because the number of values is not enough.")
                    .to_string(),
                None,
            );
        }
    }

    Some(())
}
