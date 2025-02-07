use emmylua_parser::{LuaAst, LuaAstNode, LuaIndexExpr, LuaNameExpr};

use crate::{DiagnosticCode, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::Unused];

pub fn check(context: &mut DiagnosticContext, semantic_model: &mut SemanticModel) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let root = semantic_model.get_root().clone();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaNameExpr(name_expr) => {
                check_name_expr(context, semantic_model, name_expr);
            }
            LuaAst::LuaIndexExpr(index_expr) => {
                check_index_expr(context, semantic_model, index_expr);
            }
            _ => {}
        }
    }

    Some(())
}

fn check_name_expr(
    context: &mut DiagnosticContext,
    semantic_model: &mut SemanticModel,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let property_owner = semantic_model
        .get_property_owner_id(rowan::NodeOrToken::Node(name_expr.syntax().clone()))?;
    let property = semantic_model
        .get_db()
        .get_property_index()
        .get_property(property_owner)?;
    if property.is_deprecated {
        let depreacated_message = if let Some(message) = &property.deprecated_message {
            message.to_string()
        } else {
            "depreacated".to_string()
        };

        context.add_diagnostic(
            DiagnosticCode::Deprecated,
            name_expr.get_range(),
            depreacated_message,
            None,
        );
    }
    Some(())
}

fn check_index_expr(
    context: &mut DiagnosticContext,
    semantic_model: &mut SemanticModel,
    index_expr: LuaIndexExpr,
) -> Option<()> {
    let property_owner = semantic_model
        .get_property_owner_id(rowan::NodeOrToken::Node(index_expr.syntax().clone()))?;
    let property = semantic_model
        .get_db()
        .get_property_index()
        .get_property(property_owner)?;
    if property.is_deprecated {
        let depreacated_message = if let Some(message) = &property.deprecated_message {
            message.to_string()
        } else {
            "deprecated".to_string()
        };

        let index_name_range = index_expr.get_index_name_token()?.text_range();

        context.add_diagnostic(
            DiagnosticCode::Deprecated,
            index_name_range,
            depreacated_message,
            None,
        );
    }
    Some(())
}
