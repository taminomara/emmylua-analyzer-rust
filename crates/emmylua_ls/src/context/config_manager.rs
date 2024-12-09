use std::{path::PathBuf, sync::Arc, time::Duration};

use super::{ClientProxy, VsCodeStatusBar};
use crate::handlers::{init_analysis, ClientConfig};
use code_analysis::{load_configs, EmmyLuaAnalysis, Emmyrc};
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
        tokio::spawn(async move {
            select! {
                _ = tokio::time::sleep(Duration::from_secs(2)) => {
                    let emmyrc = load_emmy_config(Some(file_dir.clone()), client_config);
                    init_analysis(analysis, client, &status_bar, workspace_folders, emmyrc).await;
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

    pub fn update_editorconfig(&self, path: PathBuf) {}
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
        pre_process_emmyrc(&mut emmyrc, workspace_root);
    }

    emmyrc.into()
}

fn merge_client_config(client_config: ClientConfig, emmyrc: &mut Emmyrc) -> Option<()> {
    if let Some(runtime) = &mut emmyrc.runtime {
        if runtime.extensions.is_none() {
            runtime.extensions = Some(client_config.extensions);
        }
    }

    if let Some(workspace) = &mut emmyrc.workspace {
        if workspace.ignore_globs.is_none() {
            workspace.ignore_globs = Some(client_config.exclude);
        } else if let Some(ignore_globs) = &mut workspace.ignore_globs {
            ignore_globs.extend(client_config.exclude);
        }

        if workspace.encoding.is_none() {
            workspace.encoding = Some(client_config.encoding);
        }
    }

    Some(())
}

fn pre_process_emmyrc(emmyrc: &mut Emmyrc, workspace_root: &PathBuf) {
    if let Some(workspace) = &mut emmyrc.workspace {
        if let Some(workspace_roots) = &mut workspace.workspace_roots {
            let new_workspace_roots = workspace_roots
                .iter()
                .map(|root| pre_process_path(root, workspace_root))
                .collect::<Vec<String>>();
            *workspace_roots = new_workspace_roots;
        }
        if let Some(ignore_dir) = &mut workspace.ignore_dir {
            let new_ignore_dir = ignore_dir
                .iter()
                .map(|dir| pre_process_path(dir, workspace_root))
                .collect::<Vec<String>>();
            *ignore_dir = new_ignore_dir;
        }
    }
    if let Some(resource) = &mut emmyrc.resource {
        if let Some(paths) = &mut resource.paths {
            let new_paths = paths
                .iter()
                .map(|path| pre_process_path(path, workspace_root))
                .collect::<Vec<String>>();
            *paths = new_paths;
        }
    }
}

fn pre_process_path(path: &str, workspace: &PathBuf) -> String {
    let mut path = path.to_string();

    if path.starts_with('~') {
        let home_dir = dirs::home_dir().unwrap();
        path = home_dir.join(&path[1..]).to_string_lossy().to_string();
    } else if path.starts_with("./") {
        path = workspace.join(&path[2..]).to_string_lossy().to_string();
    } else if path.starts_with('/') {
        path = workspace
            .join(path.trim_start_matches('/'))
            .to_string_lossy()
            .to_string();
    }

    path = path.replace("${workspaceFolder}", &workspace.to_string_lossy());
    path
}
