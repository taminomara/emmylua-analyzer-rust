use emmylua_parser::LuaCallExpr;

use crate::{db_index::{DbIndex, LuaType}, FileId};


pub fn instantiate_func(
    db: &mut DbIndex,
    file_id: FileId,
    call_expr: LuaCallExpr,
    type_args: Vec<LuaType>,
    return_types: Vec<LuaType>,
) -> Option<()> {

    todo!()
}
