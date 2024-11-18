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
}

impl LuaIndex for LuaFlowIndex {
    fn remove(&mut self, file_id: crate::FileId) {
        self.chains_map.remove(&file_id);
    }
}