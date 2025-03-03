mod assign_stats;
mod flow_builder;
mod flow_tree;
mod var_analyze;

use crate::{db_index::DbIndex, profile::Profile, LuaFlowId};
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaCallExpr, LuaChunk, LuaExpr, LuaIndexExpr, LuaNameExpr,
    LuaTokenKind, PathTrait,
};
use flow_builder::FlowBuilder;
use flow_tree::{FlowNode, FlowRefNode, FlowTree};
use rowan::WalkEvent;
use smol_str::SmolStr;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let _p = Profile::cond_new("flow analyze", context.tree_list.len() > 1);
    let tree_list = context.tree_list.clone();
    // build decl and ref flow chain
    for in_filed_tree in &tree_list {
        let flow_trees = build_flow(in_filed_tree.value.clone());
        analyze_flow(db, flow_trees);
    }
}

fn build_flow(root: LuaChunk) -> Vec<(LuaFlowId, FlowTree)> {
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
                LuaAst::LuaCallExpr(call_expr) => {
                    build_call_expr_flow(&mut builder, call_expr);
                }
                LuaAst::LuaReturnStat(return_stat) => {
                    builder.add_flow_node(
                        return_stat.get_position(),
                        FlowNode::Return(return_stat.clone()),
                    );
                }
                LuaAst::LuaBreakStat(break_stat) => {
                    builder.add_flow_node(
                        break_stat.get_position(),
                        FlowNode::Break(break_stat.clone()),
                    );
                }
                LuaAst::LuaGotoStat(goto_stat) => {
                    builder
                        .add_flow_node(goto_stat.get_position(), FlowNode::Goto(goto_stat.clone()));
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
                    name_expr.get_position(),
                    FlowNode::AssignRef(FlowRefNode {
                        path: SmolStr::new(&name_expr.get_access_path()?),
                        node: LuaExpr::NameExpr(name_expr.clone()),
                    }),
                );
            }
        }
        _ => {}
    }
    builder.add_flow_node(
        name_expr.get_position(),
        FlowNode::UseRef(FlowRefNode {
            path: SmolStr::new(&name_expr.get_access_path()?),
            node: LuaExpr::NameExpr(name_expr.clone()),
        }),
    );
    Some(())
}

fn build_index_expr_flow(builder: &mut FlowBuilder, index_expr: LuaIndexExpr) -> Option<()> {
    let parent = index_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIndexExpr(_) | LuaAst::LuaCallExpr(_) => return None,
        _ => {}
    }
    builder.add_flow_node(
        index_expr.get_position(),
        FlowNode::UseRef(FlowRefNode {
            path: SmolStr::new(&index_expr.get_access_path()?),
            node: LuaExpr::IndexExpr(index_expr.clone()),
        }),
    );

    Some(())
}

fn build_call_expr_flow(builder: &mut FlowBuilder, call_expr: LuaCallExpr) -> Option<()> {
    let prefix = call_expr.get_prefix_expr()?;
    if let LuaExpr::NameExpr(name) = prefix {
        let name = name.get_name_text()?;
        match name.as_str() {
            "assert" => {
                builder.add_flow_node(
                    call_expr.get_position(),
                    FlowNode::Assert(call_expr.clone()),
                );
            }
            "error" => {
                builder.add_flow_node(
                    call_expr.get_position(),
                    FlowNode::ThrowError(call_expr.clone()),
                );
            }
            _ => {}
        }
    }

    Some(())
}

#[allow(unused)]
fn analyze_flow(db: &mut DbIndex, flow_trees: Vec<(LuaFlowId, FlowTree)>) {
    for (flow_id, flow_tree) in flow_trees {}
}
