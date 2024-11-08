use std::collections::HashMap;

use emmylua_parser::{
    LuaAssignStat, LuaAst, LuaAstNode, LuaAstToken, LuaCommentOwner, LuaDocDescriptionOwner,
    LuaDocDetailOwner, LuaDocGenericDeclList, LuaDocTagAlias, LuaDocTagClass, LuaDocTagEnum,
    LuaDocTagGeneric, LuaFuncStat, LuaLocalName, LuaLocalStat, LuaNameExpr, LuaSyntaxId,
    LuaSyntaxKind, LuaTokenKind, LuaVarExpr,
};
use rowan::TextRange;

use crate::db_index::{
    LuaDeclTypeKind, LuaDescription, LuaDescriptionOwnerId, LuaMember, LuaMemberOwner, LuaType,
};

use super::{infer_type::infer_type, DocAnalyzer};

pub fn analyze_class(analyzer: &mut DocAnalyzer, tag: LuaDocTagClass) -> Option<()> {
    let file_id = analyzer.file_id;
    let name = tag.get_name_token()?.get_name_text().to_string();

    let class_decl = analyzer
        .db
        .get_type_index()
        .find_type_decl(file_id, &name)?;

    if class_decl.get_kind() != LuaDeclTypeKind::Class {
        return None;
    }
    let class_decl_id = class_decl.get_id();
    analyzer.current_type_id = Some(class_decl_id.clone());
    if let Some(generic_params) = tag.get_generic_decl() {
        let params = get_generic_params(analyzer, generic_params);
        let mut params_index = HashMap::new();
        let mut count = 0;
        for (name, _) in params.iter() {
            params_index.insert(name.clone(), count);
            count += 1;
        }

        analyzer
            .db
            .get_type_index()
            .add_generic_params(class_decl_id.clone(), params);

        add_generic_index(analyzer, params_index);
    }

    if let Some(supers) = tag.get_supers() {
        for super_doc_type in supers.get_types() {
            let super_type = infer_type(analyzer, super_doc_type);
            if super_type.is_unknown() {
                continue;
            }

            analyzer
                .db
                .get_type_index()
                .add_super_type(class_decl_id.clone(), file_id, super_type);
        }
    }

    if let Some(description) = tag.get_description() {
        let description_text = description.get_description_text();
        if !description_text.is_empty() {
            let description = LuaDescription::new(
                LuaDescriptionOwnerId::TypeDecl(class_decl_id.clone()),
                description_text,
            );
            analyzer
                .db
                .get_description_index()
                .add_description(file_id, description);
        }
    }

    bind_def_type(analyzer, LuaType::Def(class_decl_id.clone()));
    Some(())
}

pub fn analyze_enum(analyzer: &mut DocAnalyzer, tag: LuaDocTagEnum) -> Option<()> {
    let file_id = analyzer.file_id;
    let name = tag.get_name_token()?.get_name_text().to_string();

    let enum_decl = analyzer
        .db
        .get_type_index()
        .find_type_decl(file_id, &name)?;
    if enum_decl.get_kind() != LuaDeclTypeKind::Enum {
        return None;
    }

    let enum_decl_id = enum_decl.get_id();
    analyzer.current_type_id = Some(enum_decl_id.clone());

    if let Some(base_type) = tag.get_base_type() {
        let super_type = infer_type(analyzer, base_type);
        if super_type.is_unknown() {
            return None;
        }

        analyzer
            .db
            .get_type_index()
            .add_super_type(enum_decl_id.clone(), file_id, super_type);
    }

    if let Some(description) = tag.get_description() {
        let description_text = description.get_description_text();
        if !description_text.is_empty() {
            let description = LuaDescription::new(
                LuaDescriptionOwnerId::TypeDecl(enum_decl_id.clone()),
                description_text,
            );
            analyzer
                .db
                .get_description_index()
                .add_description(file_id, description);
        }
    }

    bind_def_type(analyzer, LuaType::Def(enum_decl_id.clone()));

    Some(())
}

