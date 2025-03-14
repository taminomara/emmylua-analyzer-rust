use emmylua_parser::{LuaAstNode, LuaAstToken, LuaIndexExpr, LuaStringToken};

use crate::handlers::completion::{
    add_completions::{add_member_completion, CompletionTriggerStatus},
    completion_builder::CompletionBuilder,
};

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let index_expr = LuaIndexExpr::cast(builder.trigger_token.parent()?)?;
    let index_token = index_expr.get_index_token()?;
    let completion_status = if index_token.is_dot() {
        CompletionTriggerStatus::Dot
    } else if index_token.is_colon() {
        CompletionTriggerStatus::Colon
    } else if LuaStringToken::can_cast(builder.trigger_token.kind().into()) {
        CompletionTriggerStatus::InString
    } else {
        CompletionTriggerStatus::LeftBracket
    };

    let prefix_expr = index_expr.get_prefix_expr()?;
    let prefix_type = builder.semantic_model.infer_expr(prefix_expr.into())?;
    let member_infos = builder.semantic_model.infer_member_infos(&prefix_type)?;
    for member_info in member_infos {
        add_member_completion(builder, member_info, completion_status);
    }

    Some(())
}
