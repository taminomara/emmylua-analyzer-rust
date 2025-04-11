use std::collections::HashMap;

use emmylua_parser::{LuaAstNode, LuaClosureExpr, LuaSyntaxKind, LuaSyntaxNode};
use rowan::{TextRange, TextSize};

use crate::db_index::TypeAssertion;

use super::VarRefId;

#[derive(Debug)]
pub struct LuaFlowChain {
    flow_id: LuaFlowId,
    type_asserts: HashMap<VarRefId, Vec<LuaFlowChainEntry>>,
}

#[derive(Debug)]
pub struct LuaFlowChainEntry {
    pub type_assert: TypeAssertion,
    pub block_range: TextRange,
    pub actual_range: TextRange,
}

impl LuaFlowChain {
    pub fn new(flow_id: LuaFlowId) -> Self {
        Self {
            flow_id,
            type_asserts: HashMap::new(),
        }
    }

    pub fn get_flow_id(&self) -> LuaFlowId {
        self.flow_id
    }

    pub fn add_type_assert(
        &mut self,
        var_ref_id: &VarRefId,
        type_assert: TypeAssertion,
        block_range: TextRange,
        actual_range: TextRange,
    ) {
        self.type_asserts
            .entry(var_ref_id.clone())
            .or_insert_with(Vec::new)
            .push(LuaFlowChainEntry {
                type_assert,
                block_range,
                actual_range,
            });
    }

    pub fn get_type_asserts(
        &self,
        var_ref_id: &VarRefId,
        position: TextSize,
        start_position: Option<TextSize>,
    ) -> impl Iterator<Item = &TypeAssertion> {
        self.type_asserts
            .get(var_ref_id)
            .into_iter()
            .flat_map(move |asserts| {
                asserts.iter().filter_map(move |entry| {
                    if !entry.block_range.contains(position)
                        || position < entry.actual_range.start()
                    {
                        return None;
                    }
                    // 变量可能被重定义, 需要抛弃之前的声明
                    if let Some(start_pos) = start_position {
                        if entry.actual_range.start() >= start_pos {
                            Some(&entry.type_assert)
                        } else {
                            None
                        }
                    } else {
                        Some(&entry.type_assert)
                    }
                })
            })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LuaFlowId(TextSize);

impl LuaFlowId {
    pub fn from_closure(closure_expr: LuaClosureExpr) -> Self {
        Self(closure_expr.get_position())
    }

    pub fn chunk() -> Self {
        Self(TextSize::from(0))
    }

    pub fn from_node(node: &LuaSyntaxNode) -> Self {
        let flow_id = node.ancestors().find_map(|node| match node.kind().into() {
            LuaSyntaxKind::ClosureExpr => LuaClosureExpr::cast(node).map(LuaFlowId::from_closure),
            _ => None,
        });

        flow_id.unwrap_or_else(|| LuaFlowId::chunk())
    }

    pub fn get_position(&self) -> TextSize {
        self.0
    }
}
