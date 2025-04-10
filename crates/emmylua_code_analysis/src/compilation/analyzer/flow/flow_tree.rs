use emmylua_parser::{LuaDocTagCast, LuaVarExpr};
use rowan::{TextRange, TextSize};

use crate::{LuaFlowId, VarRefId};
use std::collections::HashMap;

use super::flow_node::FlowNode;

#[derive(Debug)]
pub struct FlowTree {
    current_flow_id: LuaFlowId,
    flow_id_stack: Vec<LuaFlowId>,
    flow_trees: HashMap<LuaFlowId, FlowNode>,
    var_flow_ref: HashMap<VarRefId, Vec<(VarRefNode, LuaFlowId)>>,
    root_flow_id: LuaFlowId,
}

impl FlowTree {
    pub fn new(document_range: TextRange) -> FlowTree {
        let mut builder = FlowTree {
            current_flow_id: LuaFlowId::chunk(),
            flow_id_stack: Vec::new(),
            flow_trees: HashMap::new(),
            var_flow_ref: HashMap::new(),
            root_flow_id: LuaFlowId::chunk(),
        };

        let flow_id = LuaFlowId::chunk();
        builder
            .flow_trees
            .insert(flow_id, FlowNode::new(flow_id, document_range, None));
        builder
    }

    pub fn enter_flow(&mut self, flow_id: LuaFlowId, range: TextRange) {
        let parent = self.current_flow_id;
        self.flow_id_stack.push(flow_id);
        self.current_flow_id = flow_id;
        self.flow_trees
            .insert(flow_id, FlowNode::new(flow_id, range, Some(parent)));
        if let Some(parent_tree) = self.flow_trees.get_mut(&parent) {
            parent_tree.add_child(flow_id);
        }
    }

    pub fn pop_flow(&mut self) {
        self.flow_id_stack.pop();
        self.current_flow_id = *self.flow_id_stack.last().unwrap();
    }

    pub fn add_flow_node(&mut self, ref_id: VarRefId, ref_node: VarRefNode) -> Option<()> {
        let flow_id = self.current_flow_id;
        self
            .var_flow_ref
            .entry(ref_id)
            .or_insert_with(Vec::new)
            .push((ref_node, flow_id));

        Some(())
    }

    pub fn get_flow_tree(&self, flow_id: LuaFlowId) -> Option<&FlowNode> {
        self.flow_trees.get(&flow_id)
    }

    pub fn get_flow_tree_mut(&mut self, flow_id: LuaFlowId) -> Option<&mut FlowNode> {
        self.flow_trees.get_mut(&flow_id)
    }

    pub fn get_current_flow_tree(&self) -> Option<&FlowNode> {
        self.flow_trees.get(&self.current_flow_id)
    }

    pub fn get_current_flow_tree_mut(&mut self) -> Option<&mut FlowNode> {
        self.flow_trees.get_mut(&self.current_flow_id)
    }

    pub fn get_current_flow_id(&self) -> LuaFlowId {
        self.current_flow_id
    }

    pub fn get_var_ref_ids(&self) -> Vec<VarRefId> {
        self.var_flow_ref.keys().cloned().collect()
    }

    pub fn get_flow_id_from_position(&self, position: TextSize) -> LuaFlowId {
        let mut result = self.root_flow_id;
        let mut stack = vec![self.root_flow_id];

        while let Some(flow_id) = stack.pop() {
            if let Some(node) = self.flow_trees.get(&flow_id) {
                if node.get_range().contains(position) {
                    result = flow_id;
                    if node.get_children().is_empty() {
                        break;
                    }

                    stack.extend(node.get_children().iter().rev().copied());
                }
            }
        }

        result
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VarRefNode {
    UseRef(LuaVarExpr),
    AssignRef(LuaVarExpr),
    CastRef(LuaDocTagCast),
}
