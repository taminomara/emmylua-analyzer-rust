use emmylua_parser::{LuaAst, LuaAstNode, LuaCallExpr};
use rowan::TextRange;

use crate::{humanize_type, DiagnosticCode, LuaType, RenderLevel, SemanticModel, TypeCheckFailReason, TypeCheckResult};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::ParamTypeNotMatch];

/// a simple implementation of param type check, later we will do better
pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaCallExpr(call_expr) => {
                check_call_expr(context, semantic_model, call_expr);
            }
            _ => {}
        }
    }

    Some(())
}

fn check_call_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let func = semantic_model.infer_call_expr_func(call_expr.clone(), None)?;
    let params = func.get_params();
    let mut args = call_expr
        .get_args_list()?
        .get_args()
        .map(|arg| Some(arg))
        .collect::<Vec<_>>();
    let colon_call = call_expr.is_colon_call();
    let colon_define = func.is_colon_define();
    match (colon_call, colon_define) {
        (true, true) | (false, false) => {}
        (false, true) => {
            if args.len() > 0 {
                args.remove(0);
            }
        }
        (true, false) => {
            args.insert(0, None);
        }
    }

    for (idx, param) in params.iter().enumerate() {
        if idx >= args.len() {
            break;
        }

        let arg = &args[idx];
        if arg.is_none() {
            continue;
        }
        let arg = arg.clone().unwrap();

        if param.0 == "..." {
            if param.1.is_none() {
                break;
            }

            let param_type = param.1.clone().unwrap();
            for arg in args.iter().skip(idx) {
                if let Some(arg) = arg {
                    let mut expr_type = semantic_model
                        .infer_expr(arg.clone())
                        .unwrap_or(LuaType::Any);
                    // treat unknown type as any
                    if expr_type.is_unknown() {
                        expr_type = LuaType::Any;
                    }

                    if !semantic_model.type_check(&param_type, &expr_type).is_ok() {
                        let db = semantic_model.get_db();
                        context.add_diagnostic(
                            DiagnosticCode::ParamTypeNotMatch,
                            arg.get_range(),
                            format!(
                                "expected {} but found {}",
                                humanize_type(db, &param_type, RenderLevel::Simple),
                                humanize_type(db, &expr_type, RenderLevel::Simple)
                            ),
                            None,
                        );
                    }
                }
            }
        } else {
            if param.1.is_none() {
                continue;
            }

            let param_type = param.1.clone().unwrap();
            let mut expr_type = semantic_model
                .infer_expr(arg.clone())
                .unwrap_or(LuaType::Any);
            // treat unknown type as any
            if expr_type.is_unknown() {
                expr_type = LuaType::Any;
            }
            // todo: use type check result
            let result = semantic_model.type_check(&param_type, &expr_type);
            if !result.is_ok() {
                add_type_check_diagnostic(
                    context,
                    semantic_model,
                    arg.get_range(),
                    &param_type,
                    &expr_type,
                    result,
                );
            }
        }
    }

    Some(())
}

fn add_type_check_diagnostic(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    range: TextRange,
    param_type: &LuaType,
    expr_type: &LuaType,
    result: TypeCheckResult,
) {
    let db = semantic_model.get_db();
    match result {
        Ok(_) => return,
        Err(reason) => match reason {
            TypeCheckFailReason::TypeNotMatchWithReason(reason) => {
                context.add_diagnostic(
                    DiagnosticCode::ParamTypeNotMatch,
                    range,
                    reason,
                    None,
                );
            }
            TypeCheckFailReason::TypeNotMatch => {
                context.add_diagnostic(
                    DiagnosticCode::ParamTypeNotMatch,
                    range,
                    t!(
                        "expected %{source} but found %{found}",
                        source = humanize_type(db, &param_type, RenderLevel::Simple),
                        found = humanize_type(db, &expr_type, RenderLevel::Simple)
                    ).to_string(),
                    None,
                );
            }
            TypeCheckFailReason::TypeRecursion => {
                context.add_diagnostic(
                    DiagnosticCode::ParamTypeNotMatch,
                    range,
                    "type recursion".into(),
                    None,
                );
            }
        },
    }
}
