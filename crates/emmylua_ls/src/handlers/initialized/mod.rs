mod client_config;
mod lua_finder;
mod config_loader;

use std::path::PathBuf;

use client_config::get_client_config;
use code_analysis::uri_to_file_path;
use lsp_types::{ClientInfo, InitializeParams};
use lua_finder::collect_files;

use crate::context::ServerContextSnapshot;

pub async fn initialized_handler(
    context: ServerContextSnapshot,
    params: InitializeParams,
) -> Option<()> {
    let analysis = context.analysis.write().await;
    let client_id = get_client_id(&params.client_info);
    let client_config = get_client_config(&context, client_id).await;
    let workspace_folders = get_workspace_folders(&params);
    let files = collect_files(&workspace_folders, &client_config);


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
