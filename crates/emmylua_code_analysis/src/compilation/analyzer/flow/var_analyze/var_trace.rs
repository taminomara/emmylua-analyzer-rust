use std::collections::HashMap;

use emmylua_parser::LuaAstNode;
use rowan::TextRange;

use crate::{
    compilation::analyzer::flow::flow_tree::VarRefNode, LuaFlowId, TypeAssertion, VarRefId,
};

use super::unresolve_trace_id::UnResolveTraceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarTrace {
    var_ref_id: VarRefId,
    var_refs: Vec<(VarRefNode, LuaFlowId)>,
    assertions: HashMap<TextRange, Vec<TypeAssertion>>,
    current_flow_id: Option<LuaFlowId>,
    unresolve_traces: HashMap<UnResolveTraceId, TypeAssertion>,
}

#[allow(unused)]
impl VarTrace {
    pub fn new(var_ref_id: VarRefId, var_refs: Vec<(VarRefNode, LuaFlowId)>) -> Self {
        Self {
            var_ref_id,
            var_refs,
            assertions: HashMap::new(),
            current_flow_id: None,
            unresolve_traces: HashMap::new(),
        }
    }

    pub fn set_current_flow_id(&mut self, flow_id: LuaFlowId) {
        self.current_flow_id = Some(flow_id);
    }

    pub fn get_current_flow_id(&self) -> Option<LuaFlowId> {
        self.current_flow_id.clone()
    }

    pub fn get_var_ref_id(&self) -> &VarRefId {
        &self.var_ref_id
    }

    pub fn add_assert(&mut self, assertion: TypeAssertion, effect_range: TextRange) -> Option<()> {
        let current_flow_id = self.current_flow_id?;
        for var_ref in &self.var_refs {
            if effect_range.contains(var_ref.0.get_position()) {
                let var_use_ref = match &var_ref.0 {
                    VarRefNode::UseRef(var) => var,
                    _ => continue,
                };
                let var_ref_flow_id = var_ref.1;
                if current_flow_id == var_ref_flow_id {
                    self.assertions
                        .entry(var_use_ref.get_range())
                        .or_default()
                        .push(assertion);
                    return Some(());
                }
                // different closure, check is mutable after the flow
                let is_mutable_after_closure = self.var_refs.iter().any(|(node, flow_id)| {
                    flow_id == &current_flow_id
                        && node.is_assign_ref()
                        && (node.get_position() > var_ref_flow_id.get_range().end())
                });

                if !is_mutable_after_closure {
                    self.assertions
                        .entry(var_use_ref.get_range())
                        .or_default()
                        .push(assertion);
                    return Some(());
                }
            }
        }

        Some(())
    }

    pub fn add_unresolve_trace(&mut self, trace_id: UnResolveTraceId, assertion: TypeAssertion) {
        self.unresolve_traces.insert(trace_id, assertion);
    }

    pub fn check_var_use_in_range(&self, range: TextRange) -> bool {
        for (node, _) in &self.var_refs {
            if node.is_use_ref() && range.contains(node.get_position()) {
                return true;
            }
        }

        false
    }

    pub fn pop_unresolve_trace(&mut self, trace_id: &UnResolveTraceId) -> Option<TypeAssertion> {
        self.unresolve_traces.remove(trace_id)
    }
}
