use emmylua_parser::{
    BinaryOperator, LuaBinaryExpr, LuaCallExpr, LuaChunk, LuaExpr, LuaLiteralToken,
};

use crate::{
    infer_expr,
    semantic::infer::{
        narrow::{
            condition_flow::{call_flow::get_type_at_call_expr, InferConditionFlow},
            get_single_antecedent,
            get_type_at_flow::get_type_at_flow,
            narrow_down_type,
            var_ref_id::get_var_expr_var_ref_id,
            ResultTypeOrContinue,
        },
        VarRefId,
    },
    DbIndex, FlowNode, FlowTree, InferFailReason, LuaInferCache, LuaType, TypeOps,
};

pub fn get_type_at_binary_expr(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    binary_expr: LuaBinaryExpr,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let Some(op_token) = binary_expr.get_op_token() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some((left_expr, right_expr)) = binary_expr.get_exprs() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    match op_token.get_op() {
        BinaryOperator::OpLt
        | BinaryOperator::OpLe
        | BinaryOperator::OpGt
        | BinaryOperator::OpGe => {
            // todo check number range
        }
        BinaryOperator::OpEq => {
            let result_type = maybe_type_guard_binary(
                db,
                tree,
                cache,
                root,
                var_ref_id,
                flow_node,
                left_expr.clone(),
                right_expr.clone(),
                condition_flow,
            )?;
            if let ResultTypeOrContinue::Result(result_type) = result_type {
                return Ok(ResultTypeOrContinue::Result(result_type));
            }

            return maybe_var_eq_narrow(
                db,
                tree,
                cache,
                root,
                var_ref_id,
                flow_node,
                left_expr,
                right_expr,
                condition_flow,
            );
        }
        BinaryOperator::OpNe => {
            let result_type = maybe_type_guard_binary(
                db,
                tree,
                cache,
                root,
                var_ref_id,
                flow_node,
                left_expr.clone(),
                right_expr.clone(),
                condition_flow.get_negated(),
            )?;
            if let ResultTypeOrContinue::Result(result_type) = result_type {
                return Ok(ResultTypeOrContinue::Result(result_type));
            }

            return maybe_var_eq_narrow(
                db,
                tree,
                cache,
                root,
                var_ref_id,
                flow_node,
                left_expr,
                right_expr,
                condition_flow.get_negated(),
            );
        }
        _ => {}
    }

    Ok(ResultTypeOrContinue::Continue)
}

fn maybe_type_guard_binary(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    left_expr: LuaExpr,
    right_expr: LuaExpr,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let mut type_guard_expr: Option<LuaCallExpr> = None;
    let mut literal_string = String::new();
    if let LuaExpr::CallExpr(call_expr) = left_expr {
        if call_expr.is_type() {
            type_guard_expr = Some(call_expr);
            if let LuaExpr::LiteralExpr(literal_expr) = right_expr {
                match literal_expr.get_literal() {
                    Some(LuaLiteralToken::String(s)) => {
                        literal_string = s.get_value();
                    }
                    _ => return Ok(ResultTypeOrContinue::Continue),
                }
            }
        }
    } else if let LuaExpr::CallExpr(call_expr) = right_expr {
        if call_expr.is_type() {
            type_guard_expr = Some(call_expr);
            if let LuaExpr::LiteralExpr(literal_expr) = left_expr {
                match literal_expr.get_literal() {
                    Some(LuaLiteralToken::String(s)) => {
                        literal_string = s.get_value();
                    }
                    _ => return Ok(ResultTypeOrContinue::Continue),
                }
            }
        }
    }

    if type_guard_expr.is_none() || literal_string.is_empty() {
        return Ok(ResultTypeOrContinue::Continue);
    }

    let Some(arg_list) = type_guard_expr.unwrap().get_args_list() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(arg) = arg_list.get_args().next() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let LuaExpr::NameExpr(name_expr) = arg else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(maybe_var_ref_id) =
        get_var_expr_var_ref_id(db, cache, LuaExpr::NameExpr(name_expr.clone()))
    else {
        // If we cannot find a reference declaration ID, we cannot narrow it
        return Ok(ResultTypeOrContinue::Continue);
    };

    if maybe_var_ref_id != *var_ref_id {
        return Ok(ResultTypeOrContinue::Continue);
    }

    let anatecedent_flow_id = get_single_antecedent(tree, flow_node)?;
    let antecedent_type = get_type_at_flow(db, tree, cache, root, var_ref_id, anatecedent_flow_id)?;

    let narrow = match literal_string.as_str() {
        "number" => LuaType::Number,
        "string" => LuaType::String,
        "boolean" => LuaType::Boolean,
        "table" => LuaType::Table,
        "function" => LuaType::Function,
        "thread" => LuaType::Thread,
        "userdata" => LuaType::Userdata,
        "nil" => LuaType::Nil,
        _ => {
            // If the type is not recognized, we cannot narrow it
            return Ok(ResultTypeOrContinue::Continue);
        }
    };

    let result_type = match condition_flow {
        InferConditionFlow::TrueCondition => {
            narrow_down_type(db, antecedent_type.clone(), narrow.clone()).unwrap_or(narrow)
        }
        InferConditionFlow::FalseCondition => TypeOps::Remove.apply(db, &antecedent_type, &narrow),
    };

    Ok(ResultTypeOrContinue::Result(result_type))
}

