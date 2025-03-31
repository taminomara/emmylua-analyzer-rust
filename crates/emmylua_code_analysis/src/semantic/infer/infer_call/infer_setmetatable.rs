use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr, LuaIndexKey};

use crate::{
    infer_expr, semantic::infer::InferResult, DbIndex, InFiled, InferFailReason, LuaInferCache,
    LuaType,
};

pub fn infer_setmetatable_call(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    call_expr: LuaCallExpr,
) -> InferResult {
    let arg_list = call_expr.get_args_list().ok_or(InferFailReason::None)?;
    let args = arg_list.get_args().collect::<Vec<LuaExpr>>();

    if args.len() != 2 {
        return Ok(LuaType::Any);
    }

    let basic_table = args[0].clone();
    let metatable = args[1].clone();

    let (meta_type, is_index) = infer_metatable_index_type(db, cache, metatable)?;
    match &basic_table {
        LuaExpr::TableExpr(table_expr) => {
            if table_expr.is_empty() && is_index {
                return Ok(meta_type);
            }

            return Ok(LuaType::TableConst(InFiled::new(
                cache.get_file_id(),
                table_expr.get_range(),
            )));
        }
        _ => {
            if meta_type.is_unknown() {
                return infer_expr(db, cache, basic_table);
            }

            return Ok(meta_type);
        }
    }
}

fn infer_metatable_index_type(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    metatable: LuaExpr,
) -> Result<(LuaType, bool /*__index type*/), InferFailReason> {
    match &metatable {
        LuaExpr::TableExpr(table) => {
            let fields = table.get_fields();
            for field in fields {
                let field_name = match field.get_field_key() {
                    Some(key) => match key {
                        LuaIndexKey::Name(n) => n.get_name_text().to_string(),
                        LuaIndexKey::String(s) => s.get_value(),
                        _ => continue,
                    },
                    None => continue,
                };

                if field_name == "__index" {
                    let field_value = field.get_value_expr().ok_or(InferFailReason::None)?;
                    if matches!(
                        field_value,
                        LuaExpr::TableExpr(_)
                            | LuaExpr::CallExpr(_)
                            | LuaExpr::IndexExpr(_)
                            | LuaExpr::NameExpr(_)
                    ) {
                        let meta_type = infer_expr(db, cache, field_value)?;
                        return Ok((meta_type, true));
                    }
                }
            }
        }
        _ => {}
    };

    let meta_type = infer_expr(db, cache, metatable)?;
    Ok((meta_type, false))
}
