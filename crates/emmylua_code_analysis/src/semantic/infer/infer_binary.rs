use emmylua_parser::{BinaryOperator, LuaBinaryExpr};
use smol_str::SmolStr;

use crate::{
    check_type_compact,
    db_index::{DbIndex, LuaOperatorMetaMethod, LuaType},
    LuaInferCache, LuaUnionType, TypeOps,
};

use super::{get_custom_type_operator, infer_expr, InferResult};

pub fn infer_binary_expr(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    expr: LuaBinaryExpr,
) -> InferResult {
    let op = expr.get_op_token()?.get_op();
    let (left, right) = expr.get_exprs()?;
    let left_type = infer_expr(db, cache, left);
    let right_type = infer_expr(db, cache, right);
    let left_type = left_type?;
    let right_type = right_type?;

    match (&left_type, &right_type) {
        (LuaType::Union(u), right) => infer_union(db, &u, right, op),
        (left, LuaType::Union(u)) => infer_union(db, &u, left, op),
        _ => infer_binary_expr_type(db, left_type, right_type, op),
    }
}

fn infer_binary_expr_type(
    db: &DbIndex,
    left_type: LuaType,
    right_type: LuaType,
    op: BinaryOperator,
) -> InferResult {
    match op {
        BinaryOperator::OpAdd => infer_binary_expr_add(db, left_type, right_type),
        BinaryOperator::OpSub => infer_binary_expr_sub(db, left_type, right_type),
        BinaryOperator::OpMul => infer_binary_expr_mul(db, left_type, right_type),
        BinaryOperator::OpDiv => infer_binary_expr_div(db, left_type, right_type),
        BinaryOperator::OpIDiv => infer_binary_expr_idiv(db, left_type, right_type),
        BinaryOperator::OpMod => infer_binary_expr_mod(db, left_type, right_type),
        BinaryOperator::OpPow => infer_binary_expr_pow(db, left_type, right_type),
        BinaryOperator::OpBAnd => infer_binary_expr_band(db, left_type, right_type),
        BinaryOperator::OpBOr => infer_binary_expr_bor(db, left_type, right_type),
        BinaryOperator::OpBXor => infer_binary_expr_bxor(db, left_type, right_type),
        BinaryOperator::OpShl => infer_binary_expr_shl(db, left_type, right_type),
        BinaryOperator::OpShr => infer_binary_expr_shr(db, left_type, right_type),
        BinaryOperator::OpConcat => infer_binary_expr_concat(db, left_type, right_type),
        BinaryOperator::OpOr => infer_binary_expr_or(left_type, right_type),
        BinaryOperator::OpAnd => infer_binary_expr_and(left_type, right_type),
        BinaryOperator::OpLt
        | BinaryOperator::OpLe
        | BinaryOperator::OpGt
        | BinaryOperator::OpGe
        | BinaryOperator::OpEq
        | BinaryOperator::OpNe => infer_cmp_expr(db, left_type, right_type, op),
        _ => Some(left_type),
    }
}


fn infer_union(db: &DbIndex, u: &LuaUnionType, right: &LuaType, op: BinaryOperator) -> InferResult {
    let mut union_types = vec![];
    for ty in u.get_types() {
        let ty = infer_binary_expr_type(db, ty.clone(), right.clone(), op)?;
        union_types.push(ty);
    }
    union_types.dedup();

    match union_types.len() {
        0 => Some(LuaType::Unknown),
        1 => Some(union_types[0].clone()),
        _ => Some(LuaType::Union(LuaUnionType::new(union_types).into())),
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
            if check_type_compact(db, first_param, right).is_ok() {
                return Some(operator.get_result().clone());
            }
        }
    }

    let operators = get_custom_type_operator(db, right.clone(), op);
    if let Some(operators) = operators {
        for operator in operators {
            let first_param = operator.get_operands().get(0)?;
            if check_type_compact(db, first_param, left).is_ok() {
                return Some(operator.get_result().clone());
            }
        }
    }

    match op {
        LuaOperatorMetaMethod::Add
        | LuaOperatorMetaMethod::Sub
        | LuaOperatorMetaMethod::Mul
        | LuaOperatorMetaMethod::Div
        | LuaOperatorMetaMethod::Mod
        | LuaOperatorMetaMethod::Pow => Some(LuaType::Number),
        LuaOperatorMetaMethod::IDiv
        | LuaOperatorMetaMethod::BAnd
        | LuaOperatorMetaMethod::BOr
        | LuaOperatorMetaMethod::BXor
        | LuaOperatorMetaMethod::Shl
        | LuaOperatorMetaMethod::Shr => Some(LuaType::Integer),

        LuaOperatorMetaMethod::Concat => Some(LuaType::String),
        _ => None,
    }
}

