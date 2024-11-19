use emmylua_parser::{BinaryOperator, LuaBinaryExpr};

use crate::db_index::{DbIndex, LuaOperator, LuaOperatorMetaMethod, LuaType};

use super::{infer_expr, InferResult};

pub fn infer_binary_expr(db: &DbIndex, expr: LuaBinaryExpr) -> InferResult {
    let op = expr.get_op_token()?.get_op();
    let (left, right) = expr.get_exprs()?;
    let left_type = infer_expr(db, left)?;
    let right_type = infer_expr(db, right)?;

    match op {
        BinaryOperator::OpAdd => infer_binary_expr_add(db, left_type, right_type),
        BinaryOperator::OpSub => todo!(),
        BinaryOperator::OpMul => todo!(),
        BinaryOperator::OpDiv => todo!(),
        BinaryOperator::OpIDiv => todo!(),
        BinaryOperator::OpMod => todo!(),
        BinaryOperator::OpPow => todo!(),
        BinaryOperator::OpBAnd => todo!(),
        BinaryOperator::OpBOr => todo!(),
        BinaryOperator::OpBXor => todo!(),
        BinaryOperator::OpShl => todo!(),
        BinaryOperator::OpShr => todo!(),
        BinaryOperator::OpConcat => todo!(),
        BinaryOperator::OpLt => todo!(),
        BinaryOperator::OpLe => todo!(),
        BinaryOperator::OpGt => todo!(),
        BinaryOperator::OpGe => todo!(),
        BinaryOperator::OpEq => todo!(),
        BinaryOperator::OpNe => todo!(),
        BinaryOperator::OpAnd => todo!(),
        BinaryOperator::OpOr => todo!(),
        BinaryOperator::OpNop => todo!(),
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

fn infer_binary_custom_operator(
    db: &DbIndex,
    left: &LuaType,
    right: &LuaType,
    op: LuaOperatorMetaMethod,
) -> InferResult {
    let operators = get_custom_type_operator(db, left.clone(), op);
    if let Some(operators) = operators {
        for operator in operators {
            let first_param = operator.get_operands().get(0)?;
            if first_param == right {
                return Some(operator.get_result().clone());
            }
        }
    }

    let operators = get_custom_type_operator(db, right.clone(), op);
    if let Some(operators) = operators {
        for operator in operators {
            let first_param = operator.get_operands().get(0)?;
            if first_param == left {
                return Some(operator.get_result().clone());
            }
        }
    }

    None
}

fn infer_binary_expr_add(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 + int2))
            }
            _ => {
                if left.is_integer() && right.is_integer() {
                    Some(LuaType::Integer)
                } else {
                    Some(LuaType::Number)
                }
            }
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Add)
}

