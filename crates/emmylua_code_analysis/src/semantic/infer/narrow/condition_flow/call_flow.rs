use std::ops::Deref;

use emmylua_parser::{LuaCallExpr, LuaChunk, LuaExpr};

use crate::{
    infer_expr,
    semantic::infer::{
        narrow::{
            condition_flow::InferConditionFlow, get_single_antecedent,
            get_type_at_cast_flow::cast_type, get_type_at_flow::get_type_at_flow,
            var_ref_id::get_var_expr_var_ref_id, ResultTypeOrContinue,
        },
        VarRefId,
    },
    DbIndex, FlowNode, FlowTree, InferFailReason, LuaInferCache, LuaSignatureCast, LuaSignatureId,
    LuaType, TypeOps,
};

pub fn get_type_at_call_expr(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    call_expr: LuaCallExpr,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let Some(prefix_expr) = call_expr.get_prefix_expr() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let maybe_func = infer_expr(db, cache, prefix_expr.clone())?;
    match maybe_func {
        LuaType::DocFunction(f) => {
            let return_type = f.get_ret();
            match return_type {
                LuaType::TypeGuard(guard_type) => get_type_at_call_expr_by_type_guard(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    call_expr,
                    guard_type.deref().clone(),
                    condition_flow,
                ),
                _ => {
                    // If the return type is not a type guard, we cannot infer the type cast.
                    Ok(ResultTypeOrContinue::Continue)
                }
            }
        }
        LuaType::Signature(signature_id) => {
            let Some(signature_cast) = db
                .get_flow_index()
                .get_signature_cast(&cache.get_file_id(), &signature_id)
            else {
                return Ok(ResultTypeOrContinue::Continue);
            };

            match signature_cast.name.as_str() {
                "self" => get_type_at_call_expr_by_signature_self(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    prefix_expr,
                    signature_cast,
                    condition_flow,
                ),
                name => get_type_at_call_expr_by_signature_param_name(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    call_expr,
                    signature_cast,
                    signature_id,
                    name,
                    condition_flow,
                ),
            }
        }
        _ => {
            // If the prefix expression is not a function, we cannot infer the type cast.
            Ok(ResultTypeOrContinue::Continue)
        }
    }
}

fn get_type_at_call_expr_by_type_guard(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    call_expr: LuaCallExpr,
    guard_type: LuaType,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let Some(arg_list) = call_expr.get_args_list() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(first_arg) = arg_list.get_args().next() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(maybe_ref_id) = get_var_expr_var_ref_id(db, cache, first_arg) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    if maybe_ref_id != *var_ref_id {
        return Ok(ResultTypeOrContinue::Continue);
    }

    match condition_flow {
        InferConditionFlow::TrueCondition => Ok(ResultTypeOrContinue::Result(guard_type)),
        InferConditionFlow::FalseCondition => {
            let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
            let antecedent_type =
                get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;
            Ok(ResultTypeOrContinue::Result(TypeOps::Remove.apply(
                db,
                &antecedent_type,
                &guard_type,
            )))
        }
    }
}

fn get_type_at_call_expr_by_signature_self(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    call_prefix: LuaExpr,
    signature_cast: &LuaSignatureCast,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let LuaExpr::IndexExpr(call_prefix_index) = call_prefix else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(self_expr) = call_prefix_index.get_prefix_expr() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(name_var_ref_id) = get_var_expr_var_ref_id(db, cache, self_expr) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    if name_var_ref_id != *var_ref_id {
        return Ok(ResultTypeOrContinue::Continue);
    }

    let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
    let antecedent_type = get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;

    let Some(cast_op_type) = signature_cast.cast.to_node(root) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let result_type = cast_type(
        db,
        cache.get_file_id(),
        cast_op_type,
        antecedent_type,
        condition_flow,
    )?;
    Ok(ResultTypeOrContinue::Result(result_type))
}

fn get_type_at_call_expr_by_signature_param_name(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    call_expr: LuaCallExpr,
    signature_cast: &LuaSignatureCast,
    signature_id: LuaSignatureId,
    name: &str,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let colon_call = call_expr.is_colon_call();
    let Some(arg_list) = call_expr.get_args_list() else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(signature) = db.get_signature_index().get(&signature_id) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(mut param_idx) = signature.find_param_idx(name) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let colon_define = signature.is_colon_define;
    match (colon_call, colon_define) {
        (true, false) => {
            if param_idx == 0 {
                return Ok(ResultTypeOrContinue::Continue);
            }

            param_idx -= 1;
        }
        (false, true) => {
            param_idx += 1;
        }
        _ => {}
    }

    let Some(expr) = arg_list.get_args().nth(param_idx) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let Some(name_var_ref_id) = get_var_expr_var_ref_id(db, cache, expr) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    if name_var_ref_id != *var_ref_id {
        return Ok(ResultTypeOrContinue::Continue);
    }

    let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
    let antecedent_type = get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;

    let Some(cast_op_type) = signature_cast.cast.to_node(root) else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    let result_type = cast_type(
        db,
        cache.get_file_id(),
        cast_op_type,
        antecedent_type,
        condition_flow,
    )?;
    Ok(ResultTypeOrContinue::Result(result_type))
}
