mod infer_binary;

use emmylua_parser::{LuaBinaryExpr, LuaExpr, LuaLiteralExpr, LuaLiteralToken};
use infer_binary::infer_binary_expr;

use crate::db_index::{DbIndex, LuaType};

pub type InferResult = Option<LuaType>;

pub fn infer_expr(db: &DbIndex, expr: LuaExpr) -> InferResult {
    match expr {
        LuaExpr::CallExpr(lua_call_expr) => todo!(),
        LuaExpr::TableExpr(lua_table_expr) => todo!(),
        LuaExpr::LiteralExpr(lua_literal_expr) => return infer_literal_expr(lua_literal_expr),
        LuaExpr::BinaryExpr(lua_binary_expr) => return infer_binary_expr(db, lua_binary_expr),
        LuaExpr::UnaryExpr(lua_unary_expr) => todo!(),
        LuaExpr::ClosureExpr(lua_closure_expr) => todo!(),
        LuaExpr::ParenExpr(lua_paren_expr) => todo!(),
        LuaExpr::NameExpr(lua_name_expr) => todo!(),
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

