use emmylua_parser::{
    LuaAstNode, LuaClosureExpr, LuaIndexExpr, LuaIndexKey, LuaNameExpr, LuaTableExpr,
};

use crate::{
    db_index::{LuaDecl, LuaMember, LuaMemberKey, LuaMemberOwner, LuaReferenceKey},
    InFiled,
};

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
    let (decl_id, is_local) = if let Some(decl) = analyzer.find_decl(&name, position) {
        if decl.is_local() {
            // reference local variable
            (Some(decl.get_id()), true)
        } else {
            if decl.get_position() == position {
                return;
            }
            // reference in filed global variable
            (Some(decl.get_id()), false)
        }
    } else {
        (None, false)
    };

    let reference_index = analyzer.db.get_reference_index_mut();

    if let Some(id) = decl_id {
        reference_index.add_local_reference(id, file_id, range);
    }

    if !is_local {
        reference_index.add_global_reference(name, file_id, range);
    }
}

pub fn analyze_index_expr(analyzer: &mut DeclAnalyzer, expr: LuaIndexExpr) -> Option<()> {
    let index_key = expr.get_index_key()?;
    let key = match index_key {
        LuaIndexKey::Name(name) => LuaReferenceKey::Name(name.get_name_text().to_string().into()),
        LuaIndexKey::Integer(int) => {
            if int.is_int() {
                LuaReferenceKey::Integer(int.get_int_value())
            } else {
                return None;
            }
        }
        LuaIndexKey::String(string) => LuaReferenceKey::Name(string.get_value().into()),
        LuaIndexKey::Expr(_) => return None,
    };

    let file_id = analyzer.get_file_id();
    let syntax_id = expr.get_syntax_id();
    analyzer
        .db
        .get_reference_index_mut()
        .add_index_reference(key, file_id, syntax_id);
    Some(())
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
            kind: param.syntax().kind().into(),
            range,
            attrib: None,
            decl_type: None,
        };

        analyzer.add_decl(decl);
    }
}

pub fn analyze_table_expr(analyzer: &mut DeclAnalyzer, expr: LuaTableExpr) -> Option<()> {
    if expr.is_object() {
        let file_id = analyzer.get_file_id();
        let owner_id = LuaMemberOwner::Table(InFiled {
            file_id,
            value: expr.get_range(),
        });

        for field in expr.get_fields() {
            if let Some(field_key) = field.get_field_key() {
                let key = match field_key {
                    LuaIndexKey::Name(name) => {
                        LuaReferenceKey::Name(name.get_name_text().to_string().into())
                    }
                    LuaIndexKey::Integer(int) => LuaReferenceKey::Integer(int.get_int_value()),
                    LuaIndexKey::String(string) => LuaReferenceKey::Name(string.get_value().into()),
                    LuaIndexKey::Expr(_) => return None,
                };

                analyzer.db.get_reference_index_mut().add_index_reference(
                    key.clone(),
                    file_id,
                    expr.get_syntax_id(),
                );
                let member_key = match key {
                    LuaReferenceKey::Name(name) => LuaMemberKey::Name(name),
                    LuaReferenceKey::Integer(int) => LuaMemberKey::Integer(int),
                };

                let member = LuaMember::new(
                    owner_id.clone(),
                    member_key,
                    file_id,
                    expr.get_syntax_id(),
                    None,
                );
                analyzer.db.get_member_index_mut().add_member(member);
            }
        }
    }

    Some(())
}