fn infer_binary_expr_add(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 + int2))
            }
            (LuaType::FloatConst(num1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst(num1 + num2))
            }
            (LuaType::IntegerConst(int1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst((*int1 as f64 + *num2).into()))
            }
            (LuaType::FloatConst(num1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::FloatConst((*num1 + *int2 as f64).into()))
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

fn infer_binary_expr_sub(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 - int2))
            }
            (LuaType::FloatConst(num1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst(num1 - num2))
            }
            (LuaType::IntegerConst(int1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst((*int1 as f64 - *num2).into()))
            }
            (LuaType::FloatConst(num1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::FloatConst((*num1 - *int2 as f64).into()))
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

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Sub)
}

fn infer_binary_expr_mul(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 * int2))
            }
            (LuaType::FloatConst(num1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst(num1 * num2))
            }
            (LuaType::IntegerConst(int1), LuaType::FloatConst(num2)) => {
                Some(LuaType::FloatConst((*int1 as f64 * *num2).into()))
            }
            (LuaType::FloatConst(num1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::FloatConst((*num1 * *int2 as f64).into()))
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

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Mul)
}

fn infer_binary_expr_div(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                if *int2 != 0 {
                    return Some(LuaType::FloatConst((*int1 as f64 / *int2 as f64).into()));
                }
                Some(LuaType::Number)
            }
            (LuaType::FloatConst(num1), LuaType::FloatConst(num2)) => {
                if *num2 != 0.0 {
                    return Some(LuaType::FloatConst(num1 / num2));
                }
                Some(LuaType::Number)
            }
            (LuaType::IntegerConst(int1), LuaType::FloatConst(num2)) => {
                if *num2 != 0.0 {
                    return Some(LuaType::FloatConst((*int1 as f64 / *num2).into()));
                }
                Some(LuaType::Number)
            }
            (LuaType::FloatConst(num1), LuaType::IntegerConst(int2)) => {
                if *int2 != 0 {
                    return Some(LuaType::FloatConst(*num1 / *int2 as f64));
                }
                Some(LuaType::Number)
            }
            _ => Some(LuaType::Number),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Div)
}

fn infer_binary_expr_idiv(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                if *int2 != 0 {
                    return Some(LuaType::IntegerConst(int1 / int2));
                }
                Some(LuaType::Integer)
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::IDiv)
}

fn infer_binary_expr_mod(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                if *int2 != 0 {
                    return Some(LuaType::IntegerConst(int1 % int2));
                }
                Some(LuaType::Integer)
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Mod)
}

fn infer_binary_expr_pow(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() && right.is_number() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                if let Some(int3) = int1.checked_pow(*int2 as u32) {
                    Some(LuaType::IntegerConst(int3))
                } else {
                    Some(LuaType::Number)
                }
            }
            (LuaType::FloatConst(num1), LuaType::IntegerConst(num2)) => {
                Some(LuaType::FloatConst(num1.powf(*num2 as f64).into()))
            }
            _ => Some(LuaType::Number),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Pow)
}

fn infer_binary_expr_band(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 & int2))
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::BAnd)
}

fn infer_binary_expr_bor(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 | int2))
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::BOr)
}

fn infer_binary_expr_bxor(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 ^ int2))
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::BXor)
}

fn infer_binary_expr_shl(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 << int2))
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Shl)
}

fn infer_binary_expr_shr(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_integer() && right.is_integer() {
        return match (&left, &right) {
            (LuaType::IntegerConst(int1), LuaType::IntegerConst(int2)) => {
                Some(LuaType::IntegerConst(int1 >> int2))
            }
            _ => Some(LuaType::Integer),
        };
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Shr)
}

fn infer_binary_expr_concat(db: &DbIndex, left: LuaType, right: LuaType) -> InferResult {
    if left.is_number() || left.is_string() || right.is_number() || right.is_string() {
        match (&left, &right) {
            (LuaType::StringConst(s1), LuaType::StringConst(s2)) => {
                return Some(LuaType::StringConst(
                    SmolStr::new(format!("{}{}", s1.as_str(), s2.as_str())).into(),
                ));
            }
            (LuaType::StringConst(s1), LuaType::IntegerConst(i)) => {
                return Some(LuaType::StringConst(
                    SmolStr::new(format!("{}{}", s1.as_str(), i)).into(),
                ));
            }
            (LuaType::IntegerConst(i), LuaType::StringConst(s2)) => {
                return Some(LuaType::StringConst(
                    SmolStr::new(format!("{}{}", i, s2.as_str())).into(),
                ));
            }
            _ => return Some(LuaType::String),
        }
    }

    infer_binary_custom_operator(db, &left, &right, LuaOperatorMetaMethod::Concat)
}

fn infer_binary_expr_or(left: LuaType, right: LuaType) -> InferResult {
    if left.is_always_truthy() {
        return Some(left);
    } else if left.is_always_falsy() {
        return Some(right);
    }

    Some(TypeOps::Union.apply(&TypeOps::Remove.apply(&left, &LuaType::Nil), &right))
}

