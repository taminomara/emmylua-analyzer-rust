use emmylua_parser::{LuaAstNode, LuaExpr, LuaTableExpr};

use crate::db_index::{DbIndex, LuaType};

use super::{InferResult, LuaInferConfig};

pub fn infer_table_expr(_: &DbIndex, config: &LuaInferConfig, table: LuaTableExpr) -> InferResult {
    Some(LuaType::TableConst(crate::InFiled {
        file_id: config.get_file_id(),
        value: table.get_range(),
    }))
}
