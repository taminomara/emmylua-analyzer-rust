use crate::{DiagnosticCode, LocalAttribute, LuaDeclExtra, LuaDeclId, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[
    DiagnosticCode::LocalConstReassign,
    DiagnosticCode::IterVariableReassign,
];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let decl_tree = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl_tree(&file_id)?;
    for (decl_id, decl) in decl_tree.get_decls() {
        match &decl.extra {
            LuaDeclExtra::Local { attrib, .. } => {
                if let Some(attrib) = attrib {
                    if matches!(attrib, LocalAttribute::Const | LocalAttribute::IterConst) {
                        check_local_const_reassign(context, semantic_model, decl_id, &attrib);
                    }
                }
            }
            _ => {}
        }
    }

    Some(())
}

fn check_local_const_reassign(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    decl_id: &LuaDeclId,
    attrib: &LocalAttribute,
) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let refs_index = semantic_model.get_db().get_reference_index();
    let local_refs = refs_index.get_local_reference(&file_id)?;
    let ranges = local_refs.get_local_references(decl_id)?;
    for range in ranges {
        if refs_index.is_write_range(file_id, *range) {
            match attrib {
                LocalAttribute::Const => {
                    context.add_diagnostic(
                        DiagnosticCode::LocalConstReassign,
                        *range,
                        t!("Cannot reassign to a constant variable").to_string(),
                        None,
                    );
                }
                LocalAttribute::IterConst => {
                    context.add_diagnostic(
                        DiagnosticCode::IterVariableReassign,
                        *range,
                        t!("Should not reassign to iter variable").to_string(),
                        None,
                    );
                }
                _ => {}
            }
        }
    }

    Some(())
}
