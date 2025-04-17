use std::sync::Arc;

use emmylua_parser::{LuaAstNode, LuaBlock, LuaStat};

use crate::DbIndex;

use super::{broadcast_outside::broadcast_outside_block, var_trace_info::VarTraceInfo, VarTrace};

pub fn broadcast_inside_condition_block(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    block: LuaBlock,
    check_broadcast_outside: bool,
) -> Option<()> {
    var_trace.add_assert(trace_info.type_assertion.clone(), block.get_range());
    if check_broadcast_outside {
        if !trace_info.check_cover_all_branch() {
            return Some(());
        }

        analyze_block_inside_condition(db, var_trace, trace_info, block.clone(), block);
    }

    Some(())
}

fn analyze_block_inside_condition(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    block: LuaBlock,
    origin_block: LuaBlock,
) -> Option<()> {
    for stat in block.get_stats() {
        match stat {
            LuaStat::CallExprStat(call_stat) => {
                let call_expr = call_stat.get_call_expr()?;
                if call_expr.is_error() {
                    let ne_type_assert = trace_info.type_assertion.get_negation()?;
                    let ne_trace_info = trace_info.with_type_assertion(ne_type_assert);
                    broadcast_outside_block(db, var_trace, ne_trace_info, origin_block.clone());
                    return Some(());
                }
            }
            LuaStat::ReturnStat(_) | LuaStat::BreakStat(_) => {
                let ne_type_assert = trace_info.type_assertion.get_negation()?;
                let ne_trace_info = trace_info.with_type_assertion(ne_type_assert);
                broadcast_outside_block(db, var_trace, ne_trace_info, origin_block.clone());
                return Some(());
            }
            LuaStat::DoStat(do_stat) => {
                analyze_block_inside_condition(
                    db,
                    var_trace,
                    trace_info.clone(),
                    do_stat.get_block()?,
                    origin_block.clone(),
                );
            }
            _ => {}
        }
    }
    Some(())
}
