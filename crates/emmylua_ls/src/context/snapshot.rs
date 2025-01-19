use std::sync::Arc;
use tokio::sync::RwLock;

use code_analysis::EmmyLuaAnalysis;

use super::{
    client::ClientProxy, config_manager::ConfigManager, file_diagnostic::FileDiagnostic,
    status_bar::StatusBar,
};

#[derive(Clone)]
pub struct ServerContextSnapshot {
    pub analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    pub client: Arc<ClientProxy>,
    pub file_diagnostic: Arc<FileDiagnostic>,
    pub config_manager: Arc<RwLock<ConfigManager>>,
    pub status_bar: Arc<StatusBar>,
}
