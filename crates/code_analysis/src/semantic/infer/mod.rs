mod infer_binary;
mod infer_call;
mod infer_config;
mod infer_index;
mod infer_name;
mod infer_table;
mod infer_unary;

use emmylua_parser::{LuaAstNode, LuaClosureExpr, LuaExpr, LuaLiteralExpr, LuaLiteralToken};
use infer_binary::infer_binary_expr;
use infer_call::infer_call_expr;
pub use infer_call::instantiate_doc_function;
use infer_config::ExprCache;
pub use infer_config::LuaInferConfig;
use infer_index::infer_index_expr;
use infer_name::infer_name_expr;
use infer_table::infer_table_expr;
use infer_unary::infer_unary_expr;
use smol_str::SmolStr;

use crate::{
    db_index::{DbIndex, LuaOperator, LuaOperatorMetaMethod, LuaSignatureId, LuaType},
    InFiled,
};

pub type InferResult = Option<LuaType>;

pub fn infer_expr(db: &DbIndex, config: &mut LuaInferConfig, expr: LuaExpr) -> InferResult {
    let syntax_id = expr.get_syntax_id();
    match config.get_cache_expr_type(&syntax_id) {
        Some(ExprCache::Cache(ty)) => return Some(ty.clone()),
        Some(ExprCache::ReadyCache) => return Some(LuaType::Unknown),
        None => {}
    }

    // for @as
    let file_id = config.get_file_id();
    let in_filed_syntax_id = InFiled::new(file_id, syntax_id);
    if let Some(force_type) = db.get_type_index().get_as_force_type(&in_filed_syntax_id) {
        config.cache_expr_type(syntax_id, force_type.clone());
        return Some(force_type.clone());
    }

    config.mark_ready_cache(syntax_id);
    let result_type = match expr {
        LuaExpr::CallExpr(call_expr) => infer_call_expr(db, config, call_expr)?,
        LuaExpr::TableExpr(table_expr) => infer_table_expr(db, config, table_expr)?,
        LuaExpr::LiteralExpr(literal_expr) => infer_literal_expr(literal_expr)?,
        LuaExpr::BinaryExpr(binary_expr) => infer_binary_expr(db, config, binary_expr)?,
        LuaExpr::UnaryExpr(unary_expr) => infer_unary_expr(db, config, unary_expr)?,
        LuaExpr::ClosureExpr(closure_expr) => infer_closure_expr(config, closure_expr)?,
        LuaExpr::ParenExpr(paren_expr) => infer_expr(db, config, paren_expr.get_expr()?)?,
        LuaExpr::NameExpr(name_expr) => infer_name_expr(db, config, name_expr)?,
        LuaExpr::IndexExpr(index_expr) => infer_index_expr(db, config, index_expr)?,
    };

    config.cache_expr_type(syntax_id, result_type.clone());

    Some(result_type)
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
        LuaLiteralToken::String(str) => Some(LuaType::StringConst(SmolStr::new(str.get_value()).into())),
    }
}

fn infer_closure_expr(config: &LuaInferConfig, closure: LuaClosureExpr) -> InferResult {
    Some(LuaType::Signature(LuaSignatureId::new(
        config.get_file_id(),
        &closure,
    )))
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
