mod client_config;
mod init_config;
mod collect_files;

use std::{path::PathBuf, str::FromStr};

use client_config::get_client_config;
use code_analysis::uri_to_file_path;
use init_config::init_config;
use log::info;
use lsp_types::{ClientInfo, InitializeParams};
use collect_files::collect_files;

use crate::{context::ServerContextSnapshot, logger::init_logger};

pub async fn initialized_handler(
    context: ServerContextSnapshot,
    params: InitializeParams,
) -> Option<()> {
    let mut analysis = context.analysis.write().await;
    let client_id = get_client_id(&params.client_info);
    let client_config = get_client_config(&context, client_id).await;
    let mut workspace_folders = get_workspace_folders(&params);
    for workspace_root in &workspace_folders {
        info!("add workspace root: {:?}", workspace_root);
        analysis.add_workspace_root(workspace_root.clone());
    }
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

    let emmyrc = init_config(config_root, client_config.clone());
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

    let emmyrc_json = serde_json::to_string_pretty(emmyrc.as_ref()).unwrap();
    info!("current config : {}", emmyrc_json);

    // load files
    let files = collect_files(&workspace_folders, &emmyrc);
    let files = files.into_iter().map(|file| file.into_tuple()).collect();
    analysis.update_files_by_path(files);

    Some(())
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
