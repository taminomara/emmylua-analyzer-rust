use emmylua_parser::{LuaAst, LuaAstNode, LuaCallExpr};

use crate::{humanize_type, DiagnosticCode, LuaType, RenderLevel, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::ParamTypeNotMatch];

/// a simple implementation of param type check, later we will do better
pub fn check(context: &mut DiagnosticContext, semantic_model: &mut SemanticModel) -> Option<()> {
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
    semantic_model: &mut SemanticModel,
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

                    if !semantic_model.check_type_compact(&param_type, &expr_type) {
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

            if !semantic_model.check_type_compact(&param_type, &expr_type) {
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

    Some(())
}
