use emmylua_parser::{LuaAstNode, LuaAstToken, LuaCallExpr, LuaGeneralToken, LuaLiteralExpr, LuaLiteralToken};

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::MissingParameter];

pub fn check(context: &mut DiagnosticContext, semantic_model: &mut SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for call_expr in root.descendants::<LuaCallExpr>() {
        check_call_expr(context, semantic_model, call_expr);
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
    let args_count = call_expr.get_args_list()?.get_args().count();
    if args_count < params.len() {
        // fix last arg is `...`
        if args_count != 0 {
            let last_arg = call_expr.get_args_list()?.child::<LuaLiteralExpr>()?;
            if let Some(literal_token) = last_arg.get_literal() {
                if let LuaLiteralToken::Dots(_) = literal_token {
                    return Some(());
                }
            }
        }


        let mut miss_parameter_info = Vec::new();
        for i in args_count..params.len() {
            let param_info = params.get(i)?;
            if param_info.0 == "..." {
                break;
            }

            let typ = param_info.1.clone();
            if typ.is_none() || !typ.unwrap().is_optional() {
                miss_parameter_info.push(t!("missing parameter: %{name}", name = param_info.0));
            }
        }

        let right_paren = call_expr
            .get_args_list()?
            .tokens::<LuaGeneralToken>()
            .last()?;
        if !miss_parameter_info.is_empty() {
            context.add_diagnostic(
                DiagnosticCode::MissingParameter,
                right_paren.get_range(),
                t!(
                    "expected %{num} but founded %{found_num}.\n%{infos}",
                    num = params.len(),
                    found_num = args_count,
                    infos = miss_parameter_info.join("\n")
                ).to_string(),
                None,
            );
        }
    }

    Some(())
}
