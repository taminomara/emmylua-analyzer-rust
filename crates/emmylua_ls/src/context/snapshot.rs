use std::sync::{Arc, RwLock};

use code_analysis::EmmyLuaAnalysis;


#[derive(Debug, Clone)]
pub struct ServerContextSnapshot {
    pub analysis: Arc<RwLock<EmmyLuaAnalysis>>,
}