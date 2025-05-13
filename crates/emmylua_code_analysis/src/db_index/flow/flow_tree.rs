// use std::collections::{HashMap, HashSet};

// use super::{LuaVarRefId, LuaVarRefNode};

// #[derive(Debug)]
// pub struct LuaFlowTree {
//     previous: HashMap<LuaVarRefId, HashMap<LuaVarRefNode, LuaVarRefNode>>,
//     children: HashMap<LuaVarRefId, HashMap<LuaVarRefNode, Vec<LuaVarRefNode>>>,
//     var_ref_set: HashMap<LuaVarRefId, HashSet<LuaVarRefNode>>,
// }

// #[allow(unused)]
// impl LuaFlowTree {
//     pub fn new() -> Self {
//         Self {
//             previous: HashMap::new(),
//             children: HashMap::new(),
//             var_ref_set: HashMap::new(),
//         }
//     }

//     pub fn is_ref_node(&self, ref_id: &LuaVarRefId, node: &LuaVarRefNode) -> bool {
//         if let Some(ref_set) = self.var_ref_set.get(ref_id) {
//             ref_set.contains(node)
//         } else {
//             false
//         }
//     }

//     pub fn add_previous(&mut self, var_ref_id: LuaVarRefId, child: LuaVarRefNode, previous: LuaVarRefNode) {
//         self.previous.entry(var_ref_id.clone()).or_insert_with(HashMap::new).insert(child, previous);

//         if previous.get_range().contains(child.get_position()) {
//             self.children
//                 .entry(var_ref_id)
//                 .or_insert_with(HashMap::new)
//                 .entry(previous)
//                 .or_insert_with(Vec::new);
//         }
//     }

//     pub fn get_children(&self, var_ref_id: &LuaVarRefId, parent: &LuaVarRefNode) -> Option<Vec<LuaVarRefNode>> {
//         self.children.get(var_ref_id).and_then(|map| map.get(parent)).cloned()
//     }

//     pub fn get_path(&self, var_ref_id: &LuaVarRefId, child: &LuaVarRefNode) -> Option<Vec<LuaVarRefNode>> {
//         let mut path = Vec::new();
//         let mut current_node = child.clone();
//         let previous_map = self.previous.get(var_ref_id)?;
//         while let Some(previous_node) = previous_map.get(&current_node) {
//             path.push(previous_node.clone());
//             current_node = previous_node.clone();
//         }

//         path.reverse();
//         Some(path)
//     }
// }
