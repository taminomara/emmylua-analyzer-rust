use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaClosureExpr, LuaIndexExpr, LuaIndexKey, LuaLiteralExpr,
    LuaLiteralToken, LuaNameExpr, LuaTableExpr,
};

use crate::{
    db_index::{LuaDecl, LuaMember, LuaMemberKey, LuaMemberOwner},
    InFiled, LuaDeclExtra, LuaDeclId, LuaSignatureId,
};

use super::DeclAnalyzer;

pub fn analyze_name_expr(analyzer: &mut DeclAnalyzer, expr: LuaNameExpr) -> Option<()> {
    let name_token = expr.get_name_token()?;
    let name = name_token.get_name_text();
    // donot analyze self here
    if name == "self" {
        return Some(());
    }

    let position = name_token.get_position();
    let range = name_token.get_range();
    let file_id = analyzer.get_file_id();
    let decl_id = LuaDeclId::new(file_id, position);
    let (decl_id, is_local) = if analyzer.decl.get_decl(&decl_id).is_some() {
        (Some(decl_id), false)
    } else if let Some(decl) = analyzer.find_decl(&name, position) {
        if decl.is_local() {
            // reference local variable
            (Some(decl.get_id()), true)
        } else {
            if decl.get_position() == position {
                return Some(());
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

    Some(())
}

pub fn analyze_index_expr(analyzer: &mut DeclAnalyzer, expr: LuaIndexExpr) -> Option<()> {
    let index_key = expr.get_index_key()?;
    let key = match index_key {
        LuaIndexKey::Name(name) => LuaMemberKey::Name(name.get_name_text().to_string().into()),
        LuaIndexKey::Integer(int) => {
            if int.is_int() {
                LuaMemberKey::Integer(int.get_int_value())
            } else {
                return None;
            }
        }
        LuaIndexKey::String(string) => LuaMemberKey::Name(string.get_value().into()),
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

pub fn analyze_closure_expr(analyzer: &mut DeclAnalyzer, expr: LuaClosureExpr) -> Option<()> {
    let params = expr.get_params_list()?;
    let signature_id = LuaSignatureId::new(analyzer.get_file_id(), &expr);
    let file_id = analyzer.get_file_id();
    for (idx, param) in params.get_params().enumerate() {
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

        let decl = LuaDecl::new(
            &name,
            file_id,
            range,
            LuaDeclExtra::Param { idx, signature_id },
        );

        analyzer.add_decl(decl);
    }

    analyze_closure_params(analyzer, &signature_id, &expr);

    Some(())
}

fn analyze_closure_params(
    analyzer: &mut DeclAnalyzer,
    signature_id: &LuaSignatureId,
    closure: &LuaClosureExpr,
) -> Option<()> {
    let signature = analyzer
        .db
        .get_signature_index_mut()
        .get_or_create(signature_id.clone());
    let params = closure.get_params_list()?.get_params();
    for param in params {
        let name = if let Some(name_token) = param.get_name_token() {
            name_token.get_name_text().to_string()
        } else if param.is_dots() {
            "...".to_string()
        } else {
            return None;
        };

        signature.params.push(name);
    }

    Some(())
}

pub fn analyze_table_expr(analyzer: &mut DeclAnalyzer, expr: LuaTableExpr) -> Option<()> {
    if expr.is_object() {
        let file_id = analyzer.get_file_id();
        let owner_id = LuaMemberOwner::Element(InFiled {
            file_id,
            value: expr.get_range(),
        });

        for field in expr.get_fields() {
            if let Some(field_key) = field.get_field_key() {
                let key: LuaMemberKey = field_key.into();
                if key.is_none() {
                    continue;
                }

                analyzer.db.get_reference_index_mut().add_index_reference(
                    key.clone(),
                    file_id,
                    field.get_syntax_id(),
                );

                let member =
                    LuaMember::new(owner_id.clone(), key, file_id, field.get_syntax_id(), None);
                analyzer.db.get_member_index_mut().add_member(member);
            }
        }
    }

    Some(())
}

pub fn analyze_literal_expr(analyzer: &mut DeclAnalyzer, expr: LuaLiteralExpr) -> Option<()> {
    if let LuaLiteralToken::String(string_token) = expr.get_literal()? {
        let file_id = analyzer.get_file_id();
        let value = string_token.get_value();
        if value.len() > 64 {
            return Some(());
        }

        analyzer.db.get_reference_index_mut().add_string_reference(
            file_id,
            &value,
            string_token.get_range(),
        );
    }

    Some(())
}
