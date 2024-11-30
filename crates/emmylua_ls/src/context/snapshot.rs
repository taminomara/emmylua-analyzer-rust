use std::sync::Arc;
use tokio::sync::RwLock;

use code_analysis::EmmyLuaAnalysis;

use super::client::ClientProxy;

pub struct ServerContextSnapshot {
    pub analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    pub client: Arc<ClientProxy>,
}
