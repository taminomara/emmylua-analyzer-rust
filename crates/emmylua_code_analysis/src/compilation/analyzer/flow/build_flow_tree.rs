use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaBlock, LuaBreakStat, LuaChunk, LuaDocTagCast, LuaGotoStat,
    LuaIndexExpr, LuaLabelStat, LuaLoopStat, LuaNameExpr, LuaTokenKind, PathTrait,
};
use rowan::{TextRange, WalkEvent};
use smol_str::SmolStr;

use crate::{AnalyzeError, DbIndex, DiagnosticCode, FileId, LuaDeclId, LuaFlowId, VarRefId};

use super::{
    flow_node::BlockId,
    flow_tree::{FlowTree, VarRefNode},
};

pub fn build_flow_tree(db: &mut DbIndex, file_id: FileId, root: LuaChunk) -> FlowTree {
    let range = root.get_range();
    let mut flow_tree = FlowTree::new(range);
    let mut goto_vecs: Vec<(LuaFlowId, LuaGotoStat)> = vec![];
    for walk_node in root.walk_descendants::<LuaAst>() {
        match walk_node {
            WalkEvent::Enter(node) => match node {
                LuaAst::LuaClosureExpr(closure) => {
                    flow_tree.enter_flow(
                        LuaFlowId::from_closure(closure.clone()),
                        closure.get_range(),
                    );
                }
                LuaAst::LuaNameExpr(name_expr) => {
                    build_name_expr_flow(db, &mut flow_tree, file_id, name_expr);
                }
                LuaAst::LuaIndexExpr(index_expr) => {
                    build_index_expr_flow(db, &mut flow_tree, file_id, index_expr);
                }
                LuaAst::LuaDocTagCast(cast) => {
                    build_cast_flow(db, &mut flow_tree, file_id, cast);
                }
                LuaAst::LuaLabelStat(label) => {
                    build_label_flow(db, &mut flow_tree, file_id, label);
                }
                LuaAst::LuaGotoStat(goto_stat) => {
                    let current_flow_id = flow_tree.get_current_flow_id();
                    goto_vecs.push((current_flow_id, goto_stat.clone()));
                }
                LuaAst::LuaBreakStat(break_stat) => {
                    build_break_flow(db, &mut flow_tree, file_id, break_stat);
                }
                _ => {}
            },
            WalkEvent::Leave(node) => match node {
                LuaAst::LuaClosureExpr(_) => flow_tree.pop_flow(),
                _ => {}
            },
        }
    }

    for (flow_id, goto_stat) in goto_vecs {
        build_goto_flow(db, &mut flow_tree, file_id, goto_stat, flow_id);
    }

    flow_tree
}

fn build_name_expr_flow(
    db: &DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let parent = name_expr.get_parent::<LuaAst>()?;
    let mut is_assign = false;
    match parent {
        LuaAst::LuaIndexExpr(_) | LuaAst::LuaCallExpr(_) | LuaAst::LuaFuncStat(_) => return None,
        LuaAst::LuaAssignStat(assign_stat) => {
            let eq_pos = assign_stat
                .token_by_kind(LuaTokenKind::TkAssign)?
                .get_position();
            let decl_id = LuaDeclId::new(file_id, name_expr.get_position());
            if db.get_decl_index().get_decl(&decl_id).is_some() {
                return None;
            }

            if name_expr.get_position() < eq_pos {
                is_assign = true;
            }
        }
        _ => {}
    }
    let mut ref_id: Option<VarRefId> = None;
    if let Some(local_refs) = db.get_reference_index().get_local_reference(&file_id) {
        if let Some(decl_id) = local_refs.get_decl_id(&name_expr.get_range()) {
            ref_id = Some(VarRefId::DeclId(decl_id.clone()));
        }
    }

    if ref_id.is_none() {
        ref_id = Some(VarRefId::Name(SmolStr::new(&name_expr.get_name_text()?)));
    }

    let ref_id = ref_id?;
    if is_assign {
        builder.add_flow_node(ref_id, VarRefNode::AssignRef(name_expr.into()));
    } else {
        builder.add_flow_node(ref_id, VarRefNode::UseRef(name_expr.into()));
    }

    Some(())
}

