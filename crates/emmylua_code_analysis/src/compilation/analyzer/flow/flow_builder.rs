use std::collections::{HashMap, HashSet};

use rowan::TextSize;

use crate::LuaFlowId;

use super::flow_tree::{FlowNode, FlowTree};

#[derive(Debug)]
pub struct FlowBuilder {
    current_flow_id: LuaFlowId,
    flow_id_stack: Vec<LuaFlowId>,
    may_has_flow_node: HashSet<TextSize>,
    flow_trees: HashMap<LuaFlowId, FlowTree>,
}

impl FlowBuilder {
    pub fn new() -> FlowBuilder {
    
        let mut builder = FlowBuilder {
            current_flow_id: LuaFlowId::chunk(),
            flow_id_stack: Vec::new(),
            may_has_flow_node: HashSet::new(),
            flow_trees: HashMap::new(),
        };

        builder.enter_flow(LuaFlowId::chunk());
        builder
    }

    pub fn enter_flow(&mut self, flow_id: LuaFlowId) {
        self.flow_id_stack.push(flow_id);
        self.current_flow_id = flow_id;
        self.flow_trees.insert(flow_id, FlowTree::new());
    }

    pub fn pop_flow(&mut self) {
        self.flow_id_stack.pop();
        self.current_flow_id = *self.flow_id_stack.last().unwrap();
    }

    pub fn add_flow_node(&mut self, pos: TextSize, flow_type: FlowNode) -> Option<()> {
        let flow_id = self.flow_id_stack.last()?;
        let flow_tree = self.flow_trees.get_mut(flow_id)?;
        flow_tree.add_node(pos, flow_type);

        Some(())
    }

    pub fn finish(self) -> Vec<(LuaFlowId, FlowTree)> {
        self.flow_trees.into_iter().collect()
    }
}
