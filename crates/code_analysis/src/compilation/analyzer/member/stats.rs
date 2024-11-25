use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaExpr, LuaLocalStat, LuaVarExpr};

use crate::{
    compilation::analyzer::unresolve::{
        merge_decl_expr_type, merge_member_type, UnResolveDecl, UnResolveMember,
    },
    db_index::{LuaDeclId, LuaMemberId, LuaMemberOwner, LuaType},
};

use super::MemberAnalyzer;

pub fn analyze_local_stat(analyzer: &mut MemberAnalyzer, local_stat: LuaLocalStat) -> Option<()> {
    let name_list: Vec<_> = local_stat.get_local_name_list().collect();
    let expr_list: Vec<_> = local_stat.get_value_exprs().collect();
    let name_count = name_list.len();
    let expr_count = expr_list.len();
    for i in 0..name_count {
        let name = name_list.get(i)?;
        let position = name.get_position();
        let expr = expr_list.get(i);
        if expr.is_none() {
            break;
        }
        let expr = expr.unwrap();
        let expr_type = analyzer.infer_expr(expr);
        match expr_type {
            Some(expr_type) => {
                let decl_id = LuaDeclId::new(analyzer.file_id, position);
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
                merge_decl_expr_type(decl, expr_type);
            }
            None => {
                let decl_id = LuaDeclId::new(analyzer.file_id, position);
                let unresolve = UnResolveDecl {
                    decl_id,
                    expr: expr.clone(),
                    ret_idx: 0,
                };

                analyzer.add_unresolved(unresolve.into());
            }
        }
    }

    // The complexity brought by multiple return values is too high
    if name_count > expr_count {
        let last_expr = expr_list.last();
        if let Some(last_expr) = last_expr {
            let last_expr_type = analyzer.infer_expr(last_expr);
            if let Some(last_expr_type) = last_expr_type {
                if let LuaType::MuliReturn(multi) = last_expr_type {
                    for i in expr_count..name_count {
                        let name = name_list.get(i)?;
                        let position = name.get_position();
                        let decl_id = LuaDeclId::new(analyzer.file_id, position);
                        let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
                        let ret_type = multi.get_type(i - expr_count + 1);
                        if let Some(ty) = ret_type {
                            merge_decl_expr_type(decl, ty.clone());
                        } else {
                            decl.set_decl_type(LuaType::Unknown);
                        }
                    }
                    return Some(());
                }
            } else {
                for i in expr_count..name_count {
                    let name = name_list.get(i)?;
                    let position = name.get_position();
                    let decl_id = LuaDeclId::new(analyzer.file_id, position);
                    let unresolve = UnResolveDecl {
                        decl_id,
                        expr: last_expr.clone(),
                        ret_idx: i - expr_count + 1,
                    };

                    analyzer.add_unresolved(unresolve.into());
                }
                return Some(());
            }
        }

        for i in expr_count..name_count {
            let name = name_list.get(i)?;
            let position = name.get_position();
            let decl_id = LuaDeclId::new(analyzer.file_id, position);
            let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
            decl.set_decl_type(LuaType::Unknown);
        }
    }

    Some(())
}

#[derive(Debug)]
enum TypeOwner {
    Decl(LuaDeclId),
    Member(LuaMemberId),
}

impl TypeOwner {
    pub fn is_decl(&self) -> bool {
        matches!(self, TypeOwner::Decl(_))
    }

    pub fn is_member(&self) -> bool {
        matches!(self, TypeOwner::Member(_))
    }
}

fn get_var_type_owner(
    analyzer: &mut MemberAnalyzer,
    var: LuaVarExpr,
    expr: LuaExpr,
) -> Option<TypeOwner> {
    let file_id = analyzer.file_id;
    match var {
        LuaVarExpr::NameExpr(var_name) => {
            let position = var_name.get_position();
            let decl_id = LuaDeclId::new(file_id, position);
            let mut decl = analyzer.db.get_decl_index().get_decl(&decl_id);
            if decl.is_none() {
                let decl_tree = analyzer.db.get_decl_index().get_decl_tree(&file_id)?;
                let name = var_name.get_name_text()?;
                decl = decl_tree.find_local_decl(&name, position);
            }

            if decl.is_some() {
                return Some(TypeOwner::Decl(decl.unwrap().get_id()));
            }
        }
        LuaVarExpr::IndexExpr(var_index) => {
            let prefix_expr = var_index.get_prefix_expr()?;
            let prefix_type = analyzer.infer_expr(&prefix_expr.clone().into());
            match prefix_type {
                Some(prefix_type) => {
                    var_index.get_index_key()?;

                    let member_owner = match prefix_type {
                        LuaType::TableConst(in_file_range) => LuaMemberOwner::Table(in_file_range),
                        LuaType::Def(def_id) => LuaMemberOwner::Type(def_id),
                        // is ref need extend field?
                        _ => {
                            return None;
                        }
                    };
                    let member_id = LuaMemberId::new(var_index.get_syntax_id(), file_id);
                    analyzer
                        .db
                        .get_member_index_mut()
                        .add_member_owner(member_owner, member_id);
                    return Some(TypeOwner::Member(member_id));
                }
                None => {
                    // record unresolve
                    let unresolve_member = UnResolveMember {
                        member_id: LuaMemberId::new(var_index.get_syntax_id(), file_id),
                        expr: expr.clone(),
                        prefix: Some(prefix_expr.into()),
                        ret_idx: 0,
                    };
                    analyzer.add_unresolved(unresolve_member.into());
                }
            }
        }
    }

    None
}

