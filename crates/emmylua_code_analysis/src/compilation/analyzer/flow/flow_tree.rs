use std::collections::HashMap;

use emmylua_parser::{
    LuaBreakStat, LuaCallExpr, LuaExpr, LuaGotoStat, LuaReturnStat,
};
use rowan::TextSize;
use smol_str::SmolStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FlowRefNode {
    pub path: SmolStr,
    pub node: LuaExpr,
}

#[allow(unused)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FlowNode {
    ThrowError(LuaCallExpr),
    Break(LuaBreakStat),
    Goto(LuaGotoStat),
    Return(LuaReturnStat),
    Assert(LuaCallExpr),
    UseRef(FlowRefNode),
    AssignRef(FlowRefNode),
}

#[derive(Debug)]
pub struct FlowTree {
    pub nodes: HashMap<TextSize, FlowNode>,
}

#[allow(unused)]
impl FlowTree {
    pub fn new() -> FlowTree {
        FlowTree {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, position: TextSize, kind: FlowNode) {
        self.nodes.insert(position, kind);
    }

    pub fn get_node(&self, position: TextSize) -> Option<&FlowNode> {
        self.nodes.get(&position)
    }

    pub fn get_nodes(&self) -> Vec<&FlowNode> {
        self.nodes.values().collect()
    }
}
