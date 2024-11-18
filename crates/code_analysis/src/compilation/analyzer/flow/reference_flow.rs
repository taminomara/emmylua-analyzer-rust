use emmylua_parser::{LuaAst, LuaAstNode, LuaExpr, LuaNameExpr, LuaStat, LuaSyntaxId, LuaSyntaxKind, LuaSyntaxNode};

use crate::{compilation::analyzer::decl, db_index::LuaFlowChain};

use super::FlowAnalyzer;

pub fn analyze(analyzer: &mut FlowAnalyzer) -> Option<()> {
    let references_index = analyzer.db.get_reference_index();
    let decl_index = analyzer.db.get_decl_index();
    let refs_map = references_index.get_local_references_map(&analyzer.file_id)?;
    let tree = analyzer.tree;
    for (decl_id, ranges) in refs_map {
        let decl = decl_index.get_decl(decl_id)?;
        let mut flow_chains = LuaFlowChain::new(decl.get_id());
        for range in ranges {
            let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), range.clone());
            let node = LuaNameExpr::cast(syntax_id.to_node(tree)?)?;
            infer_flow_chain_by_name_expr(&mut flow_chains, node);
        }
    }

    Some(())
}

fn infer_flow_chain_by_name_expr(
    flow_chains: &mut LuaFlowChain,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let parent = name_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIfStat(if_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false 
        }
        _=> {}
    }

    Some(())
}
