use std::{path::PathBuf, sync::Arc, time::Duration};

use super::{ClientProxy, VsCodeStatusBar};
use crate::handlers::{init_analysis, ClientConfig};
use code_analysis::{load_configs, EmmyLuaAnalysis, Emmyrc};
use emmylua_codestyle::update_code_style;
use log::{debug, info};
use tokio::{
    select,
    sync::{Mutex, RwLock},
};
use tokio_util::sync::CancellationToken;

pub struct ConfigManager {
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client: Arc<ClientProxy>,
    status_bar: Arc<VsCodeStatusBar>,
    pub client_config: ClientConfig,
    pub workspace_folders: Vec<PathBuf>,
    config_update_token: Arc<Mutex<Option<CancellationToken>>>,
    pub watcher: Option<notify::RecommendedWatcher>,
}

impl ConfigManager {
    pub fn new(
        analysis: Arc<RwLock<EmmyLuaAnalysis>>,
        client: Arc<ClientProxy>,
        status_bar: Arc<VsCodeStatusBar>,
    ) -> Self {
        Self {
            analysis,
            client,
            status_bar,
            client_config: ClientConfig::default(),
            workspace_folders: Vec::new(),
            config_update_token: Arc::new(Mutex::new(None)),
            watcher: None,
        }
    }

    pub async fn add_update_emmyrc_task(&self, file_dir: PathBuf) {
        let mut config_update_tokens = self.config_update_token.lock().await;
        if let Some(token) = config_update_tokens.as_ref() {
            token.cancel();
            debug!("cancel update config: {:?}", file_dir);
        }

        let cancel_token = CancellationToken::new();
        config_update_tokens.replace(cancel_token.clone());
        drop(config_update_tokens);

        let analysis = self.analysis.clone();
        let client = self.client.clone();
        let workspace_folders = self.workspace_folders.clone();
        let config_update_token = self.config_update_token.clone();
        let client_config = self.client_config.clone();
        let status_bar = self.status_bar.clone();
        let client_id = client_config.client_id;
        tokio::spawn(async move {
            select! {
                _ = tokio::time::sleep(Duration::from_secs(2)) => {
                    let emmyrc = load_emmy_config(Some(file_dir.clone()), client_config);
                    init_analysis(analysis, client, &status_bar, workspace_folders, emmyrc, client_id).await;
                    // After completion, remove from HashMap
                    let mut tokens = config_update_token.lock().await;
                    tokens.take();
                }
                _ = cancel_token.cancelled() => {
                    debug!("cancel diagnostic: {:?}", file_dir);
                }
            }
        });
    }

    pub fn update_editorconfig(&self, path: PathBuf) {
        let parent_dir = path
            .parent()
            .unwrap()
            .to_path_buf()
            .to_string_lossy()
            .to_string()
            .replace("\\", "/");
        let file_normalized = path.to_string_lossy().to_string().replace("\\", "/");
        log::info!("update code style: {:?}", file_normalized);
        update_code_style(&parent_dir, &file_normalized);
    }
}

pub fn load_emmy_config(config_root: Option<PathBuf>, client_config: ClientConfig) -> Arc<Emmyrc> {
    let mut config_files = Vec::new();
    if let Some(config_root) = &config_root {
        let luarc_path = config_root.join(".luarc.json");
        if luarc_path.exists() {
            info!("load config from: {:?}", luarc_path);
            config_files.push(luarc_path);
        }
        let emmyrc_path = config_root.join(".emmyrc.json");
        if emmyrc_path.exists() {
            info!("load config from: {:?}", emmyrc_path);
            config_files.push(emmyrc_path);
        }
    }

    let mut emmyrc = load_configs(config_files);
    merge_client_config(client_config, &mut emmyrc);
    if let Some(workspace_root) = &config_root {
        emmyrc.pre_process_emmyrc(workspace_root);
    }

    emmyrc.into()
}

fn merge_client_config(client_config: ClientConfig, emmyrc: &mut Emmyrc) -> Option<()> {
    emmyrc.runtime.extensions.extend(client_config.extensions);
    emmyrc.workspace.ignore_globs.extend(client_config.exclude);
    if client_config.encoding != "utf-8" {
        emmyrc.workspace.encoding = client_config.encoding;
    }

    Some(())
}
