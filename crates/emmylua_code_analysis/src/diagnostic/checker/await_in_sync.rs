use emmylua_parser::{LuaAstNode, LuaCallArgList, LuaCallExpr, LuaClosureExpr, LuaExpr};

use crate::{DiagnosticCode, LuaSignatureId, LuaType, SemanticModel};

use super::{Checker, DiagnosticContext};

pub struct AwaitInSyncChecker;

impl Checker for AwaitInSyncChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::AwaitInSync];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for call_expr in root.descendants::<LuaCallExpr>() {
            check_call_expr(context, semantic_model, call_expr.clone());
            check_pcall_or_xpcall(context, semantic_model, call_expr);
        }
    }
}

fn check_call_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let function_type = semantic_model.infer_call_expr_func(call_expr.clone(), None)?;
    let is_async = function_type.is_async();

    if is_async {
        let prefix_expr = call_expr.get_prefix_expr()?;
        if !check_call_is_in_async_function(semantic_model, call_expr).unwrap_or(false) {
            context.add_diagnostic(
                DiagnosticCode::AwaitInSync,
                prefix_expr.get_range(),
                t!("Async function can only be called in async function.").to_string(),
                None,
            );
        }
    }

    Some(())
}

fn check_pcall_or_xpcall(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    if let LuaExpr::NameExpr(name_expr) = prefix_expr {
        let name = name_expr.get_name_text()?;
        if name == "pcall" || name == "xpcall" {
            let arg_list = call_expr.get_args_list()?;
            let first_arg = arg_list.get_args().next()?;
            let range = first_arg.get_range();
            let arg_type = semantic_model.infer_expr(first_arg)?;
            let is_async = match &arg_type {
                LuaType::DocFunction(f) => f.is_async(),
                LuaType::Signature(sig) => {
                    let signature = semantic_model.get_db().get_signature_index().get(&sig)?;
                    signature.is_async
                }
                _ => return None,
            };

            if is_async {
                if !check_call_is_in_async_function(semantic_model, call_expr).unwrap_or(false) {
                    context.add_diagnostic(
                        DiagnosticCode::AwaitInSync,
                        range,
                        t!("Async function can only be called in async function.").to_string(),
                        None,
                    );
                }
            }
        }
    }

    Some(())
}

fn check_call_is_in_async_function(
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<bool> {
    let file_id = semantic_model.get_file_id();
    let closures = call_expr.ancestors::<LuaClosureExpr>();
    for closure in closures {
        let signature_id = LuaSignatureId::from_closure(file_id, &closure);
        let is_async = semantic_model
            .get_db()
            .get_signature_index()
            .get(&signature_id)?
            .is_async;
        if is_async {
            return Some(true);
        }

        if !is_in_pcall_or_xpcall(closure).unwrap_or(false) {
            break;
        }
    }

    Some(false)
}

// special case
fn is_in_pcall_or_xpcall(closure: LuaClosureExpr) -> Option<bool> {
    let call_expr = closure
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;
    let prefix_expr = call_expr.get_prefix_expr()?;
    if let LuaExpr::NameExpr(name_expr) = prefix_expr {
        let name = name_expr.get_name_text()?;
        if name == "pcall" || name == "xpcall" {
            return Some(true);
        }
    }

    Some(false)
}