fn infer_binary_expr_and(left: LuaType, right: LuaType) -> InferResult {
    if left.is_always_falsy() {
        return Some(left);
    } else if left.is_always_truthy() {
        return Some(right);
    }

    Some(TypeOps::Union.apply(&TypeOps::NarrowFalseOrNil.apply_source(&left), &right))
}

fn infer_cmp_expr(_: &DbIndex, left: LuaType, right: LuaType, op: BinaryOperator) -> InferResult {
    match (left, right) {
        (LuaType::IntegerConst(i), LuaType::IntegerConst(j)) => {
            Some(LuaType::BooleanConst(integer_cmp(i, j, op)))
        }
        (LuaType::IntegerConst(i), LuaType::DocIntegerConst(j)) => {
            Some(LuaType::BooleanConst(integer_cmp(i, j, op)))
        }
        (LuaType::DocIntegerConst(i), LuaType::IntegerConst(j)) => {
            Some(LuaType::BooleanConst(integer_cmp(i, j, op)))
        }
        (LuaType::DocIntegerConst(i), LuaType::DocIntegerConst(j)) => {
            Some(LuaType::BooleanConst(integer_cmp(i, j, op)))
        }
        (LuaType::FloatConst(i), LuaType::FloatConst(j)) => {
            Some(LuaType::BooleanConst(float_cmp(i, j, op)))
        }
        (LuaType::IntegerConst(i), LuaType::FloatConst(j)) => {
            Some(LuaType::BooleanConst(float_cmp(i as f64, j, op)))
        }
        (LuaType::FloatConst(i), LuaType::IntegerConst(j)) => {
            Some(LuaType::BooleanConst(float_cmp(i, j as f64, op)))
        }
        (LuaType::DocIntegerConst(i), LuaType::FloatConst(j)) => {
            Some(LuaType::BooleanConst(float_cmp(i as f64, j, op)))
        }
        (LuaType::FloatConst(i), LuaType::DocIntegerConst(j)) => {
            Some(LuaType::BooleanConst(float_cmp(i, j as f64, op)))
        }
        (LuaType::DocBooleanConst(i), LuaType::DocBooleanConst(j)) => match op {
            BinaryOperator::OpEq => Some(LuaType::BooleanConst(i == j)),
            BinaryOperator::OpNe => Some(LuaType::BooleanConst(i != j)),
            _ => Some(LuaType::Boolean),
        },
        (LuaType::BooleanConst(i), LuaType::BooleanConst(j)) => match op {
            BinaryOperator::OpEq => Some(LuaType::BooleanConst(i == j)),
            BinaryOperator::OpNe => Some(LuaType::BooleanConst(i != j)),
            _ => Some(LuaType::Boolean),
        },
        (LuaType::DocStringConst(i), LuaType::DocStringConst(j)) => match op {
            BinaryOperator::OpEq => Some(LuaType::BooleanConst(i == j)),
            BinaryOperator::OpNe => Some(LuaType::BooleanConst(i != j)),
            _ => Some(LuaType::Boolean),
        },
        (LuaType::StringConst(i), LuaType::StringConst(j)) => match op {
            BinaryOperator::OpEq => Some(LuaType::BooleanConst(i == j)),
            BinaryOperator::OpNe => Some(LuaType::BooleanConst(i != j)),
            _ => Some(LuaType::Boolean),
        },
        (LuaType::TableConst(i), LuaType::TableConst(j)) => match op {
            BinaryOperator::OpEq => Some(LuaType::BooleanConst(i == j)),
            BinaryOperator::OpNe => Some(LuaType::BooleanConst(i != j)),
            _ => Some(LuaType::Boolean),
        },
        (left, right) if left.is_const() && right.is_const() => Some(LuaType::BooleanConst(false)),
        _ => Some(LuaType::Boolean),
    }
}

fn integer_cmp(left: i64, right: i64, op: BinaryOperator) -> bool {
    match op {
        BinaryOperator::OpGt => left > right,
        BinaryOperator::OpGe => left >= right,
        BinaryOperator::OpLt => left < right,
        BinaryOperator::OpLe => left <= right,
        BinaryOperator::OpEq => left == right,
        BinaryOperator::OpNe => left != right,
        _ => false,
    }
}

fn float_cmp(left: f64, right: f64, op: BinaryOperator) -> bool {
    match op {
        BinaryOperator::OpGt => left > right,
        BinaryOperator::OpGe => left >= right,
        BinaryOperator::OpLt => left < right,
        BinaryOperator::OpLe => left <= right,
        BinaryOperator::OpEq => left == right,
        BinaryOperator::OpNe => left != right,
        _ => false,
    }
}