pub fn analyze_alias(analyzer: &mut DocAnalyzer, tag: LuaDocTagAlias) -> Option<()> {
    let file_id = analyzer.file_id;
    let name = tag.get_name_token()?.get_name_text().to_string();

    let alias_decl = analyzer
        .db
        .get_type_index()
        .find_type_decl(file_id, &name)?;
    if alias_decl.get_kind() != LuaDeclTypeKind::Alias {
        return None;
    }

    let alias_decl_id = alias_decl.get_id();
    if let Some(generic_params) = tag.get_generic_decl_list() {
        let params = get_generic_params(analyzer, generic_params);
        let mut params_index = HashMap::new();
        let mut count = 0;
        for (name, _) in params.iter() {
            params_index.insert(name.clone(), count);
            count += 1;
        }

        analyzer
            .db
            .get_type_index()
            .add_generic_params(alias_decl_id.clone(), params);
        let range = analyzer.comment.get_range();
        analyzer
            .generic_index
            .add_generic_scope(vec![range], params_index);
    }

    if let Some(super_type) = tag.get_type() {
        let super_type = infer_type(analyzer, super_type);
        if super_type.is_unknown() {
            return None;
        }

        analyzer
            .db
            .get_type_index()
            .add_super_type(alias_decl_id.clone(), file_id, super_type);
    } else if let Some(field_list) = tag.get_alias_fields() {
        for (i, field) in field_list.get_fields().enumerate() {
            let alias_member_type = if let Some(field_type) = field.get_type() {
                let type_ref = infer_type(analyzer, field_type);
                if type_ref.is_unknown() {
                    continue;
                }
                type_ref
            } else {
                continue;
            };

            let member = LuaMember::new(
                LuaMemberOwner::Type(alias_decl_id.clone()),
                format!("[{}]", i),
                file_id,
                field.syntax().kind(),
                field.get_range(),
                Some(alias_member_type),
            );
            let member_id = analyzer.db.get_member_index().add_member(member);

            if let Some(description_text) = field.get_detail_text() {
                if description_text.is_empty() {
                    continue;
                }

                let description =
                    LuaDescription::new(LuaDescriptionOwnerId::Member(member_id), description_text);
                analyzer
                    .db
                    .get_description_index()
                    .add_description(file_id, description);
            }
        }
    }

    let description_text = tag.get_description()?.get_description_text();
    if description_text.is_empty() {
        return None;
    }
    let description = LuaDescription::new(
        LuaDescriptionOwnerId::TypeDecl(alias_decl_id.clone()),
        description_text,
    );
    analyzer
        .db
        .get_description_index()
        .add_description(file_id, description);

    Some(())
}

fn get_generic_params(
    analyzer: &mut DocAnalyzer,
    params: LuaDocGenericDeclList,
) -> Vec<(String, Option<LuaType>)> {
    let mut params_result = Vec::new();
    for param in params.get_generic_decl() {
        let name = if let Some(param) = param.get_name_token() {
            param.get_name_text().to_string()
        } else {
            continue;
        };

        let type_ref = if let Some(type_ref) = param.get_type() {
            Some(infer_type(analyzer, type_ref))
        } else {
            None
        };

        params_result.push((name, type_ref));
    }

    params_result
}

fn add_generic_index(analyzer: &mut DocAnalyzer, params_index: HashMap<String, usize>) {
    let mut ranges = Vec::new();
    let range = analyzer.comment.get_range();
    ranges.push(range);
    if let Some(comment_owner) = analyzer.comment.get_owner() {
        let range = comment_owner.get_range();
        ranges.push(range);
        match comment_owner {
            LuaAst::LuaLocalStat(local_stat) => {
                if let Some(result) = get_local_stat_reference_ranges(analyzer, local_stat) {
                    ranges.extend(result);
                }
            }
            LuaAst::LuaAssignStat(assign_stat) => {
                if let Some(result) = get_global_reference_ranges(analyzer, assign_stat) {
                    ranges.extend(result);
                }
            }
            _ => {}
        }
    }

    analyzer
        .generic_index
        .add_generic_scope(ranges, params_index);
}

