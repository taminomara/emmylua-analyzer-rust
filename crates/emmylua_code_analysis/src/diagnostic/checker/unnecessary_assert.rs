use emmylua_parser::{LuaAstNode, LuaCallExpr};

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::UnnecessaryAssert];

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
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let args = call_expr.get_args_list()?;
    let arg_exprs = args.get_args().collect::<Vec<_>>();
    if let Some(first_expr) = arg_exprs.first() {
        let expr_type = semantic_model.infer_expr(first_expr.clone());
        if expr_type.is_some_and(|t| t.is_always_truthy()) {
            context.add_diagnostic(
                DiagnosticCode::UnnecessaryAssert,
                call_expr.get_range(),
                t!("Unnecessary assert: this expression is always truthy").to_string(),
                None,
            );
        }
    }
    Some(())
}
