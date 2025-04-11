use emmylua_parser::{LuaAst, LuaAstNode, LuaBlock};

use crate::{DbIndex, LuaFlowChain, TypeAssertion, VarRefId};

use super::broadcast_down::broadcast_down;

pub fn broadcast_outside(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    var_ref_id: &VarRefId,
    node: LuaBlock,
    type_assert: TypeAssertion,
) -> Option<()> {
    let parent = node.get_parent::<LuaAst>()?;
    match &parent {
        LuaAst::LuaIfStat(_)
        | LuaAst::LuaDoStat(_)
        | LuaAst::LuaWhileStat(_)
        | LuaAst::LuaForStat(_)
        | LuaAst::LuaForRangeStat(_)
        | LuaAst::LuaRepeatStat(_) => {
            broadcast_down(db, flow_chain, var_ref_id, parent, type_assert, false);
        }
        LuaAst::LuaElseIfClauseStat(_) | LuaAst::LuaElseClauseStat(_) => {
            broadcast_down(
                db,
                flow_chain,
                var_ref_id,
                parent.get_parent::<LuaAst>()?,
                type_assert,
                false,
            );
        }
        _ => {}
    }

    Some(())
}
