use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use code_analysis::EmmyLuaAnalysis;

use super::{client::ClientProxy, config_manager::ConfigManager, file_diagnostic::FileDiagnostic};

#[derive(Clone)]
pub struct ServerContextSnapshot {
    pub analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    pub client: Arc<ClientProxy>,
    pub file_diagnostic: Arc<FileDiagnostic>,
    pub config_manager: Arc<Mutex<ConfigManager>>,
}
