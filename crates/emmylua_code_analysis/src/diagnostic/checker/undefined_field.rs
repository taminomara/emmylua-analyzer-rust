use emmylua_parser::{LuaAstNode, LuaIndexExpr};

use crate::{DiagnosticCode, LuaMemberOwner, LuaType, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::UndefinedField];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    for index_expr in root.descendants::<LuaIndexExpr>() {
        check_index_expr(context, semantic_model, index_expr);
    }

    Some(())
}

fn check_index_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    index_expr: LuaIndexExpr,
) -> Option<()> {
    let db = context.db;
    let prefix_expr_type = semantic_model.infer_expr(index_expr.get_prefix_expr()?)?;
    match prefix_expr_type {
        LuaType::Ref(id) | LuaType::Def(id) => {
            let member_map = db
                .get_member_index()
                .get_member_map(LuaMemberOwner::Type(id.clone()))?;
            let key = index_expr.get_index_key()?;
            let member = member_map.get(&key.clone().into());
            if member.is_none() {
                context.add_diagnostic(
                    DiagnosticCode::UndefinedField,
                    key.clone().get_range()?,
                    t!("Undefined field: `%{name}`", name = key.get_path_part()).to_string(),
                    None,
                );
            }
        }
        _ => {}
    }

    Some(())
}