fn build_index_expr_flow(
    db: &DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    index_expr: LuaIndexExpr,
) -> Option<()> {
    let parent = index_expr.get_parent::<LuaAst>()?;
    let mut is_assign = false;
    match parent {
        LuaAst::LuaIndexExpr(_) | LuaAst::LuaCallExpr(_) | LuaAst::LuaFuncStat(_) => return None,
        LuaAst::LuaAssignStat(assign_stat) => {
            let eq_pos = assign_stat
                .token_by_kind(LuaTokenKind::TkAssign)?
                .get_position();

            let decl_id = LuaDeclId::new(file_id, index_expr.get_position());
            if db.get_decl_index().get_decl(&decl_id).is_some() {
                return None;
            }

            if index_expr.get_position() < eq_pos {
                is_assign = true;
            }
        }
        _ => {}
    }

    let ref_id = VarRefId::Name(SmolStr::new(&index_expr.get_access_path()?));
    if is_assign {
        builder.add_flow_node(ref_id, VarRefNode::AssignRef(index_expr.into()));
    } else {
        builder.add_flow_node(ref_id, VarRefNode::UseRef(index_expr.into()));
    }

    Some(())
}

fn build_cast_flow(
    db: &DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    tag_cast: LuaDocTagCast,
) -> Option<()> {
    let name_token = tag_cast.get_name_token()?;
    let decl_tree = db.get_decl_index().get_decl_tree(&file_id)?;
    let text = name_token.get_name_text();
    if let Some(decl) = decl_tree.find_local_decl(text, name_token.get_position()) {
        let decl_id = decl.get_id();
        builder.add_flow_node(
            VarRefId::DeclId(decl_id),
            VarRefNode::CastRef(tag_cast.clone()),
        );
    } else {
        let ref_id = VarRefId::Name(SmolStr::new(text));
        if db
            .get_decl_index()
            .get_decl(&LuaDeclId::new(file_id, name_token.get_position()))
            .is_none()
        {
            builder.add_flow_node(ref_id, VarRefNode::CastRef(tag_cast.clone()));
        }
    }

    Some(())
}

fn build_label_flow(
    db: &mut DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    label: LuaLabelStat,
) -> Option<()> {
    let decl_id = LuaDeclId::new(file_id, label.get_position());
    if db.get_decl_index().get_decl(&decl_id).is_some() {
        return None;
    }

    let flow_tree = builder.get_current_flow_node_mut()?;
    let label_token = label.get_label_name_token()?;
    let label_name = label_token.get_name_text();
    let block = label.get_parent::<LuaBlock>()?;
    let block_id = BlockId::from_block(block);
    if flow_tree.is_exist_label_in_same_block(label_name, block_id) {
        db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(
                DiagnosticCode::SyntaxError,
                &t!(
                    "Label `%{name}` already exists in the same block",
                    name = label_name
                ),
                label.get_range(),
            ),
        );
        return None;
    }

    flow_tree.add_label_ref(label_name, label);
    Some(())
}

fn build_goto_flow(
    db: &mut DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    goto_stat: LuaGotoStat,
    flow_id: LuaFlowId,
) -> Option<()> {
    let flow_node = builder.get_flow_node_mut(flow_id)?;
    let label_token = goto_stat.get_label_name_token()?;
    let label_name = label_token.get_name_text();
    let label = flow_node.find_label(label_name, goto_stat.clone());
    if label.is_none() {
        db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(
                DiagnosticCode::SyntaxError,
                &t!("Label `%{name}` not found", name = label_name),
                label_token.get_range(),
            ),
        );
    }

    let label = label?;
    let label_block = label.get_parent::<LuaBlock>()?;
    let label_end_pos = label_block.get_range().end();
    let label_block_end_pos = label_block.get_range().end();
    if label_end_pos < label_block_end_pos {
        let new_range = TextRange::new(label_end_pos, label_block_end_pos);
        flow_node.add_jump_to_range(goto_stat.get_syntax_id(), new_range);
    }

    Some(())
}

fn build_break_flow(
    db: &mut DbIndex,
    builder: &mut FlowTree,
    file_id: FileId,
    break_stat: LuaBreakStat,
) -> Option<()> {
    let flow_tree = builder.get_current_flow_node_mut()?;
    let first_loop_stat = break_stat.ancestors::<LuaLoopStat>().next();
    if first_loop_stat.is_none() {
        db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(
                DiagnosticCode::SyntaxError,
                &t!("`break` statement not in a loop"),
                break_stat.get_range(),
            ),
        );
        return None;
    }
    let loop_stat = first_loop_stat?;
    let block = loop_stat.get_parent_block()?;
    let loop_end_pos = loop_stat.get_range().end();
    let block_end_pos = block.get_range().end();
    if loop_end_pos < block_end_pos {
        let new_range = TextRange::new(loop_end_pos, block_end_pos);
        flow_tree.add_jump_to_range(break_stat.get_syntax_id(), new_range);
    }

    Some(())
}
