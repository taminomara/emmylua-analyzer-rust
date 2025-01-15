mod flow_chain;

use std::collections::HashMap;

pub use flow_chain::LuaFlowChain;

use crate::FileId;

use super::{traits::LuaIndex, LuaDeclId};


#[derive(Debug)]
pub struct LuaFlowIndex {
    chains_map: HashMap<FileId, HashMap<LuaDeclId, LuaFlowChain>>,
}

impl LuaFlowIndex {
    pub fn new() -> Self {
        Self {
            chains_map: HashMap::new(),
        }
    }

    pub fn add_flow_chain(&mut self, file_id: FileId, chain: LuaFlowChain) {
        let id = chain.get_decl_id();
        self.chains_map
            .entry(file_id)
            .or_insert_with(HashMap::new)
            .insert(id, chain);
    }

    pub fn get_flow_chain(&self, file_id: FileId, decl_id: LuaDeclId) -> Option<&LuaFlowChain> {
        self.chains_map
            .get(&file_id)
            .and_then(|map| map.get(&decl_id))
    }

    pub fn get_or_create_flow_chain(&mut self, file_id: FileId, decl_id: LuaDeclId) -> &mut LuaFlowChain {
        self.chains_map
            .entry(file_id)
            .or_insert_with(HashMap::new)
            .entry(decl_id)
            .or_insert_with(|| LuaFlowChain::new(decl_id))
    }
}

impl LuaIndex for LuaFlowIndex {
    fn remove(&mut self, file_id: crate::FileId) {
        self.chains_map.remove(&file_id);
    }

    fn fill_snapshot_info(&self, info: &mut HashMap<String, String>) {
        info.insert("flow_chain_count".to_string(), self.chains_map.len().to_string());
    }
}