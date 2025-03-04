mod assign_stats;
mod flow_builder;
mod flow_nodes;
mod var_analyze;

use crate::{db_index::DbIndex, profile::Profile, FileId, LuaFlowChain, LuaFlowId};
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaChunk, LuaExpr, LuaIndexExpr, LuaNameExpr, LuaTokenKind, PathTrait
};
use flow_builder::FlowBuilder;
use flow_nodes::{FlowNode, FlowNodes, FlowRef};
use rowan::WalkEvent;
use var_analyze::analyze_ref_expr;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("flow analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let flow_trees = build_flow_cache(in_filed_tree.value.clone());
        analyze_flow(db, in_filed_tree.file_id, flow_trees);
    }
}

fn build_flow_cache(root: LuaChunk) -> Vec<(LuaFlowId, FlowNodes)> {
    let mut builder = FlowBuilder::new();
    for walk_node in root.walk_descendants::<LuaAst>() {
        match walk_node {
            WalkEvent::Enter(node) => match node {
                LuaAst::LuaClosureExpr(closure) => {
                    builder.enter_flow(LuaFlowId::from_closure(closure));
                }
                LuaAst::LuaNameExpr(name_expr) => {
                    build_name_expr_flow(&mut builder, name_expr);
                }
                LuaAst::LuaIndexExpr(index_expr) => {
                    build_index_expr_flow(&mut builder, index_expr);
                }
                _ => {}
            },
            WalkEvent::Leave(node) => match node {
                LuaAst::LuaClosureExpr(_) => builder.pop_flow(),
                _ => {}
            },
        }
    }

    builder.finish()
}

fn build_name_expr_flow(builder: &mut FlowBuilder, name_expr: LuaNameExpr) -> Option<()> {
    let parent = name_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIndexExpr(_) | LuaAst::LuaCallExpr(_) => return None,
        LuaAst::LuaAssignStat(assign_stat) => {
            let eq_pos = assign_stat
                .token_by_kind(LuaTokenKind::TkEq)?
                .get_position();
            if name_expr.get_position() < eq_pos {
                builder.add_flow_node(
                    &name_expr.get_access_path()?,
                    FlowNode::AssignRef(FlowRef::NameExpr(name_expr.clone())),
                );
            }
        }
        _ => {}
    }
    builder.add_flow_node(
        &name_expr.get_access_path()?,
        FlowNode::UseRef(FlowRef::NameExpr(name_expr.clone())),
    );
    Some(())
}

fn build_index_expr_flow(builder: &mut FlowBuilder, index_expr: LuaIndexExpr) -> Option<()> {
    let parent = index_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIndexExpr(_) | LuaAst::LuaCallExpr(_) => return None,
        LuaAst::LuaAssignStat(assign_stat) => {
            let eq_pos = assign_stat
                .token_by_kind(LuaTokenKind::TkEq)?
                .get_position();
            if index_expr.get_position() < eq_pos {
                builder.add_flow_node(
                    &index_expr.get_access_path()?,
                    FlowNode::AssignRef(FlowRef::IndexExpr(index_expr.clone())),
                );
            }
        }
        _ => {}
    }
    builder.add_flow_node(
        &index_expr.get_access_path()?,
        FlowNode::UseRef(FlowRef::IndexExpr(index_expr.clone())),
    );

    Some(())
}

// fn get_stat_special(builder: &mut FlowBuilder, call_expr: LuaCallExpr) -> Option<()> {
//     // let prefix = call_expr.get_prefix_expr()?;
//     // if let LuaExpr::NameExpr(name) = prefix {
//     //     let name = name.get_name_text()?;
//     //     match name.as_str() {
//     //         "assert" => {
//     //             builder.add_flow_node(
//     //                 call_expr.get_position(),
//     //                 FlowNode::Assert(call_expr.clone()),
//     //             );
//     //         }
//     //         "error" => {
//     //             builder.add_flow_node(
//     //                 call_expr.get_position(),
//     //                 FlowNode::ThrowError(call_expr.clone()),
//     //             );
//     //         }
//     //         _ => {}
//     //     }
//     // }

//     Some(())
// }

#[allow(unused)]
fn analyze_flow(db: &mut DbIndex, file_id: FileId, flow_caches: Vec<(LuaFlowId, FlowNodes)>) {
    for (flow_id, cache) in flow_caches {
        let nodes = cache.get_var_flow_nodes();
        let mut flow_chain = LuaFlowChain::new(flow_id);
        for (var_path, flow_nodes) in nodes {
            for flow_node in flow_nodes {
                match flow_node {
                    FlowNode::UseRef(ref_node) => {
                        let expr = match ref_node {
                            FlowRef::NameExpr(expr) => LuaExpr::NameExpr(expr.clone()),
                            FlowRef::IndexExpr(expr) => LuaExpr::IndexExpr(expr.clone()),
                        };

                        analyze_ref_expr(db, &mut flow_chain, &expr, var_path);
                    }
                    FlowNode::AssignRef(ref_node) => {
                        // let path = &ref_node.path;
                        // let expr = &ref_node.node;
                        // analyze_ref_expr(db, &mut flow_chain, expr, path);
                    }
                }
            }
        }

        db.get_flow_index_mut().add_flow_chain(file_id, flow_chain);
    }
}
