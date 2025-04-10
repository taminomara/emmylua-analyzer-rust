mod build_flow_tree;
mod cast_analyze;
mod flow_node;
mod flow_tree;
mod var_analyze;

use crate::{db_index::DbIndex, profile::Profile, FileId, LuaDeclId, VarRefId};
use build_flow_tree::build_flow_tree;
use flow_node::{BlockId, FlowNode};
use flow_tree::{FlowTree, VarRefNode};
use smol_str::SmolStr;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("flow analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let flow_tree = build_flow_tree(db, in_filed_tree.file_id, in_filed_tree.value.clone());
        analyze_flow(db, in_filed_tree.file_id, flow_tree, context);
    }
}

fn analyze_flow(
    db: &mut DbIndex,
    file_id: FileId,
    flow_tree: FlowTree,
    context: &mut AnalyzeContext,
) {
    let var_ref_ids = flow_tree.get_var_ref_ids();
    for var_ref_id in var_ref_ids {
        match var_ref_id {
            VarRefId::DeclId(decl_id) => {
                analyze_decl_flow(db, &flow_tree, file_id, decl_id, context);
            }
            VarRefId::Name(_) => {}
        }
    }
}

fn analyze_decl_flow(
    db: &mut DbIndex,
    flow_tree: &FlowTree,
    file_id: FileId,
    decl_id: LuaDeclId,
    context: &mut AnalyzeContext,
) {
    let start_flow_id = flow_tree.get_flow_id_from_position(decl_id.position);

    // for (flow_id, tree) in flow_trees {
    //     let nodes = tree.get_var_flow_nodes();
    //     let mut flow_chain = LuaFlowChain::new(flow_id);
    //     for (var_ref_id, var_ref_nodes) in nodes {
    //         for flow_node in var_ref_nodes {
    //             match flow_node {
    //                 VarRefNode::UseRef(var_expr) => {
    //                     analyze_ref_expr(db, &mut flow_chain, &var_expr, var_ref_id);
    //                 }
    //                 VarRefNode::AssignRef(var_expr) => {
    //                     analyze_ref_assign(db, &mut flow_chain, &var_expr, var_ref_id, file_id);
    //                 }
    //                 VarRefNode::CastRef(tag_cast) => {
    //                     analyze_cast(
    //                         &mut flow_chain,
    //                         file_id,
    //                         var_ref_id,
    //                         tag_cast.clone(),
    //                         context,
    //                     );
    //                 }
    //             }
    //         }
    //     }

    //     db.get_flow_index_mut().add_flow_chain(file_id, flow_chain);
    // }
}
