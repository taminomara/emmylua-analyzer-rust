mod infer_binary;
mod infer_unary;
mod infer_table;
mod infer_name;
mod infer_config;

use emmylua_parser::{LuaExpr, LuaLiteralExpr, LuaLiteralToken};
use infer_binary::infer_binary_expr;
pub use infer_config::LuaInferConfig;
use infer_name::infer_name_expr;
use infer_table::infer_table_expr;
use infer_unary::infer_unary_expr;

use crate::db_index::{DbIndex, LuaOperator, LuaOperatorMetaMethod, LuaType};

pub type InferResult = Option<LuaType>;

pub fn infer_expr(db: &DbIndex, config: &LuaInferConfig, expr: LuaExpr) -> InferResult {
    match expr {
        LuaExpr::CallExpr(lua_call_expr) => todo!(),
        LuaExpr::TableExpr(lua_table_expr) => infer_table_expr(db, config, lua_table_expr),
        LuaExpr::LiteralExpr(lua_literal_expr) => return infer_literal_expr(lua_literal_expr),
        LuaExpr::BinaryExpr(lua_binary_expr) => return infer_binary_expr(db, config, lua_binary_expr),
        LuaExpr::UnaryExpr(lua_unary_expr) => infer_unary_expr(db, config, lua_unary_expr),
        LuaExpr::ClosureExpr(lua_closure_expr) => todo!(),
        LuaExpr::ParenExpr(lua_paren_expr) => return infer_expr(db, config, lua_paren_expr.get_expr()?),
        LuaExpr::NameExpr(lua_name_expr) => return infer_name_expr(db, config, lua_name_expr),
        LuaExpr::IndexExpr(lua_index_expr) => todo!(),
    }
}

fn infer_literal_expr(expr: LuaLiteralExpr) -> InferResult {
    match expr.get_literal()? {
        LuaLiteralToken::Nil(_) => Some(LuaType::Nil),
        LuaLiteralToken::Bool(bool) => Some(LuaType::BooleanConst(bool.is_true())),
        LuaLiteralToken::Number(num) => {
            if num.is_int() {
                Some(LuaType::IntegerConst(num.get_int_value()))
            } else {
                Some(LuaType::Number)
            }
        }
        LuaLiteralToken::String(str) => Some(LuaType::StringConst(str.get_value().into())),
    }
}


fn get_custom_type_operator(
    db: &DbIndex,
    operand_type: LuaType,
    op: LuaOperatorMetaMethod,
) -> Option<Vec<&LuaOperator>> {
    if operand_type.is_custom_type() {
        let type_id = match operand_type {
            LuaType::Ref(type_id) => type_id,
            LuaType::Def(type_id) => type_id,
            _ => return None,
        };
        let ops = db.get_operator_index().get_operators_by_type(&type_id)?;
        let op_ids = ops.get(&op)?;
        let operators = op_ids
            .iter()
            .filter_map(|id| db.get_operator_index().get_operator(id))
            .collect();

        Some(operators)
    } else {
        None
    }
}