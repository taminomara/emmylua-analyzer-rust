use emmylua_parser::{
    LuaAst, LuaAstToken, LuaDocDescriptionOwner, LuaDocTagParam, LuaDocTagType, LuaLocalName,
    LuaVarExpr,
};

use super::{infer_type::infer_type, DocAnalyzer};

pub fn analyze_type(analyzer: &mut DocAnalyzer, tag: LuaDocTagType) -> Option<()> {
    let mut type_list = Vec::new();
    for lua_doc_type in tag.get_type_list() {
        let type_ref = infer_type(analyzer, lua_doc_type);
        type_list.push(type_ref);
    }

    // bind ref type
    let owner = analyzer.comment.get_owner()?;
    match owner {
        LuaAst::LuaAssignStat(assign_stat) => {
            let (vars, _) = assign_stat.get_var_and_expr_list();
            let min_len = vars.len().min(type_list.len());
            for i in 0..min_len {
                let var_expr = vars.get(i)?;
                let type_ref = type_list.get(i)?;
                match var_expr {
                    LuaVarExpr::NameExpr(name_expr) => {
                        let name_token = name_expr.get_name_token()?;
                        let name = name_token.get_name_text().to_string();
                        let position = name_token.get_position();
                        let file_id = analyzer.file_id;
                        let decl = analyzer
                            .db
                            .get_decl_index()
                            .get_decl_tree(&file_id)?
                            .find_decl(&name, position)?;
                        let decl_id = decl.get_id();
                        analyzer
                            .db
                            .get_decl_index()
                            .add_decl_type(decl_id, type_ref.clone());
                    }
                    LuaVarExpr::IndexExpr(index_expr) => {
                        analyzer
                            .context
                            .unresolve_index_expr_type
                            .insert(index_expr.clone(), type_ref.clone());
                    }
                }
            }
        }
        LuaAst::LuaLocalStat(local_assign_stat) => {
            let local_list: Vec<LuaLocalName> = local_assign_stat.get_local_name_list().collect();
            let min_len = local_list.len().min(type_list.len());
            for i in 0..min_len {
                let local_name = local_list.get(i)?;
                let type_ref = type_list.get(i)?;
                let name_token = local_name.get_name_token()?;
                let name = name_token.get_name_text().to_string();
                let position = name_token.get_position();
                let file_id = analyzer.file_id;
                let decl = analyzer
                    .db
                    .get_decl_index()
                    .get_decl_tree(&file_id)?
                    .find_decl(&name, position)?;
                let decl_id = decl.get_id();
                analyzer
                    .db
                    .get_decl_index()
                    .add_decl_type(decl_id, type_ref.clone());
            }
        }
        LuaAst::LuaTableField(table_field) => {
            if let Some(first_type) = type_list.get(0) {
                analyzer
                    .context
                    .unresolve_table_field_type
                    .insert(table_field.clone(), first_type.clone());
            }
        }
        _ => {}
    }

    Some(())
}

pub fn analyze_param(analyzer: &mut DocAnalyzer, tag: LuaDocTagParam) -> Option<()> {
    let name = if let Some(name) = tag.get_name_token() {
        name.get_name_text().to_string()
    } else if tag.is_vararg() {
        "...".to_string()
    } else {
        return None;
    };

    let type_ref = if let Some(lua_doc_type) = tag.get_type() {
        infer_type(analyzer, lua_doc_type)
    } else {
        return None;
    };

    let nullable = tag.is_nullable();
    let description = if let Some(des) = tag.get_description() {
        des.get_description_text().to_string()
    } else {
        "".to_string()
    };

    Some(())
}
