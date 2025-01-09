use emmylua_parser::{LuaAst, LuaAstNode, LuaCallExpr};

use crate::{humanize_type, DiagnosticCode, LuaType, SemanticModel};

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
    let args = call_expr.get_args_list()?.get_args().collect::<Vec<_>>();
    for (idx, param) in params.iter().enumerate() {
        if idx >= args.len() {
            break;
        }

        if param.0 == "..." {
            if param.1.is_none() {
                break;
            }

            let param_type = param.1.clone().unwrap();
            for arg in args.iter().skip(idx) {
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
                            "expected {} but founded {}",
                            humanize_type(db, &param_type),
                            humanize_type(db, &expr_type)
                        ),
                        None,
                    );
                }
            }
        } else {
            if param.1.is_none() {
                continue;
            }

            let param_type = param.1.clone().unwrap();
            let arg = &args[idx];
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
                        "expected {} but founded {}",
                        humanize_type(db, &param_type),
                        humanize_type(db, &expr_type)
                    ),
                    None,
                );
            }
        }
    }

    Some(())
}
