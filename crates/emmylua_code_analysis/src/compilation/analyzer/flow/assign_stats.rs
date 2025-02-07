#[allow(unused_imports)]
use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaNameExpr, LuaSyntaxId, LuaSyntaxKind};

use crate::{DeclReference, LuaFlowChain};

use super::FlowAnalyzer;

#[allow(unused)]
pub fn infer_from_assign_stats(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    decl_refs: Vec<&DeclReference>,
) -> Option<()> {
    // for decl_ref in decl_refs {
    //     let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), decl_ref.range.clone());
    //     let name_expr = LuaNameExpr::cast(syntax_id.to_node_from_root(analyzer.root.syntax())?)?;
    //     let assign_stat = name_expr.get_parent::<LuaAssignStat>()?;
        
    // }


    Some(())
}

