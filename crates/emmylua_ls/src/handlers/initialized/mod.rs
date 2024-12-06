mod client_config;
mod collect_files;
mod regsiter_file_watch;

use std::{path::PathBuf, str::FromStr};

use crate::{
    context::{load_emmy_config, ClientProxy, FileDiagnostic, ServerContextSnapshot},
    logger::init_logger,
};
use client_config::get_client_config;
pub use client_config::ClientConfig;
use code_analysis::{uri_to_file_path, EmmyLuaAnalysis, Emmyrc};
use collect_files::collect_files;
use log::info;
use lsp_types::{ClientInfo, InitializeParams};
use regsiter_file_watch::register_files_watch;

pub async fn initialized_handler(
    context: ServerContextSnapshot,
    params: InitializeParams,
) -> Option<()> {
    let mut analysis = context.analysis.write().await;
    let client_id = get_client_id(&params.client_info);
    let client_config = get_client_config(&context, client_id).await;
    let workspace_folders = get_workspace_folders(&params);
    let main_root: Option<&str> = match workspace_folders.first() {
        Some(path) => path.to_str(),
        None => None,
    };

    // init logger
    init_logger(main_root);
    info!("client_id: {:?}", client_id);
    let params_json = serde_json::to_string_pretty(&params).unwrap();
    info!("initialization_params: {}", params_json);

    // init config
    // todo! support multi config
    let config_root: Option<PathBuf> = match main_root {
        Some(root) => Some(PathBuf::from(root)),
        None => None,
    };

    let emmyrc = load_emmy_config(config_root, client_config.clone());
    // update config
    analysis.update_config(emmyrc.clone());

    let emmyrc_json = serde_json::to_string_pretty(emmyrc.as_ref()).unwrap();
    info!("current config : {}", emmyrc_json);

    let mut config_manager = context.config_manager.lock().await;
    config_manager.workspace_folders = workspace_folders.clone();
    config_manager.client_config = client_config.clone();
    drop(config_manager);

    init_analysis(
        &mut analysis,
        &context.client,
        &context.file_diagnostic,
        workspace_folders,
        &emmyrc,
    )
    .await;

    register_files_watch(context.client.clone(), &params.capabilities);
    Some(())
}

#[allow(unused)]
pub async fn init_analysis(
    analysis: &mut EmmyLuaAnalysis,
    client_proxy: &ClientProxy,
    file_diagnostic: &FileDiagnostic,
    workspace_folders: Vec<PathBuf>,
    emmyrc: &Emmyrc,
    // todo add cancel token
) {
    let mut workspace_folders = workspace_folders;
    for workspace_root in &workspace_folders {
        info!("add workspace root: {:?}", workspace_root);
        analysis.add_workspace_root(workspace_root.clone());
    }

    if let Some(workspace) = &emmyrc.workspace {
        if let Some(workspace_roots) = &workspace.workspace_roots {
            for workspace_root in workspace_roots {
                info!("add workspace root: {:?}", workspace_root);
                analysis.add_workspace_root(PathBuf::from_str(workspace_root).unwrap());
            }
        }

        if let Some(library) = &workspace.library {
            for lib in library {
                info!("add library: {:?}", lib);
                analysis.add_workspace_root(PathBuf::from_str(lib).unwrap());
                workspace_folders.push(PathBuf::from_str(lib).unwrap());
            }
        }
    }

    // load files
    let files = collect_files(&workspace_folders, &emmyrc);
    let files = files.into_iter().map(|file| file.into_tuple()).collect();
    let file_ids = analysis.update_files_by_path(files);

    // add tdiagnostic
    file_diagnostic.add_files_diagnostic_task(file_ids).await;
}

fn get_workspace_folders(params: &InitializeParams) -> Vec<PathBuf> {
    let mut workspace_folders = Vec::new();
    if let Some(workspaces) = &params.workspace_folders {
        for workspace in workspaces {
            if let Some(path) = uri_to_file_path(&workspace.uri) {
                workspace_folders.push(path);
            }
        }
    }

    if workspace_folders.is_empty() {
        // However, most LSP clients still provide this field
        #[allow(deprecated)]
        if let Some(uri) = &params.root_uri {
            let root_workspace = uri_to_file_path(&uri);
            if let Some(path) = root_workspace {
                workspace_folders.push(path);
            }
        }
    }

    workspace_folders
}

#[derive(Debug, Clone, Copy)]
pub enum ClientId {
    VSCode,
    Intellij,
    Neovim,
    Other,
}

impl Default for ClientId {
    fn default() -> Self {
        ClientId::Other
    }
}

#[allow(unused)]
impl ClientId {
    pub fn is_vscode(&self) -> bool {
        matches!(self, ClientId::VSCode)
    }

    pub fn is_intellij(&self) -> bool {
        matches!(self, ClientId::Intellij)
    }

    pub fn is_neovim(&self) -> bool {
        matches!(self, ClientId::Neovim)
    }

    pub fn is_other(&self) -> bool {
        matches!(self, ClientId::Other)
    }
}

fn get_client_id(client_info: &Option<ClientInfo>) -> ClientId {
    match client_info {
        Some(info) => {
            if info.name == "Visual Studio Code" {
                ClientId::VSCode
            } else if info.name == "IntelliJ" {
                ClientId::Intellij
            } else if info.name == "Neovim" {
                ClientId::Neovim
            } else {
                ClientId::Other
            }
        }
        None => ClientId::Other,
    }
}
