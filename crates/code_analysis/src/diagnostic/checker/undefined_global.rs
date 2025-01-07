use std::collections::HashSet;

use emmylua_parser::{LuaAstNode, LuaNameExpr};
use rowan::TextRange;

use crate::{DiagnosticCode, LuaMemberKey, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::UndefinedGlobal];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    let mut use_range_set = HashSet::new();
    calc_name_expr_ref(semantic_model, &mut use_range_set);
    for name_expr in root.descendants::<LuaNameExpr>() {
        check_name_expr(context, semantic_model, &mut use_range_set, name_expr);
    }

    Some(())
}

fn calc_name_expr_ref(
    semantic_model: &SemanticModel,
    use_range_set: &mut HashSet<TextRange>,
) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let db = semantic_model.get_db();
    let refs_index = db.get_reference_index().get_local_reference(&file_id)?;
    for (_, ranges) in refs_index.get_local_references_map() {
        for range in ranges {
            use_range_set.insert(range.clone());
        }
    }

    None
}

fn check_name_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    use_range_set: &mut HashSet<TextRange>,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let name_range = name_expr.get_range();
    if use_range_set.contains(&name_range) {
        return Some(());
    }

    let name_text = name_expr.get_name_text()?;
    if name_text == "self" || name_text == "_" {
        return Some(());
    }

    let decl_index = semantic_model.get_db().get_decl_index();
    let member_key = LuaMemberKey::Name(name_text.clone().into());
    if decl_index.get_global_decl_id(&member_key).is_some() {
        return Some(());
    }

    let emmyrc = semantic_model.get_emmyrc();
    if emmyrc.diagnostics.globals.contains(&name_text) {
        return Some(());
    }

    context.add_diagnostic(
        DiagnosticCode::UndefinedGlobal,
        name_range,
        t!("undefined global variable: %{name}", name = name_text).to_string(),
        None,
    );

    Some(())
}
