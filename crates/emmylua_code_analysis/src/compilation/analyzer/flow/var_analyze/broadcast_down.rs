use emmylua_parser::{LuaAst, LuaAstNode, LuaBlock, LuaStat};
use rowan::TextRange;

use crate::{DbIndex, TypeAssertion};

use super::{broadcast_outside::broadcast_outside_block, VarTrace};

pub fn broadcast_down_after_node(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    node: LuaAst,
    type_assert: TypeAssertion,
    continue_broadcast_outside: bool,
) -> Option<()> {
    let parent_block = node.get_parent::<LuaBlock>()?;
    let parent_block_range = parent_block.get_range();
    let range = node.get_range();
    if range.end() < parent_block_range.end() {
        let range = TextRange::new(range.end(), parent_block_range.end());
        var_trace.add_assert(type_assert.clone(), range);
    }

    if is_block_has_return(Some(parent_block.clone())).unwrap_or(false) {
        return Some(());
    }

    if continue_broadcast_outside {
        broadcast_outside_block(db, var_trace, parent_block, type_assert);
    }

    Some(())
}

fn is_block_has_return(block: Option<LuaBlock>) -> Option<bool> {
    if let Some(block) = block {
        for stat in block.get_stats() {
            if is_stat_change_flow(stat.clone()).unwrap_or(false) {
                return Some(true);
            }
        }
    }

    Some(false)
}

fn is_stat_change_flow(stat: LuaStat) -> Option<bool> {
    match stat {
        LuaStat::CallExprStat(call_stat) => {
            let call_expr = call_stat.get_call_expr()?;
            if call_expr.is_error() {
                return Some(true);
            }
            Some(false)
        }
        LuaStat::ReturnStat(_) => Some(true),
        LuaStat::DoStat(do_stat) => Some(is_block_has_return(do_stat.get_block()).unwrap_or(false)),
        LuaStat::BreakStat(_) => Some(true),
        _ => Some(false),
    }
}