fn maybe_var_eq_narrow(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    left_expr: LuaExpr,
    right_expr: LuaExpr,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    // only check left as need narrow
    match left_expr {
        LuaExpr::NameExpr(left_name_expr) => {
            let Some(maybe_ref_id) =
                get_var_expr_var_ref_id(db, cache, LuaExpr::NameExpr(left_name_expr.clone()))
            else {
                return Ok(ResultTypeOrContinue::Continue);
            };

            if maybe_ref_id != *var_ref_id {
                // If the reference declaration ID does not match, we cannot narrow it
                return Ok(ResultTypeOrContinue::Continue);
            }

            let right_expr_type = infer_expr(db, cache, right_expr)?;
            let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
            let antecedent_type =
                get_type_at_flow(db, tree, cache, root, &var_ref_id, antecedent_flow_id)?;

            let result_type = match condition_flow {
                InferConditionFlow::TrueCondition => {
                    narrow_down_type(db, antecedent_type, right_expr_type.clone())
                        .unwrap_or(right_expr_type)
                }
                InferConditionFlow::FalseCondition => {
                    TypeOps::Remove.apply(db, &antecedent_type, &right_expr_type)
                }
            };
            Ok(ResultTypeOrContinue::Result(result_type))
        }
        LuaExpr::CallExpr(left_call_expr) => {
            match right_expr {
                LuaExpr::LiteralExpr(literal_expr) => match literal_expr.get_literal() {
                    Some(LuaLiteralToken::Bool(b)) => {
                        let flow = if b.is_true() {
                            condition_flow
                        } else {
                            condition_flow.get_negated()
                        };

                        return get_type_at_call_expr(
                            db,
                            tree,
                            cache,
                            root,
                            &var_ref_id,
                            flow_node,
                            left_call_expr,
                            flow,
                        );
                    }
                    _ => return Ok(ResultTypeOrContinue::Continue),
                },
                _ => {}
            };

            Ok(ResultTypeOrContinue::Continue)
        }
        LuaExpr::IndexExpr(left_index_expr) => {
            let Some(maybe_ref_id) =
                get_var_expr_var_ref_id(db, cache, LuaExpr::IndexExpr(left_index_expr.clone()))
            else {
                return Ok(ResultTypeOrContinue::Continue);
            };

            if maybe_ref_id != *var_ref_id {
                // If the reference declaration ID does not match, we cannot narrow it
                return Ok(ResultTypeOrContinue::Continue);
            }

            let right_expr_type = infer_expr(db, cache, right_expr)?;
            let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
            let antecedent_type =
                get_type_at_flow(db, tree, cache, root, &var_ref_id, antecedent_flow_id)?;

            let result_type = match condition_flow {
                InferConditionFlow::TrueCondition => {
                    narrow_down_type(db, antecedent_type, right_expr_type.clone())
                        .unwrap_or(right_expr_type)
                }
                InferConditionFlow::FalseCondition => {
                    TypeOps::Remove.apply(db, &antecedent_type, &right_expr_type)
                }
            };
            Ok(ResultTypeOrContinue::Result(result_type))
        }
        _ => {
            // If the left expression is not a name or call expression, we cannot narrow it
            Ok(ResultTypeOrContinue::Continue)
        }
    }
}
