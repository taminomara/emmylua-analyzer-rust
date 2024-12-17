use std::collections::HashSet;

use emmylua_parser::{LuaAstNode, LuaNameExpr};

use crate::handlers::completion::{
    add_completions::add_decl_completion, completion_builder::CompletionBuilder,
};

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    if !LuaNameExpr::can_cast(builder.trigger_token.parent()?.kind().into()) {
        return None;
    }

    let file_id = builder.semantic_model.get_file_id();
    let decl_tree = builder
        .semantic_model
        .get_db()
        .get_decl_index()
        .get_decl_tree(&file_id)?;

    let mut duplicated_name = HashSet::new();
    let local_env = decl_tree.get_env_decls(builder.trigger_token.text_range().start())?;
    for decl_id in local_env.iter() {
        let name = builder
            .semantic_model
            .get_db()
            .get_decl_index()
            .get_decl(decl_id)?
            .get_name().to_string();
        if duplicated_name.contains(&name) {
            continue;
        }
        
        duplicated_name.insert(name);
        add_decl_completion(builder, decl_id.clone());
    }

    Some(())
}
