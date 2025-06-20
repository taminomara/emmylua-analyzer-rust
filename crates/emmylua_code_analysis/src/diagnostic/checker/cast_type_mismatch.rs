use emmylua_parser::{LuaAst, LuaAstNode, LuaDocTagCast};
use rowan::TextRange;

use crate::diagnostic::checker::generic::infer_doc_type::infer_doc_type;
use crate::{DiagnosticCode, LuaType, SemanticModel, TypeCheckFailReason, TypeCheckResult};

use super::{humanize_lint_type, Checker, DiagnosticContext};

pub struct CastTypeMismatchChecker;

impl Checker for CastTypeMismatchChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::CastTypeMismatch];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        // dbg!(&semantic_model.get_root());
        for node in semantic_model.get_root().descendants::<LuaAst>() {
            if let LuaAst::LuaDocTagCast(cast_tag) = node {
                check_cast_tag(context, semantic_model, &cast_tag);
            }
        }
    }
}

fn check_cast_tag(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    cast_tag: &LuaDocTagCast,
) -> Option<()> {
    let key_expr = cast_tag.get_key_expr()?;
    let origin_type = semantic_model.infer_expr(key_expr).ok()?;

    // 检查每个 cast 操作类型
    for op_type in cast_tag.get_op_types() {
        // 如果具有操作符, 则不检查
        if let Some(_) = op_type.get_op() {
            continue;
        }
        if let Some(target_doc_type) = op_type.get_type() {
            let target_type = infer_doc_type(semantic_model, &target_doc_type);
            check_cast_compatibility(
                context,
                semantic_model,
                op_type.get_range(),
                &origin_type,
                &target_type,
            );
        }
    }

    Some(())
}

fn check_cast_compatibility(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    range: TextRange,
    original_type: &LuaType,
    target_type: &LuaType,
) -> Option<()> {
    // 如果类型相同，则无需检查
    if original_type == target_type {
        return Some(());
    }

    // 检查是否可以从原始类型转换为目标类型
    let result = can_cast_type(semantic_model, original_type, target_type);

    if !result.is_ok() {
        add_cast_type_mismatch_diagnostic(
            context,
            semantic_model,
            range,
            original_type,
            target_type,
            result,
        );
    }

    Some(())
}

fn can_cast_type(
    semantic_model: &SemanticModel,
    original_type: &LuaType,
    target_type: &LuaType,
) -> TypeCheckResult {
    if let LuaType::Union(union_type) = original_type {
        for member_type in union_type.get_types() {
            if semantic_model.type_check(target_type, member_type).is_ok() {
                return Ok(());
            }
        }
        return Err(TypeCheckFailReason::TypeNotMatch);
    }

    semantic_model.type_check(target_type, original_type)
}

fn add_cast_type_mismatch_diagnostic(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    range: TextRange,
    original_type: &LuaType,
    target_type: &LuaType,
    result: TypeCheckResult,
) {
    let db = semantic_model.get_db();
    match result {
        Ok(_) => return,
        Err(reason) => match reason {
            TypeCheckFailReason::TypeNotMatchWithReason(reason) => {
                context.add_diagnostic(
                    DiagnosticCode::CastTypeMismatch,
                    range,
                    t!(
                        "Cannot cast `%{original}` to `%{target}`. %{reason}",
                        original = humanize_lint_type(db, original_type),
                        target = humanize_lint_type(db, target_type),
                        reason = reason
                    )
                    .to_string(),
                    None,
                );
            }
            _ => {
                context.add_diagnostic(
                    DiagnosticCode::CastTypeMismatch,
                    range,
                    t!(
                        "Cannot cast `%{original}` to `%{target}`. %{reason}",
                        original = humanize_lint_type(db, original_type),
                        target = humanize_lint_type(db, target_type),
                        reason = ""
                    )
                    .to_string(),
                    None,
                );
            }
        },
    }
}
