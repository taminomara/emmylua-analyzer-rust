use emmylua_parser::{LuaAstNode, LuaCallExprStat};
use rowan::NodeOrToken;

use crate::{DiagnosticCode, LuaSemanticDeclId, SemanticDeclLevel, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::DiscardReturns];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for call_expr_stat in root.descendants::<LuaCallExprStat>() {
        check_call_expr(context, semantic_model, call_expr_stat);
    }

    Some(())
}

fn check_call_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr_stat: LuaCallExprStat,
) -> Option<()> {
    let call_expr = call_expr_stat.get_call_expr()?;
    let prefix_node = call_expr.get_prefix_expr()?.syntax().clone();
    let semantic_decl = semantic_model.find_decl(
        NodeOrToken::Node(prefix_node.clone()),
        SemanticDeclLevel::default(),
    )?;

    if let LuaSemanticDeclId::Signature(signature_id) = semantic_decl {
        let signature = semantic_model
            .get_db()
            .get_signature_index()
            .get(&signature_id)?;
        if signature.is_nodiscard {
            context.add_diagnostic(
                DiagnosticCode::DiscardReturns,
                prefix_node.text_range(),
                "discard returns".to_string(),
                None,
            );
        }
    }

    Some(())
}
