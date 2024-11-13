use emmylua_parser::{LuaAstNode, LuaClosureExpr, LuaNameExpr};

use crate::db_index::LuaDecl;

use super::DeclAnalyzer;

pub fn analyze_name_expr(analyzer: &mut DeclAnalyzer, expr: LuaNameExpr) {
    let name = expr.get_name_token().map_or_else(
        || "".to_string(),
        |name_token| name_token.get_name_text().to_string(),
    );
    // donot analyze self here
    if name == "self" {
        return;
    }

    let position = expr.get_position();
    let range = expr.get_range();
    let file_id = analyzer.get_file_id();
    let local_decl_id = if let Some(decl) = analyzer.find_decl(&name, position) {
        if decl.is_local() {
            Some(decl.get_id())
        } else {
            if decl.get_position() == position {
                return;
            }
            None
        }
    } else {
        None
    };
    let reference_index = analyzer.db.get_reference_index_mut();

    if let Some(id) = local_decl_id {
        reference_index.add_local_reference(file_id, id, range);
    } else {
        reference_index.add_global_reference(name, range, file_id);
    }
}

pub fn analyze_closure_expr(analyzer: &mut DeclAnalyzer, expr: LuaClosureExpr) {
    let params = expr.get_params_list();
    if params.is_none() {
        return;
    }

    for param in params.unwrap().get_params() {
        let name = param.get_name_token().map_or_else(
            || {
                if param.is_dots() {
                    "...".to_string()
                } else {
                    "".to_string()
                }
            },
            |name_token| name_token.get_name_text().to_string(),
        );

        let range = param.get_range();
        let decl = LuaDecl::Local {
            name,
            file_id: analyzer.get_file_id(),
            range,
            attrib: None,
            decl_type: None,
        };

        analyzer.add_decl(decl);
    }
}