fn get_local_stat_reference_ranges(
    analyzer: &mut DocAnalyzer,
    local_stat: LuaLocalStat,
) -> Option<Vec<TextRange>> {
    let file_id = analyzer.file_id;
    let first_local = local_stat.child::<LuaLocalName>()?;
    let first_local_name = first_local.get_name_token()?.get_name_text().to_string();
    let decl = analyzer
        .db
        .get_decl_index()
        .get_decl_tree(&file_id)?
        .find_decl(&first_local_name, first_local.get_position())?;
    let mut ranges = Vec::new();

    let id = decl.get_id();
    let refs = analyzer
        .db
        .get_reference_index()
        .get_local_references(&file_id, &id)?;
    for reference_range in refs {
        let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr, reference_range.clone());
        let name_node = syntax_id.to_node_from_root(&analyzer.root)?;
        if let Some(parent1) = name_node.parent() {
            if parent1.kind() == LuaSyntaxKind::IndexExpr.into() {
                if let Some(parent2) = parent1.parent() {
                    if parent2.kind() == LuaSyntaxKind::FuncStat.into() {
                        ranges.push(parent2.text_range());
                        let stat = LuaFuncStat::cast(parent2)?;
                        for comment in stat.get_comments() {
                            ranges.push(comment.get_range());
                        }
                    } else if parent2.kind() == LuaSyntaxKind::AssignStat.into() {
                        let stat = LuaAssignStat::cast(parent2)?;
                        if let Some(assign_token) = stat.token_by_kind(LuaTokenKind::TkAssign) {
                            if assign_token.get_position() > reference_range.start() {
                                ranges.push(stat.get_range());
                                for comment in stat.get_comments() {
                                    ranges.push(comment.get_range());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(ranges)
}

fn get_global_reference_ranges(
    analyzer: &mut DocAnalyzer,
    assign_stat: LuaAssignStat,
) -> Option<Vec<TextRange>> {
    let file_id = analyzer.file_id;
    let name_token = assign_stat.child::<LuaNameExpr>()?.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    let mut ranges = Vec::new();

    let refs = analyzer
        .db
        .get_reference_index()
        .get_global_file_references(&name, file_id)?;
    for reference_range in refs {
        let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr, reference_range.clone());
        let name_node = syntax_id.to_node_from_root(&analyzer.root)?;
        if let Some(parent1) = name_node.parent() {
            if parent1.kind() == LuaSyntaxKind::IndexExpr.into() {
                if let Some(parent2) = parent1.parent() {
                    if parent2.kind() == LuaSyntaxKind::FuncStat.into() {
                        ranges.push(parent2.text_range());
                        let stat = LuaFuncStat::cast(parent2)?;
                        for comment in stat.get_comments() {
                            ranges.push(comment.get_range());
                        }
                    } else if parent2.kind() == LuaSyntaxKind::AssignStat.into() {
                        let stat = LuaAssignStat::cast(parent2)?;
                        if let Some(assign_token) = stat.token_by_kind(LuaTokenKind::TkAssign) {
                            if assign_token.get_position() > reference_range.start() {
                                ranges.push(stat.get_range());
                                for comment in stat.get_comments() {
                                    ranges.push(comment.get_range());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(ranges)
}

pub fn analyze_func_generic(analyzer: &mut DocAnalyzer, tag: LuaDocTagGeneric) -> Option<()> {
    if let Some(comment_owner) = analyzer.comment.get_owner() {
        if !matches!(
            comment_owner.syntax().kind().into(),
            LuaSyntaxKind::LocalFuncStat | LuaSyntaxKind::FuncStat
        ) {
            return None;
        }

        let mut params_result = HashMap::new();
        if let Some(params_list) = tag.get_generic_decl_list() {
            let mut count = 0;
            for param in params_list.get_generic_decl() {
                let name = if let Some(param) = param.get_name_token() {
                    param.get_name_text().to_string()
                } else {
                    continue;
                };

                // let type_ref = if let Some(type_ref) = param.get_type() {
                //     Some(infer_type(analyzer, type_ref))
                // } else {
                //     None
                // };

                params_result.insert(name, count);
                count += 1;
            }
        }

        let mut ranges = Vec::new();
        let range = analyzer.comment.get_range();
        ranges.push(range);
        let range = comment_owner.get_range();
        ranges.push(range);
        analyzer
            .generic_index
            .add_generic_scope(ranges, params_result);
    }

    Some(())
}

fn bind_def_type(analyzer: &mut DocAnalyzer, type_def: LuaType) -> Option<()> {
    let owner = analyzer.comment.get_owner()?;
    match owner {
            LuaAst::LuaLocalStat(local_stat) => {
                let local_name = local_stat.child::<LuaLocalName>()?;
                let position= local_name.get_position();
                let name = local_name.get_name_token()?.get_name_text().to_string();
                let file_id = analyzer.file_id;
                let decl = analyzer
                    .db
                    .get_decl_index()
                    .get_decl_tree(&file_id)?
                    .find_decl(&name, position)?;
                let id = decl.get_id();
                analyzer
                    .db
                    .get_decl_index()
                    .add_decl_type(id, type_def);
            }
            LuaAst::LuaAssignStat(assign_stat) => {
                if let LuaVarExpr::NameExpr(name_expr) = assign_stat.child::<LuaVarExpr>()? {
                    let name = name_expr.get_name_token()?.get_name_text().to_string();
                    let position = name_expr.get_position();
                    let file_id = analyzer.file_id;
                    let decl = analyzer
                        .db
                        .get_decl_index()
                        .get_decl_tree(&file_id)?
                        .find_decl(&name, position)?;
                    let id = decl.get_id();
                    analyzer
                        .db
                        .get_decl_index()
                        .add_decl_type(id, type_def);
                } else if let LuaVarExpr::IndexExpr(index_expr) = assign_stat.child::<LuaVarExpr>()? {
                    analyzer.context.unresolve_index_expr_type.insert(index_expr, type_def);
                }
            }
            LuaAst::LuaTableField(field) => {
                analyzer.context.unresolve_table_field_type.insert(field, type_def);
            }
            _ => {}
        }
    Some(())
}
