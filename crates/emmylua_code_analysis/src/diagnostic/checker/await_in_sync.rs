use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaClosureExpr};
use rowan::NodeOrToken;

use crate::{DiagnosticCode, LuaPropertyOwnerId, LuaSignatureId, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::AwaitInSync];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for call_expr in root.descendants::<LuaCallExpr>() {
        check_call_expr(context, semantic_model, call_expr);
    }

    Some(())
}

fn check_call_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let prefix_node = call_expr.get_prefix_expr()?;
    let property_owner =
        semantic_model.get_property_owner_id(NodeOrToken::Node(prefix_node.syntax().clone()))?;

    let property = semantic_model
        .get_db()
        .get_property_index()
        .get_property(property_owner)?;
    if property.is_async {
        if !check_call_is_in_async_function(semantic_model, call_expr).unwrap_or(false) {
            context.add_diagnostic(
                DiagnosticCode::AwaitInSync,
                prefix_node.get_range(),
                "await in sync function".to_string(),
                None,
            );
        }
    }

    Some(())
}

fn check_call_is_in_async_function(
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
) -> Option<bool> {
    let file_id = semantic_model.get_file_id();
    let closure = call_expr.ancestors::<LuaClosureExpr>().next()?;
    let signature_id = LuaSignatureId::from_closure(file_id, &closure);
    let property_owner = LuaPropertyOwnerId::Signature(signature_id);
    let property = semantic_model
        .get_db()
        .get_property_index()
        .get_property(property_owner)?;
    Some(property.is_async)
}
