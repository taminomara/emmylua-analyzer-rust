use emmylua_parser::{LuaAstToken, LuaLocalStat};

use super::LuaAnalyzer;

pub fn analyze_local_stat(analyzer: &mut LuaAnalyzer, local_stat: LuaLocalStat) -> Option<()> {
    let name_list = local_stat.get_local_name_list();
    for name in name_list {
        let (name, position) = if let Some(name_token) = name.get_name_token() {
            (
                name_token.get_name_text().to_string(),
                name_token.get_position(),
            )
        } else {
            continue;
        };

        // let decl_id = decl.get_id();
        // let lua_type = analyzer.db.get_decl_index().get_decl_type(&decl_id);

    }

    Some(())
}

// fn get_decl_type(analyzer: &mut LuaAnalyzer, name)