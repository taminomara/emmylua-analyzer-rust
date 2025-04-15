use emmylua_parser::{LuaAstNode, LuaChunk, LuaDocTagCast, LuaVarExpr};
use rowan::{TextRange, TextSize};

use crate::{LuaFlowId, VarRefId};
use std::collections::HashMap;

use super::flow_node::FlowNode;

#[derive(Debug)]
pub struct FlowTree {
    current_flow_id: LuaFlowId,
    flow_id_stack: Vec<LuaFlowId>,
    flow_nodes: HashMap<LuaFlowId, FlowNode>,
    var_flow_ref: HashMap<VarRefId, Vec<(VarRefNode, LuaFlowId)>>,
    var_node_to_id: HashMap<VarRefNode, VarRefId>,
    root_flow_id: LuaFlowId,
}

#[allow(unused)]
impl FlowTree {
    pub fn new(root: LuaChunk) -> FlowTree {
        let current_flow_id = LuaFlowId::from_chunk(root.clone());
        let mut builder = FlowTree {
            current_flow_id,
            flow_id_stack: Vec::new(),
            flow_nodes: HashMap::new(),
            var_flow_ref: HashMap::new(),
            var_node_to_id: HashMap::new(),
            root_flow_id: current_flow_id,
        };

        builder.flow_nodes.insert(
            current_flow_id,
            FlowNode::new(current_flow_id, current_flow_id.get_range(), None),
        );
        builder
    }

    pub fn enter_flow(&mut self, flow_id: LuaFlowId, range: TextRange) {
        let parent = self.current_flow_id;
        self.flow_id_stack.push(flow_id);
        self.current_flow_id = flow_id;
        self.flow_nodes
            .insert(flow_id, FlowNode::new(flow_id, range, Some(parent)));
        if let Some(parent_tree) = self.flow_nodes.get_mut(&parent) {
            parent_tree.add_child(flow_id);
        }
    }

    pub fn pop_flow(&mut self) {
        self.flow_id_stack.pop();
        self.current_flow_id = self
            .flow_id_stack
            .last()
            .unwrap_or(&self.root_flow_id)
            .clone();
    }

    pub fn add_flow_node(&mut self, ref_id: VarRefId, ref_node: VarRefNode) -> Option<()> {
        let flow_id = self.current_flow_id;
        self.var_flow_ref
            .entry(ref_id.clone())
            .or_insert_with(Vec::new)
            .push((ref_node.clone(), flow_id));
        self.var_node_to_id.insert(ref_node, ref_id);

        Some(())
    }

    pub fn get_flow_node(&self, flow_id: LuaFlowId) -> Option<&FlowNode> {
        self.flow_nodes.get(&flow_id)
    }

    pub fn get_flow_node_mut(&mut self, flow_id: LuaFlowId) -> Option<&mut FlowNode> {
        self.flow_nodes.get_mut(&flow_id)
    }

    pub fn get_current_flow_node(&self) -> Option<&FlowNode> {
        self.flow_nodes.get(&self.current_flow_id)
    }

    pub fn get_current_flow_node_mut(&mut self) -> Option<&mut FlowNode> {
        self.flow_nodes.get_mut(&self.current_flow_id)
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
            if let Some(node) = self.flow_nodes.get(&flow_id) {
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

    pub fn get_var_ref_nodes(
        &self,
        var_ref_id: &VarRefId,
    ) -> Option<&Vec<(VarRefNode, LuaFlowId)>> {
        self.var_flow_ref.get(var_ref_id)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum VarRefNode {
    UseRef(LuaVarExpr),
    AssignRef(LuaVarExpr),
    CastRef(LuaDocTagCast),
}

#[allow(unused)]
impl VarRefNode {
    pub fn get_range(&self) -> TextRange {
        match self {
            VarRefNode::UseRef(expr) => expr.get_range(),
            VarRefNode::AssignRef(expr) => expr.get_range(),
            VarRefNode::CastRef(tag_cast) => tag_cast.get_range(),
        }
    }

    pub fn get_position(&self) -> TextSize {
        match self {
            VarRefNode::UseRef(expr) => expr.get_position(),
            VarRefNode::AssignRef(expr) => expr.get_position(),
            VarRefNode::CastRef(tag_cast) => tag_cast.get_position(),
        }
    }

    pub fn is_use_ref(&self) -> bool {
        matches!(self, VarRefNode::UseRef(_))
    }

    pub fn is_assign_ref(&self) -> bool {
        matches!(self, VarRefNode::AssignRef(_))
    }

    pub fn is_cast_ref(&self) -> bool {
        matches!(self, VarRefNode::CastRef(_))
    }
}
