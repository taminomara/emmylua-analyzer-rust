use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr, LuaLocalStat};

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::NonLiteralExpressionsInAssert];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for call_expr in root.descendants::<LuaCallExpr>() {
        if call_expr.is_assert() {
            check_assert_rule(context, semantic_model, call_expr);
        }
    }

    Some(())
}

fn check_assert_rule(
    context: &mut DiagnosticContext,
    _: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    // only check local a = assert(b, msg)
    call_expr.get_parent::<LuaLocalStat>()?;

    let args = call_expr.get_args_list()?;
    let arg_exprs = args.get_args().collect::<Vec<_>>();
    if arg_exprs.len() > 1 {
        let second_expr = &arg_exprs[1];
        match second_expr {
            LuaExpr::LiteralExpr(_) => {}
            _ => {
                let range = second_expr.get_range();
                context.add_diagnostic(
                    DiagnosticCode::NonLiteralExpressionsInAssert,
                    range,
                    t!("codestyle.NonLiteralExpressionsInAssert").to_string(),
                    None,
                );
            }
        }
    }

    Some(())
}
