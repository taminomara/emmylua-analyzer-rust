use emmylua_parser::LuaExpr;

use crate::db_index::{DbIndex, LuaType};

pub fn infer_expr(db: &mut DbIndex, expr: &LuaExpr) -> Result<LuaType, ()> {
    
    
    Ok(LuaType::Unknown)
}