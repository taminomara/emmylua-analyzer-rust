use emmylua_parser::{LuaUnaryExpr, UnaryOperator};

use crate::{
    db_index::{DbIndex, LuaOperatorMetaMethod, LuaType},
    LuaInferCache,
};

use super::{get_custom_type_operator, infer_expr, InferResult};

pub fn infer_unary_expr(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    unary_expr: LuaUnaryExpr,
) -> InferResult {
    let op = unary_expr.get_op_token()?.get_op();
    let inner_expr = unary_expr.get_expr()?;
    let inner_type = infer_expr(db, cache, inner_expr)?;
    match op {
        UnaryOperator::OpNot => infer_unary_expr_not(inner_type),
        UnaryOperator::OpLen => Some(LuaType::Integer),
        UnaryOperator::OpUnm => infer_unary_expr_unm(db, inner_type),
        UnaryOperator::OpBNot => infer_unary_expr_bnot(db, inner_type),
        UnaryOperator::OpNop => Some(inner_type),
    }
}

fn infer_unary_custom_operator(
    db: &DbIndex,
    inner: &LuaType,
    op: LuaOperatorMetaMethod,
) -> InferResult {
    let operators = get_custom_type_operator(db, inner.clone(), op);
    if let Some(operators) = operators {
        for operator in operators {
            return Some(operator.get_result().clone());
        }
    }

    match op {
        LuaOperatorMetaMethod::Unm => Some(LuaType::Number),
        LuaOperatorMetaMethod::BNot => Some(LuaType::Integer),
        _ => None,
    }
}

fn infer_unary_expr_not(inner_type: LuaType) -> InferResult {
    match inner_type {
        LuaType::BooleanConst(b) => Some(LuaType::BooleanConst(!b)),
        _ => Some(LuaType::Boolean),
    }
}

fn infer_unary_expr_unm(db: &DbIndex, inner_type: LuaType) -> InferResult {
    match inner_type {
        LuaType::IntegerConst(i) => Some(LuaType::IntegerConst(-i)),
        LuaType::FloatConst(f) => Some(LuaType::FloatConst((-f).into())),
        LuaType::Integer => Some(LuaType::Integer),
        _ => infer_unary_custom_operator(db, &inner_type, LuaOperatorMetaMethod::Unm),
    }
}

fn infer_unary_expr_bnot(db: &DbIndex, inner_type: LuaType) -> InferResult {
    match inner_type {
        LuaType::IntegerConst(i) => Some(LuaType::IntegerConst(!i)),
        _ => infer_unary_custom_operator(db, &inner_type, LuaOperatorMetaMethod::BNot),
    }
}
