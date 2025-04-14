use emmylua_parser::{LuaAstNode, LuaBlock, LuaStat};

use crate::{DbIndex, TypeAssertion};

use super::{broadcast_outside::broadcast_outside_block, VarTrace};

pub fn broadcast_inside_if_condition_block(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    block: LuaBlock,
    type_assert: TypeAssertion,
    check_broadcast_outside: bool,
) -> Option<()> {
    var_trace.add_assert(type_assert.clone(), block.get_range());
    if check_broadcast_outside {
        analyze_block_inside_if_condition(db, var_trace, block, type_assert);
    }

    Some(())
}

fn analyze_block_inside_if_condition(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    block: LuaBlock,
    type_assert: TypeAssertion,
) -> Option<()> {
    for stat in block.get_stats() {
        match stat {
            LuaStat::CallExprStat(call_stat) => {
                let call_expr = call_stat.get_call_expr()?;
                if call_expr.is_error() {
                    let ne_type_assert = type_assert.get_negation()?;
                    broadcast_outside_block(db, var_trace, block, ne_type_assert);
                    return Some(());
                }
            }
            LuaStat::ReturnStat(_) | LuaStat::BreakStat(_) => {
                let ne_type_assert = type_assert.get_negation()?;
                broadcast_outside_block(db, var_trace, block, ne_type_assert);
                return Some(());
            }
            LuaStat::DoStat(do_stat) => {
                analyze_block_inside_if_condition(
                    db,
                    var_trace,
                    do_stat.get_block()?,
                    type_assert.clone(),
                );
            }
            _ => {}
        }
    }
    Some(())
}
