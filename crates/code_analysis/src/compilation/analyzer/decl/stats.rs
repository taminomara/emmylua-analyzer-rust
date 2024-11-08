use emmylua_parser::{
    LuaAssignStat, LuaAstNode, LuaAstToken, LuaForRangeStat, LuaForStat, LuaFuncStat, LuaLocalFuncStat, LuaLocalStat, LuaVarExpr
};

use crate::db_index::{LocalAttribute, LuaDecl};

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
            id: None,
            range: local_name.get_range(),
            attrib,
        };
        analyzer.add_decl(decl);
    }
}

pub fn analyze_assign_stat(analyzer: &mut DeclAnalyzer, stat: LuaAssignStat) {
    let (vars, _) = stat.get_var_and_expr_list();
    for var in vars {
        let name = if let LuaVarExpr::NameExpr(name) = &var {
            name.get_name_token().map_or_else(
                || "".to_string(),
                |name_token| name_token.get_name_text().to_string(),
            )
        } else {
            continue;
        };
        let position = var.get_position();
        if analyzer.find_decl(&name, position).is_none() {
            let decl = LuaDecl::Global {
                name,
                id: None,
                range: var.get_range(),
            };

            analyzer.add_decl(decl);
        }
    }
}

pub fn analyze_for_stat(analyzer: &mut DeclAnalyzer, stat: LuaForStat) {
    let it_var = stat.get_var_name();
    let (name, pos, range) = if let Some(token) = &it_var {
        (
            token.get_name_text().to_string(),
            token.get_position(),
            token.get_range(),
        )
    } else {
        return;
    };

    if analyzer.find_decl(&name, pos).is_none() {
        let decl = LuaDecl::Local {
            name,
            id: None,
            range,
            attrib: Some(LocalAttribute::IterConst),
        };

        analyzer.add_decl(decl);
    }
}

pub fn analyze_for_range_stat(analyzer: &mut DeclAnalyzer, stat: LuaForRangeStat) {
    let var_list = stat.get_var_name_list();
    for var in var_list {
        let name = var.get_name_text().to_string();
        let range = var.get_range();

        let decl = LuaDecl::Local {
            name,
            id: None,
            range,
            attrib: Some(LocalAttribute::IterConst),
        };

        analyzer.add_decl(decl);
    }
}

pub fn analyze_func_stat(analyzer: &mut DeclAnalyzer, stat: LuaFuncStat) {
    if let Some(LuaVarExpr::NameExpr(func_name)) = stat.get_func_name() {
        let name = func_name.get_name_text().unwrap_or_default();
        let range = func_name.get_range();
        let position = func_name.get_position();
        let file_id = analyzer.get_file_id();
        let local_decl_id = if let Some(decl) = analyzer.find_decl(&name, position) {
            match decl {
                LuaDecl::Local { id, .. } => id.clone(),
                _ => None,
            }
        } else {
            None
        };
        let reference_index = analyzer.db.get_reference_index();

        if let Some(id) = local_decl_id {
            reference_index.add_local_reference(file_id, id, range);
        } else {
            reference_index.add_global_reference(name, range, file_id);
        }
    }
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
            id: None,
            range,
            attrib: None,
        };

        analyzer.add_decl(decl);
    }
}