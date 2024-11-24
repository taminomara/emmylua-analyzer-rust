use emmylua_parser::{LuaAstToken, LuaLocalStat};

use crate::db_index::{LuaDeclId, LuaType};

use super::MemberAnalyzer;

pub fn analyze_local_stat(analyzer: &mut MemberAnalyzer, local_stat: LuaLocalStat) -> Option<()> {
    let name_list: Vec<_> = local_stat.get_local_name_list().collect();
    let expr_list: Vec<_> = local_stat.get_value_exprs().collect();

    let mut last_type = LuaType::Unknown;
    let mut last_index = 0;

    for i in 0..name_list.len() {
        let name = name_list.get(i)?;
        let expr = expr_list.get(i);
        let name_token = name.get_name_token()?;
        let position = name_token.get_position();
        let file_id = analyzer.file_id;
        let decl_id = LuaDeclId::new(file_id, position);
        let expr_type = if let Some(expr) = expr {
            let ty =  analyzer.infer_expr(expr);
            match ty {
                Some(ty) => {
                    last_type = ty.clone();
                    last_index = i;
                    if let LuaType::MuliReturn(multi) = ty {
                        multi.get_type(0).unwrap_or(&LuaType::Nil).clone()
                    } else {
                        LuaType::Nil
                    }
                }
                None => {
                    // record unresolve
                    continue;
                }
            }
        } else {
            match &last_type {
                LuaType::MuliReturn(multi) => multi
                    .get_type(i - last_index)
                    .unwrap_or(&LuaType::Nil)
                    .clone(),
                _ => LuaType::Nil,
            }
        };
        // let decl = analyzer
        //     .db
        //     .get_decl_index()
        //     .get_decl_mut(&decl_id)?;

        // if decl.get_type().is_some() {
        //     continue;
        // }

        // if let Some(expr) = expr {
        //     let type_ref = infer_expr(&mut analyzer.db, expr);
        //     decl.set_decl_type(type_ref);
        // }
    }

    Some(())
}

// fn get_decl_type(analyzer: &mut LuaAnalyzer, name)
