use emmylua_parser::{LuaAstNode, LuaExpr, LuaIndexExpr, PathTrait};
use smol_str::SmolStr;

use crate::LuaMemberOwner;

use super::DeclAnalyzer;

pub fn find_index_owner(
    analyzer: &mut DeclAnalyzer,
    index_expr: LuaIndexExpr,
) -> Option<LuaMemberOwner> {
    if is_in_global_member(analyzer, &index_expr).unwrap_or(false) {
        let access_path = index_expr.get_access_path()?;
        return Some(LuaMemberOwner::GlobalPath(SmolStr::new(&access_path).into()));
    }

    Some(LuaMemberOwner::LocalUnknown)
}

fn is_in_global_member(analyzer: &DeclAnalyzer, index_expr: &LuaIndexExpr) -> Option<bool> {
    let prefix = index_expr.get_prefix_expr()?;
    match prefix {
        LuaExpr::IndexExpr(index_expr) => {
            return is_in_global_member(analyzer, &index_expr);
        }
        LuaExpr::NameExpr(name) => {
            let name_text = name.get_name_text()?;
            if name_text == "self" {
                return Some(false);
            }

            let decl = analyzer.find_decl(&name_text, name.get_position());
            return Some(decl.is_none());
        }
        _ => {}
    }
    None
}
