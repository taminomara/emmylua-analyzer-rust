use emmylua_parser::{LuaAstNode, LuaExpr, LuaLiteralToken, LuaTableExpr};

use crate::{
    db_index::{DbIndex, LuaType},
    infer_expr, LuaTupleType,
};

use super::{InferResult, LuaInferConfig};

pub fn infer_table_expr(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    table: LuaTableExpr,
) -> InferResult {
    if table.is_array() {
        return infer_table_tuple_or_array(db, config, table);
    }

    Some(LuaType::TableConst(crate::InFiled {
        file_id: config.get_file_id(),
        value: table.get_range(),
    }))
}

fn infer_table_tuple_or_array(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    table: LuaTableExpr,
) -> InferResult {
    let fields = table.get_fields().collect::<Vec<_>>();
    if fields.len() > 10 {
        let first_type = infer_expr(db, config, fields[0].get_value_expr()?)?;
        return Some(LuaType::Array(first_type.into()));
    }

    if let Some(last_field) = fields.last() {
        let last_value_expr = last_field.get_value_expr()?;
        if is_dots_expr(&last_value_expr).unwrap_or(false) {
            let dots_type = infer_expr(db, config, last_value_expr)?;
            let typ = match &dots_type {
                LuaType::MuliReturn(multi) => multi.get_type(0).unwrap_or(&LuaType::Unknown),
                _ => &dots_type,
            };
            
            return Some(LuaType::Array(typ.clone().into()));
        }
    }

    let mut types = Vec::new();
    for field in fields {
        let value_expr = field.get_value_expr()?;
        let typ = infer_expr(db, config, value_expr)?;
        types.push(typ);
    }

    Some(LuaType::Tuple(LuaTupleType::new(types).into()))
}

fn is_dots_expr(expr: &LuaExpr) -> Option<bool> {
    if let LuaExpr::LiteralExpr(literal) = expr {
        match literal.get_literal()? {
            LuaLiteralToken::Dots(_) => return Some(true),
            _ => {}
        }
    }

    Some(false)
}
