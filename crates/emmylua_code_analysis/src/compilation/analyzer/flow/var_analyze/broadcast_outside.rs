use std::sync::Arc;

use emmylua_parser::{LuaAst, LuaAstNode, LuaBlock, LuaIfStat};

use crate::DbIndex;

use super::{unresolve_trace::UnResolveTraceId, VarTrace, VarTraceInfo};

pub fn broadcast_outside_block(
    _: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    block: LuaBlock,
) -> Option<()> {
    let parent = block.get_parent::<LuaAst>()?;
    match &parent {
        LuaAst::LuaIfStat(if_stat) => {
            let trace_id = UnResolveTraceId::If(if_stat.clone());
            var_trace.add_unresolve_trace(trace_id, trace_info);
        }
        LuaAst::LuaElseIfClauseStat(_) | LuaAst::LuaElseClauseStat(_) => {
            if let Some(if_stat) = parent.get_parent::<LuaIfStat>() {
                let trace_id = UnResolveTraceId::If(if_stat.clone());
                var_trace.add_unresolve_trace(trace_id, trace_info);
            }
        }
        _ => {}
    }

    Some(())
}
