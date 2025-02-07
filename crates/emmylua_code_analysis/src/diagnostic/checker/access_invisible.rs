use emmylua_parser::{LuaAst, LuaAstNode, LuaAstToken, LuaIndexExpr, LuaNameExpr, VisibilityKind};
use rowan::TextRange;

use crate::{DiagnosticCode, Emmyrc, LuaDeclId, LuaPropertyOwnerId, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::AccessInvisible];

pub fn check(context: &mut DiagnosticContext, semantic_model: &mut SemanticModel) -> Option<()> {
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
    let decl_id = LuaDeclId::new(semantic_model.get_file_id(), name_expr.get_position());
    if semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)
        .is_some()
    {
        return Some(());
    }

    let property_owner = semantic_model
        .get_property_owner_id(rowan::NodeOrToken::Node(name_expr.syntax().clone()))?;

    let name_token = name_expr.get_name_token()?;
    if !semantic_model.is_property_visiable(name_token.syntax().clone(), property_owner.clone()) {
        let emmyrc = semantic_model.get_emmyrc();
        report_reson(context, &emmyrc, name_token.get_range(), property_owner);
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

    let index_token = index_expr.get_index_name_token()?;
    if !semantic_model.is_property_visiable(index_token.clone(), property_owner.clone()) {
        let emmyrc = semantic_model.get_emmyrc();
        report_reson(context, &emmyrc, index_token.text_range(), property_owner);
    }

    Some(())
}

fn report_reson(
    context: &mut DiagnosticContext,
    emmyrc: &Emmyrc,
    range: TextRange,
    property_owner_id: LuaPropertyOwnerId,
) -> Option<()> {
    let property = context
        .db
        .get_property_index()
        .get_property(property_owner_id)?;

    if let Some(version_conds) = &property.version_conds {
        let version_number = emmyrc.runtime.version.to_lua_version_number();
        let visiable = version_conds.iter().any(|cond| cond.check(&version_number));
        if !visiable {
            let message = t!(
                "The current Lua version %{version} is not accessible; expected %{conds}.",
                version = version_number,
                conds = version_conds
                    .iter()
                    .map(|it| format!("{}", it))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            context.add_diagnostic(
                DiagnosticCode::AccessInvisible,
                range,
                message.to_string(),
                None,
            );
            return Some(());
        }
    }

    if let Some(visiblity) = property.visibility {
        let message = match visiblity {
            VisibilityKind::Protected => {
                t!("The property is protected and cannot be accessed outside its subclasses.")
            }
            VisibilityKind::Private => {
                t!("The property is private and cannot be accessed outside the class.")
            }
            VisibilityKind::Package => {
                t!("The property is package-private and cannot be accessed outside the package.")
            }
            _ => {
                return None;
            }
        };

        context.add_diagnostic(
            DiagnosticCode::AccessInvisible,
            range,
            message.to_string(),
            None,
        );
    }

    Some(())
}
