mod flow_chain;

use std::collections::HashMap;

use flow_chain::LuaFlowChain;

use crate::FileId;

use super::traits::LuaIndex;


#[derive(Debug)]
pub struct LuaFlowIndex {
    chains_map: HashMap<FileId, LuaFlowChain>,
}

impl LuaFlowIndex {
    pub fn new() -> Self {
        Self {
            chains_map: HashMap::new(),
        }
    }
}

impl LuaIndex for LuaFlowIndex {
    fn remove(&mut self, file_id: crate::FileId) {
        todo!()
    }
}