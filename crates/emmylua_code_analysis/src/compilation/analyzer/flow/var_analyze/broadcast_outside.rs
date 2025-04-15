use emmylua_parser::{LuaAst, LuaAstNode, LuaBlock, LuaIfStat};

use crate::{DbIndex, TypeAssertion};

use super::{unresolve_trace::UnResolveTraceId, VarTrace};

pub fn broadcast_outside_block(
    _: &mut DbIndex,
    var_trace: &mut VarTrace,
    block: LuaBlock,
    type_assert: TypeAssertion,
) -> Option<()> {
    let parent = block.get_parent::<LuaAst>()?;
    match &parent {
        LuaAst::LuaIfStat(if_stat) => {
            let trace_id = UnResolveTraceId::If(if_stat.clone());
            var_trace.add_unresolve_trace(trace_id, type_assert);
        }
        LuaAst::LuaElseIfClauseStat(_) | LuaAst::LuaElseClauseStat(_) => {
            if let Some(if_stat) = parent.get_parent::<LuaIfStat>() {
                let trace_id = UnResolveTraceId::If(if_stat.clone());
                var_trace.add_unresolve_trace(trace_id, type_assert);
            }
        }
        _ => {}
    }

    Some(())
}
