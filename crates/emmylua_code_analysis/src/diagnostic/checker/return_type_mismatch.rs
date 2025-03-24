use emmylua_parser::{LuaAstNode, LuaClosureExpr, LuaExpr, LuaReturnStat};
use rowan::{NodeOrToken, TextRange};

use crate::{
    humanize_type, DiagnosticCode, LuaSemanticDeclId, LuaSignatureId, LuaType, RenderLevel,
    SemanticModel, SignatureReturnStatus, TypeCheckFailReason, TypeCheckResult,
};

use super::{get_own_return_stats, Checker, DiagnosticContext};

pub struct ReturnTypeMismatch;

impl Checker for ReturnTypeMismatch {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::ReturnTypeMismatch];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for closure_expr in root.descendants::<LuaClosureExpr>() {
            check_closure_expr(context, semantic_model, &closure_expr);
        }
    }
}

fn check_closure_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    closure_expr: &LuaClosureExpr,
) -> Option<()> {
    let signature_id = LuaSignatureId::from_closure(semantic_model.get_file_id(), &closure_expr);
    let signature = context.db.get_signature_index().get(&signature_id)?;
    if signature.resolve_return != SignatureReturnStatus::DocResolve {
        return None;
    }
    let return_types = signature.get_return_types();
    for return_stat in get_own_return_stats(closure_expr) {
        check_return_stat(context, semantic_model, &return_types, &return_stat);
    }
    Some(())
}

fn check_return_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    return_types: &[LuaType],
    return_stat: &LuaReturnStat,
) -> Option<()> {
    let (return_expr_types, return_expr_ranges) = {
        let infos = semantic_model.infer_multi_value_adjusted_expression_types(
            &return_stat.get_expr_list().collect::<Vec<_>>(),
            None,
        )?;
        let mut return_expr_types = infos.iter().map(|(typ, _)| typ.clone()).collect::<Vec<_>>();
        // 解决 setmetatable 的返回值类型问题
        let setmetatable_index = has_setmetatable(semantic_model, return_stat);
        if let Some(setmetatable_index) = setmetatable_index {
            return_expr_types[setmetatable_index] = LuaType::Any;
        }
        let return_expr_ranges = infos
            .iter()
            .map(|(_, range)| range.clone())
            .collect::<Vec<_>>();
        (return_expr_types, return_expr_ranges)
    };

    for (index, return_type) in return_types.iter().enumerate() {
        if let LuaType::Variadic(variadic) = return_type {
            if return_expr_types.len() < index {
                break;
            }
            check_variadic_return_type_match(
                context,
                semantic_model,
                index,
                variadic,
                &return_expr_types[index..],
                &return_expr_ranges[index..],
            );
            break;
        };

        let return_expr_type = return_expr_types.get(index).unwrap_or(&LuaType::Any);
        let result = semantic_model.type_check(return_type, return_expr_type);
        if !result.is_ok() {
            add_type_check_diagnostic(
                context,
                semantic_model,
                index,
                *return_expr_ranges
                    .get(index)
                    .unwrap_or(&return_stat.get_range()),
                return_type,
                return_expr_type,
                result,
            );
        }
    }

    Some(())
}

fn check_variadic_return_type_match(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    start_idx: usize,
    variadic_type: &LuaType,
    return_expr_types: &[LuaType],
    return_expr_ranges: &[TextRange],
) {
    let mut idx = start_idx;
    for (return_expr_type, return_expr_range) in
        return_expr_types.iter().zip(return_expr_ranges.iter())
    {
        let result = semantic_model.type_check(variadic_type, return_expr_type);
        if !result.is_ok() {
            add_type_check_diagnostic(
                context,
                semantic_model,
                start_idx + idx,
                *return_expr_range,
                variadic_type,
                return_expr_type,
                result,
            );
        }
        idx += 1;
    }
}

fn add_type_check_diagnostic(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    index: usize,
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
                    DiagnosticCode::ReturnTypeMismatch,
                    range,
                    t!(
                        "Annotations specify that return value %{index} has a type of `%{source}`, returning value of type `%{found}` here instead. %{reason}.",
                        index = index + 1,
                        source = humanize_type(db, &param_type, RenderLevel::Simple),
                        found = humanize_type(db, &expr_type, RenderLevel::Simple),
                        reason = reason
                    )
                    .to_string(),
                    None,
                );
            }
            TypeCheckFailReason::TypeNotMatch => {
                context.add_diagnostic(
                    DiagnosticCode::ReturnTypeMismatch,
                    range,
                    t!(
                        "Annotations specify that return value %{index} has a type of `%{source}`, returning value of type `%{found}` here instead. %{reason}.",
                        index = index + 1,
                        source = humanize_type(db, &param_type, RenderLevel::Simple),
                        found = humanize_type(db, &expr_type, RenderLevel::Simple),
                        reason = ""
                    )
                    .to_string(),
                    None,
                );
            }
            TypeCheckFailReason::TypeRecursion => {
                context.add_diagnostic(
                    DiagnosticCode::ReturnTypeMismatch,
                    range,
                    "type recursion".into(),
                    None,
                );
            }
        },
    }
}

fn has_setmetatable(semantic_model: &SemanticModel, return_stat: &LuaReturnStat) -> Option<usize> {
    for (index, expr) in return_stat.get_expr_list().enumerate() {
        if let LuaExpr::CallExpr(call_expr) = expr {
            if let Some(prefix_expr) = call_expr.get_prefix_expr() {
                let semantic_info = semantic_model
                    .get_semantic_info(NodeOrToken::Node(prefix_expr.syntax().clone().into()))?;

                if let Some(LuaSemanticDeclId::LuaDecl(decl_id)) = semantic_info.semantic_decl {
                    let decl = semantic_model.get_db().get_decl_index().get_decl(&decl_id);

                    if let Some(decl) = decl {
                        if decl.is_global()
                            && semantic_model
                                .get_db()
                                .get_module_index()
                                .is_std(&decl.get_file_id())
                            && decl.get_name() == "setmetatable"
                        {
                            return Some(index);
                        }
                    }
                }
            }
        }
    }
    None
}
