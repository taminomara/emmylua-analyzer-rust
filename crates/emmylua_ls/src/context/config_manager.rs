use std::{path::PathBuf, sync::Arc};

use code_analysis::{EmmyLuaAnalysis, Emmyrc};
use tokio::sync::RwLock;

use crate::handlers::ClientConfig;

use super::ClientProxy;

pub struct ConfigManager {
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client: Arc<ClientProxy>,
    client_config: Option<ClientConfig>,
}

impl ConfigManager {
    pub fn new(
        analysis: Arc<RwLock<EmmyLuaAnalysis>>,
        client: Arc<ClientProxy>,
    ) -> Self {
        Self {
            analysis,
            client,
            client_config: None,
        }
    }

    pub fn add_update_emmyrc_task(&self, file_dir: PathBuf) {

    }

    pub fn update_editorconfig(&self, path: PathBuf) {

    }
}