// assign stat is too complex
pub fn analyze_assign_stat(
    analyzer: &mut MemberAnalyzer,
    assign_stat: LuaAssignStat,
) -> Option<()> {
    let (var_list, expr_list) = assign_stat.get_var_and_expr_list();
    let expr_count = expr_list.len();
    let var_count = var_list.len();
    for i in 0..expr_count {
        let var = var_list.get(i)?;
        let expr = expr_list.get(i);
        if expr.is_none() {
            break;
        }
        let expr = expr.unwrap();
        let type_owner = match get_var_type_owner(analyzer, var.clone(), expr.clone()) {
            Some(type_owner) => type_owner,
            None => {
                continue;
            }
        };

        let expr_type = match analyzer.infer_expr(expr) {
            Some(expr_type) => match expr_type {
                LuaType::MuliReturn(multi) => multi.get_type(0)?.clone(),
                _ => expr_type,
            },
            None => {
                match type_owner {
                    TypeOwner::Decl(decl_id) => {
                        let decl = analyzer.db.get_decl_index().get_decl(&decl_id)?;
                        let decl_type = decl.get_type();
                        if decl_type.is_none() {
                            let unresolve_decl = UnResolveDecl {
                                decl_id,
                                expr: expr.clone(),
                                ret_idx: 0,
                            };

                            analyzer.add_unresolved(unresolve_decl.into());
                        }
                    }
                    TypeOwner::Member(member_id) => {
                        let unresolve_member = UnResolveMember {
                            member_id,
                            expr: expr.clone(),
                            prefix: None,
                            ret_idx: 0,
                        };
                        analyzer.add_unresolved(unresolve_member.into());
                    }
                }
                continue;
            }
        };

        match type_owner {
            TypeOwner::Decl(decl_id) => {
                let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
                let decl_type = decl.get_type();
                if decl_type.is_none() {
                    decl.set_decl_type(expr_type);
                } else {
                    merge_decl_expr_type(decl, expr_type);
                }
            }
            TypeOwner::Member(member_id) => {
                let member = analyzer
                    .db
                    .get_member_index_mut()
                    .get_member_mut(&member_id)?;
                if member.decl_type.is_unknown() {
                    member.decl_type = expr_type;
                } else {
                    merge_member_type(member, expr_type);
                }
            }
        }
    }

    // The complexity brought by multiple return values is too high
    if var_count > expr_count {
        let last_expr = expr_list.last();
        if let Some(last_expr) = last_expr {
            let last_expr_type = analyzer.infer_expr(last_expr);
            if let Some(last_expr_type) = last_expr_type {
                if let LuaType::MuliReturn(multi) = last_expr_type {
                    // for i in expr_count..var_count {
                    //     let var = var_list.get(i)?;
                    //     let type_owner = get_var_type_owner(analyzer, var.clone(), expr)
                    //     let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
                    //     let ret_type = multi.get_type(i - expr_count + 1);
                    //     if let Some(ty) = ret_type {
                    //         merge_decl_expr_type(decl, ty.clone());
                    //     } else {
                    //         decl.set_decl_type(LuaType::Unknown);
                    //     }
                    // }
                    return Some(());
                }
            } else {
                // for i in expr_count..var_count {
                //     let name = var_list.get(i)?;
                //     let position = name.get_position();
                //     let decl_id = LuaDeclId::new(analyzer.file_id, position);
                //     let unresolve = UnResolveDecl {
                //         decl_id,
                //         expr: last_expr.clone(),
                //         ret_idx: i - expr_count + 1,
                //     };

                //     analyzer.add_unresolved(unresolve.into());
                // }
                return Some(());
            }
        }

        // for i in expr_count..var_count {
        //     let name = var_list.get(i)?;
        //     let position = name.get_position();
        //     let decl_id = LuaDeclId::new(analyzer.file_id, position);
        //     let decl = analyzer.db.get_decl_index_mut().get_decl_mut(&decl_id)?;
        //     decl.set_decl_type(LuaType::Unknown);
        // }
    }

    Some(())
}
