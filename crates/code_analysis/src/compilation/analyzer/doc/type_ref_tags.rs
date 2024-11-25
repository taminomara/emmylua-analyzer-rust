use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaDocDescriptionOwner, LuaDocTagModule, LuaDocTagOverload,
    LuaDocTagParam, LuaDocTagReturn, LuaDocTagType, LuaLocalName, LuaVarExpr,
};

use crate::db_index::{
    LuaDeclId, LuaDocParamInfo, LuaDocReturnInfo, LuaMemberId, LuaOperator, LuaPropertyOwnerId,
    LuaSignatureId, LuaType,
};

use super::{
    infer_type::infer_type,
    tags::{find_owner_closure, get_owner_id},
    DocAnalyzer,
};

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
                        let position = name_token.get_position();
                        let file_id = analyzer.file_id;
                        let decl_id = LuaDeclId::new(file_id, position);
                        let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;

                        decl.set_decl_type(type_ref.clone());
                    }
                    LuaVarExpr::IndexExpr(index_expr) => {
                        let member_id =
                            LuaMemberId::new(index_expr.get_syntax_id(), analyzer.file_id);
                        let member = analyzer
                            .db
                            .get_member_index_mut()
                            .get_member_mut(&member_id)?;
                        member.decl_type = type_ref.clone();
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
                let position = name_token.get_position();
                let file_id = analyzer.file_id;
                let decl_id = LuaDeclId::new(file_id, position);
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;

                decl.set_decl_type(type_ref.clone());
            }
        }
        LuaAst::LuaTableField(table_field) => {
            if let Some(first_type) = type_list.get(0) {
                let member_id = LuaMemberId::new(table_field.get_syntax_id(), analyzer.file_id);
                let member = analyzer
                    .db
                    .get_member_index_mut()
                    .get_member_mut(&member_id)?;
                member.decl_type = first_type.clone();
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

    let nullable = tag.is_nullable();
    let mut type_ref = if let Some(lua_doc_type) = tag.get_type() {
        infer_type(analyzer, lua_doc_type)
    } else {
        return None;
    };

    if nullable && !type_ref.is_nullable() {
        type_ref = LuaType::Nullable(type_ref.into());
    }

    let description = if let Some(des) = tag.get_description() {
        Some(des.get_description_text().to_string())
    } else {
        None
    };

    // bind type ref to signature and param
    if let Some(closure) = find_owner_closure(analyzer) {
        let id = LuaSignatureId::new(analyzer.file_id, &closure);
        let signature = analyzer.db.get_signature_index_mut().get_or_create(id);
        let param_info = LuaDocParamInfo {
            name: name.clone(),
            type_ref: type_ref.clone(),
            nullable,
            description,
        };
        signature.param_docs.insert(name.clone(), param_info);

        let param_list = closure.get_params_list()?;
        for param in param_list.get_params() {
            let param_name = if let Some(name_token) = param.get_name_token() {
                name_token.get_name_text().to_string()
            } else {
                "...".to_string()
            };

            if param_name == name {
                let decl_id = LuaDeclId::new(analyzer.file_id, param.get_position());
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;

                decl.set_decl_type(type_ref);
                break;
            }
        }
    } else if let Some(LuaAst::LuaForRangeStat(for_range)) = analyzer.comment.get_owner() {
        for it_name_token in for_range.get_var_name_list() {
            let it_name = it_name_token.get_name_text();
            if it_name == name {
                let decl_id = LuaDeclId::new(analyzer.file_id, it_name_token.get_position());
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;

                decl.set_decl_type(type_ref);
                break;
            }
        }
    }

    Some(())
}

pub fn analyze_return(analyzer: &mut DocAnalyzer, tag: LuaDocTagReturn) -> Option<()> {
    let description = if let Some(des) = tag.get_description() {
        Some(des.get_description_text().to_string())
    } else {
        None
    };
    if let Some(closure) = find_owner_closure(analyzer) {
        let returns = tag.get_type_and_name_list();
        for (doc_type, name_token) in returns {
            let name = if let Some(name) = name_token {
                Some(name.get_name_text().to_string())
            } else {
                None
            };

            let type_ref = infer_type(analyzer, doc_type);
            let return_info = LuaDocReturnInfo {
                name,
                type_ref,
                description: description.clone(),
            };
            let id = LuaSignatureId::new(analyzer.file_id, &closure);
            let signature = analyzer.db.get_signature_index_mut().get_or_create(id);
            signature.return_docs.push(return_info);
        }
    }
    Some(())
}

pub fn analyze_overload(analyzer: &mut DocAnalyzer, tag: LuaDocTagOverload) -> Option<()> {
    if let Some(decl_id) = analyzer.current_type_id.clone() {
        let type_ref = infer_type(analyzer, tag.get_type()?);
        let operator =
            LuaOperator::new_call(decl_id.clone(), type_ref, analyzer.file_id, tag.get_range());
        analyzer.db.get_operator_index_mut().add_operator(operator);
    } else if let Some(closure) = find_owner_closure(analyzer) {
        let type_ref = infer_type(analyzer, tag.get_type()?);
        let id = LuaSignatureId::new(analyzer.file_id, &closure);
        let signature = analyzer.db.get_signature_index_mut().get_or_create(id);
        signature.overloads.push(type_ref);
    }
    Some(())
}

pub fn analyze_module(analyzer: &mut DocAnalyzer, tag: LuaDocTagModule) -> Option<()> {
    let module_path = tag.get_string_token()?.get_value().to_string();
    let decl_type = LuaType::Module(module_path.into());
    if let Some(owner) = get_owner_id(analyzer) {
        match owner {
            LuaPropertyOwnerId::LuaDecl(decl_id) => {
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
                decl.set_decl_type(decl_type);
            }
            _ => {}
        }
    }
    Some(())
}

// pub fn analyze_as(analyzer: &mut DocAnalyzer, tag: LuaDocTagAs) -> Option<()> {
//     // if let Some(owner) = analyzer.comment.get_owner() {
//     //     let type_ref = infer_type(analyzer, tag.get_type_list().get(0)?);
//     //     let id = LuaSignatureId::new(analyzer.file_id, owner.get_position());
//     //     let signature = analyzer.db.get_signature_index().get_or_create(id);
//     //     signature.as_type = Some(type_ref);
//     // }
//     // Some(())
// }
