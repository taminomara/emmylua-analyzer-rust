use emmylua_parser::{
    LuaAssignStat, LuaAstNode, LuaAstToken, LuaForRangeStat, LuaForStat, LuaFuncStat,
    LuaLocalFuncStat, LuaLocalStat, LuaVarExpr,
};

use crate::{
    db_index::{LocalAttribute, LuaDecl, LuaMember, LuaMemberKey, LuaMemberOwner},
    LuaType,
};

use super::DeclAnalyzer;

pub fn analyze_local_stat(analyzer: &mut DeclAnalyzer, stat: LuaLocalStat) {
    let local_name_list = stat.get_local_name_list();
    for local_name in local_name_list {
        let name = if let Some(name_token) = local_name.get_name_token() {
            name_token.get_name_text().to_string()
        } else {
            continue;
        };
        let attrib = if let Some(attrib) = local_name.get_attrib() {
            if attrib.is_const() {
                Some(LocalAttribute::Const)
            } else if attrib.is_close() {
                Some(LocalAttribute::Close)
            } else {
                None
            }
        } else {
            None
        };

        let decl = LuaDecl::Local {
            name,
            file_id: analyzer.get_file_id(),
            range: local_name.get_range(),
            kind: local_name.syntax().kind().into(),
            attrib,
            decl_type: None,
        };
        analyzer.add_decl(decl);
    }
}

pub fn analyze_assign_stat(analyzer: &mut DeclAnalyzer, stat: LuaAssignStat) -> Option<()> {
    let (vars, _) = stat.get_var_and_expr_list();
    for var in vars {
        match &var {
            LuaVarExpr::NameExpr(name) => {
                let name_token = name.get_name_token()?;
                let position = name_token.get_position();
                let name = name_token.get_name_text().to_string();
                if analyzer.find_decl(&name, position).is_none() {
                    let decl = LuaDecl::Global {
                        name,
                        file_id: analyzer.get_file_id(),
                        range: name_token.get_range(),
                        decl_type: None,
                    };

                    analyzer.add_decl(decl);
                }
            }
            LuaVarExpr::IndexExpr(index_expr) => {
                let index_key = index_expr.get_index_key()?;
                let key: LuaMemberKey = index_key.into();
                if key.is_none() {
                    continue;
                }

                // if let Some(LuaVarExpr::NameExpr(name_expr)) = index_expr.get_prefix_expr() {
                //     let name_text = name_expr.get_name_text()?;
                //     if name_text == "_G" || name_text == "_ENV" {
                //         if analyzer
                //             .find_decl(&name_text, index_expr.get_position())
                //             .is_none()
                //         {
                //             let decl = LuaDecl::Global {
                //                 name: name_text.to_string(),
                //                 file_id: analyzer.get_file_id(),
                //                 range: name_expr.get_range(),
                //                 decl_type: None,
                //             };

                //             analyzer.add_decl(decl);
                //         }
                //         continue;
                //     }
                // }

                let file_id = analyzer.get_file_id();
                let member = LuaMember::new(
                    LuaMemberOwner::None,
                    key,
                    file_id,
                    index_expr.get_syntax_id(),
                    None,
                );

                analyzer.db.get_member_index_mut().add_member(member);
            }
        }
    }

    Some(())
}

pub fn analyze_for_stat(analyzer: &mut DeclAnalyzer, stat: LuaForStat) -> Option<()> {
    let it_var = stat.get_var_name()?;
    let name = it_var.get_name_text().to_string();
    let pos = it_var.get_position();
    let range = it_var.get_range();

    if analyzer.find_decl(&name, pos).is_none() {
        let decl = LuaDecl::Local {
            name,
            file_id: analyzer.get_file_id(),
            kind: it_var.syntax().kind(),
            range,
            attrib: Some(LocalAttribute::IterConst),
            decl_type: Some(LuaType::Integer),
        };

        analyzer.add_decl(decl);
    }

    Some(())
}

pub fn analyze_for_range_stat(analyzer: &mut DeclAnalyzer, stat: LuaForRangeStat) {
    let var_list = stat.get_var_name_list();
    for var in var_list {
        let name = var.get_name_text().to_string();
        let range = var.get_range();

        let decl = LuaDecl::Local {
            name,
            file_id: analyzer.get_file_id(),
            kind: var.syntax().kind().into(),
            range,
            attrib: Some(LocalAttribute::IterConst),
            decl_type: None,
        };

        analyzer.add_decl(decl);
    }
}

pub fn analyze_func_stat(analyzer: &mut DeclAnalyzer, stat: LuaFuncStat) -> Option<()> {
    let func_name = stat.get_func_name()?;

    match func_name {
        LuaVarExpr::NameExpr(name_expr) => {
            let name_token = name_expr.get_name_token()?;
            let position = name_token.get_position();
            let name = name_token.get_name_text().to_string();
            if analyzer.find_decl(&name, position).is_none() {
                let decl = LuaDecl::Global {
                    name,
                    file_id: analyzer.get_file_id(),
                    range: name_token.get_range(),
                    decl_type: None,
                };

                analyzer.add_decl(decl);
            }
        }

        LuaVarExpr::IndexExpr(index_name) => {
            let index_key = index_name.get_index_key()?;
            let key: LuaMemberKey = index_key.into();
            if key.is_none() {
                return None;
            }

            let file_id = analyzer.get_file_id();
            let member = LuaMember::new(
                LuaMemberOwner::None,
                key,
                file_id,
                index_name.get_syntax_id(),
                None,
            );

            analyzer.db.get_member_index_mut().add_member(member);
        }
    }

    Some(())
}

pub fn analyze_local_func_stat(analyzer: &mut DeclAnalyzer, stat: LuaLocalFuncStat) {
    if let Some(local_name) = stat.get_local_name() {
        let name = if let Some(name_token) = local_name.get_name_token() {
            name_token.get_name_text().to_string()
        } else {
            return;
        };
        let range = local_name.get_range();
        let decl = LuaDecl::Local {
            name,
            file_id: analyzer.get_file_id(),
            kind: local_name.syntax().kind().into(),
            range,
            attrib: None,
            decl_type: None,
        };

        analyzer.add_decl(decl);
    }
}
