use emmylua_parser::{LuaAst, LuaAstNode, LuaBlock, LuaIfStat};

use crate::{DbIndex, TypeAssertion};

use super::VarTrace;

pub fn broadcast_outside_block(
    _: &mut DbIndex,
    var_trace: &mut VarTrace,
    block: LuaBlock,
    type_assert: TypeAssertion,
) -> Option<()> {
    let parent = block.get_parent::<LuaAst>()?;
    match &parent {
        LuaAst::LuaIfStat(if_stat) => {
            // let trace_id = UnResolveTraceId::If(if_stat.clone());
            // let trace_info = super::UnResolveTraceInfo {
            //     type_assert,
            //     var_ref_id: var_trace.get_var_ref_id().clone(),
            // };
            // unresolve_trace.add_unresolve_trace(trace_id, trace_info);
            // broadcast_down_after_node(db, var_trace, unresolve_trace, parent, type_assert, false);
        }
        LuaAst::LuaElseIfClauseStat(_) | LuaAst::LuaElseClauseStat(_) => {
            if let Some(if_stat) = parent.get_parent::<LuaIfStat>() {
                // let trace_id = UnResolveTraceId::If(if_stat.clone());
                // let trace_info = super::UnResolveTraceInfo {
                //     type_assert,
                //     var_ref_id: var_trace.get_var_ref_id().clone(),
                // };
                // unresolve_trace.add_unresolve_trace(trace_id, trace_info);
            }
        }
        _ => {}
    }

    Some(())
}
