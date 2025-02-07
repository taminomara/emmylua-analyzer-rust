use crate::{DiagnosticCode, LuaDecl, LuaReferenceIndex, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::Unused];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let decl_tree = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl_tree(&file_id)?;

    let ref_index = semantic_model.get_db().get_reference_index();
    for (_, decl) in decl_tree.get_decls().iter() {
        if !is_decl_used(decl, ref_index) {
            let name = decl.get_name();
            if name.starts_with('_') {
                continue;
            }
            context.add_diagnostic(
                DiagnosticCode::Unused,
                decl.get_range(),
                t!(
                    "%{name} is never used, if this is intentional, prefix it with an underscore: _%{name}",
                    name = name
                ).to_string(),
                None,
            );
        }
    }

    Some(())
}

fn is_decl_used(decl: &LuaDecl, local_refs: &LuaReferenceIndex) -> bool {
    if decl.is_global() {
        return true;
    } else if decl.is_param() {
        match decl.get_name() {
            "self" | "..." => return true,
            _ => {}
        }
    }

    let file_id = decl.get_file_id();
    if let Some(refs) = local_refs.get_decl_references(&file_id, &decl.get_id()) {
        return !refs.is_empty();
    }

    false
}
