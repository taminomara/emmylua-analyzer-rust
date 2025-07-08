use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaCallExpr, LuaChunk, LuaVarExpr};

use crate::{
    infer_expr,
    semantic::infer::{
        narrow::{
            condition_flow::{get_type_at_condition_flow, InferConditionFlow},
            get_multi_antecedents, get_single_antecedent,
            get_type_at_cast_flow::get_type_at_cast_flow,
            get_var_ref_type,
            narrow_type::{narrow_down_type, remove_false_or_nil},
            var_ref_id::get_var_expr_var_ref_id,
            ResultTypeOrContinue,
        },
        InferResult, VarRefId,
    },
    CacheEntry, DbIndex, FlowId, FlowNode, FlowNodeKind, FlowTree, InferFailReason, LuaDeclId,
    LuaInferCache, LuaMemberId, LuaType, TypeOps,
};

pub fn get_type_at_flow(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_id: FlowId,
) -> InferResult {
    let key = (var_ref_id.clone(), flow_id);
    if let Some(cache_entry) = cache.flow_node_cache.get(&key) {
        if let CacheEntry::Cache(narrow_type) = cache_entry {
            return Ok(narrow_type.clone());
        }
    }

    let mut result_type = LuaType::Unknown;
    let mut antecedent_flow_id = flow_id;
    loop {
        let flow_node = tree
            .get_flow_node(antecedent_flow_id)
            .ok_or(InferFailReason::None)?;
        match &flow_node.kind {
            FlowNodeKind::Start | FlowNodeKind::Unreachable => {
                result_type = get_var_ref_type(db, cache, var_ref_id)?;
                break;
            }
            FlowNodeKind::LoopLabel | FlowNodeKind::Break | FlowNodeKind::Return => {
                antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
            }
            FlowNodeKind::BranchLabel | FlowNodeKind::NamedLabel(_) => {
                let multi_antecedents = get_multi_antecedents(tree, flow_node)?;
                for flow_id in multi_antecedents {
                    let branch_type = get_type_at_flow(db, tree, cache, root, var_ref_id, flow_id)?;
                    result_type = TypeOps::Union.apply(db, &result_type, &branch_type);
                }
                break;
            }
            FlowNodeKind::DeclPosition(position) => {
                if *position <= var_ref_id.get_position() {
                    result_type = get_var_ref_type(db, cache, var_ref_id)?;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
            FlowNodeKind::Assignment(assign_ptr) => {
                let assign_stat = assign_ptr.to_node(root).ok_or(InferFailReason::None)?;
                let result_or_continue = get_type_at_assign_stat(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    assign_stat,
                )?;

                if let ResultTypeOrContinue::Result(assign_type) = result_or_continue {
                    result_type = assign_type;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
            FlowNodeKind::TrueCondition(condition_ptr) => {
                let condition = condition_ptr.to_node(root).ok_or(InferFailReason::None)?;
                let result_or_continue = get_type_at_condition_flow(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    condition,
                    InferConditionFlow::TrueCondition,
                )?;

                if let ResultTypeOrContinue::Result(condition_type) = result_or_continue {
                    result_type = condition_type;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
            FlowNodeKind::FalseCondition(condition_ptr) => {
                let condition = condition_ptr.to_node(root).ok_or(InferFailReason::None)?;
                let result_or_continue = get_type_at_condition_flow(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    condition,
                    InferConditionFlow::FalseCondition,
                )?;

                if let ResultTypeOrContinue::Result(condition_type) = result_or_continue {
                    result_type = condition_type;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
            FlowNodeKind::ForIStat(_) => {
                // todo check for `for i = 1, 10 do end`
                antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
            }
            FlowNodeKind::TagCast(cast_ast_ptr) => {
                let tag_cast = cast_ast_ptr.to_node(root).ok_or(InferFailReason::None)?;
                let cast_or_continue =
                    get_type_at_cast_flow(db, tree, cache, root, var_ref_id, flow_node, tag_cast)?;

                if let ResultTypeOrContinue::Result(cast_type) = cast_or_continue {
                    result_type = cast_type;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
            FlowNodeKind::AssertCall(lua_ast_ptr) => {
                let assert_call = lua_ast_ptr.to_node(root).ok_or(InferFailReason::None)?;
                let result_or_continue = get_type_at_assert_call(
                    db,
                    tree,
                    cache,
                    root,
                    var_ref_id,
                    flow_node,
                    assert_call,
                )?;

                if let ResultTypeOrContinue::Result(assert_type) = result_or_continue {
                    result_type = assert_type;
                    break;
                } else {
                    antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                }
            }
        }
    }

    cache
        .flow_node_cache
        .insert(key, CacheEntry::Cache(result_type.clone()));
    Ok(result_type)
}

fn get_type_at_assign_stat(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    assign_stat: LuaAssignStat,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let (vars, exprs) = assign_stat.get_var_and_expr_list();
    for i in 0..vars.len() {
        let var = vars[i].clone();
        let Some(maybe_ref_id) = get_var_expr_var_ref_id(db, cache, var.to_expr()) else {
            continue;
        };

        if maybe_ref_id != *var_ref_id {
            // let typ = get_var_ref_type(db, cache, var_ref_id)?;
            continue;
        }

        // maybe use type force type
        let var_type = match var {
            LuaVarExpr::NameExpr(name_expr) => {
                let decl_id = LuaDeclId::new(cache.get_file_id(), name_expr.get_position());
                let type_cache = db.get_type_index().get_type_cache(&decl_id.into());
                if let Some(typ_cache) = type_cache {
                    Some(typ_cache.as_type().clone())
                } else {
                    None
                }
            }
            LuaVarExpr::IndexExpr(index_expr) => {
                let member_id = LuaMemberId::new(index_expr.get_syntax_id(), cache.get_file_id());
                let type_cache = db.get_type_index().get_type_cache(&member_id.into());
                if let Some(typ_cache) = type_cache {
                    Some(typ_cache.as_type().clone())
                } else {
                    None
                }
            }
        };

        if let Some(var_type) = var_type {
            return Ok(ResultTypeOrContinue::Result(var_type));
        }

        // infer from expr
        let expr_type = match exprs.get(i) {
            Some(expr) => {
                let expr_type = infer_expr(db, cache, expr.clone())?;
                match &expr_type {
                    LuaType::Variadic(variadic) => match variadic.get_type(0) {
                        Some(typ) => typ.clone(),
                        None => return Ok(ResultTypeOrContinue::Continue),
                    },
                    _ => expr_type,
                }
            }
            None => {
                let expr_len = exprs.len();
                if expr_len == 0 {
                    return Ok(ResultTypeOrContinue::Continue);
                }

                let last_expr = exprs[expr_len - 1].clone();
                let last_expr_type = infer_expr(db, cache, last_expr)?;
                if let LuaType::Variadic(variadic) = last_expr_type {
                    let idx = i - expr_len + 1;
                    match variadic.get_type(idx) {
                        Some(typ) => typ.clone(),
                        None => return Ok(ResultTypeOrContinue::Continue),
                    }
                } else {
                    return Ok(ResultTypeOrContinue::Continue);
                }
            }
        };

        let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
        let antecedent_type =
            get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;

        return Ok(ResultTypeOrContinue::Result(
            narrow_down_type(db, antecedent_type, expr_type.clone()).unwrap_or(expr_type),
        ));
    }

    Ok(ResultTypeOrContinue::Continue)
}

fn get_type_at_assert_call(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    assert_call: LuaCallExpr,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let call_arg_list = match assert_call.get_args_list() {
        Some(args) => args,
        None => return Ok(ResultTypeOrContinue::Continue),
    };

    for arg in call_arg_list.get_args() {
        if let Some(ref_decl_id) = get_var_expr_var_ref_id(db, cache, arg.clone()) {
            if ref_decl_id == *var_ref_id {
                let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
                let antecedent_type =
                    get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;
                let result_type = remove_false_or_nil(antecedent_type);

                return Ok(ResultTypeOrContinue::Result(result_type));
            }
        }
    }

    Ok(ResultTypeOrContinue::Continue)
}
