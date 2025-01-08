mod client_config;
mod codestyle;
mod collect_files;
mod locale;
mod regsiter_file_watch;

use std::{path::PathBuf, str::FromStr, sync::Arc};

use crate::{
    cmd_args::CmdArgs,
    context::{load_emmy_config, ClientProxy, ServerContextSnapshot, VsCodeStatusBar},
    logger::init_logger,
};
use client_config::get_client_config;
pub use client_config::ClientConfig;
use code_analysis::{uri_to_file_path, EmmyLuaAnalysis, Emmyrc, FileId};
use codestyle::load_editorconfig;
use collect_files::collect_files;
use log::info;
use lsp_types::{ClientInfo, InitializeParams};
use regsiter_file_watch::register_files_watch;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub async fn initialized_handler(
    context: ServerContextSnapshot,
    params: InitializeParams,
    cmd_args: CmdArgs,
) -> Option<()> {
    let client_id = get_client_id(&params.client_info);
    let client_config = get_client_config(&context, client_id).await;
    let workspace_folders = get_workspace_folders(&params);
    let main_root: Option<&str> = match workspace_folders.first() {
        Some(path) => path.to_str(),
        None => None,
    };
    // init locale
    locale::set_ls_locale(&params);

    // init logger
    init_logger(main_root, &cmd_args);
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
    load_editorconfig(workspace_folders.clone());

    let mut config_manager = context.config_manager.write().await;
    config_manager.workspace_folders = workspace_folders.clone();
    config_manager.client_config = client_config.clone();
    drop(config_manager);

    init_analysis(
        context.analysis.clone(),
        context.client.clone(),
        &context.status_bar,
        workspace_folders,
        emmyrc,
        client_id,
    )
    .await;

    register_files_watch(context, &params.capabilities).await;
    Some(())
}

#[allow(unused)]
pub async fn init_analysis(
    analysis: Arc<RwLock<EmmyLuaAnalysis>>,
    client_proxy: Arc<ClientProxy>,
    status_bar: &VsCodeStatusBar,
    workspace_folders: Vec<PathBuf>,
    emmyrc: Arc<Emmyrc>,
    client_id: ClientId,
    // todo add cancel token
) {
    let mut mut_analysis = analysis.write().await;

    // update config
    mut_analysis.update_config(emmyrc.clone());

    let emmyrc_json = serde_json::to_string_pretty(emmyrc.as_ref()).unwrap();
    info!("current config : {}", emmyrc_json);

    status_bar.set_server_status("ok", true, "Load workspace");
    status_bar.report_progress("Load workspace", 0.0);

    let mut workspace_folders = workspace_folders;
    for workspace_root in &workspace_folders {
        info!("add workspace root: {:?}", workspace_root);
        mut_analysis.add_workspace_root(workspace_root.clone());
    }

    for workspace_root in &emmyrc.workspace.workspace_roots {
        info!("add workspace root: {:?}", workspace_root);
        mut_analysis.add_workspace_root(PathBuf::from_str(workspace_root).unwrap());
    }

    for lib in &emmyrc.workspace.library {
        info!("add library: {:?}", lib);
        mut_analysis.add_workspace_root(PathBuf::from_str(lib).unwrap());
        workspace_folders.push(PathBuf::from_str(lib).unwrap());
    }

    status_bar.report_progress("Collect files", 0.1);
    // load files
    let files = collect_files(&workspace_folders, &emmyrc);
    let files: Vec<(PathBuf, Option<String>)> =
        files.into_iter().map(|file| file.into_tuple()).collect();

    let file_count = files.len();
    status_bar.report_progress(format!("Index {} files", file_count).as_str(), 0.5);
    let file_ids = mut_analysis.update_files_by_path(files);

    drop(mut_analysis);

    let cancle_token = CancellationToken::new();
    // diagnostic files
    let (tx, mut rx) = tokio::sync::mpsc::channel::<FileId>(100);
    for file_id in file_ids {
        let analysis = analysis.clone();
        let token = cancle_token.clone();
        let client = client_proxy.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let analysis = analysis.read().await;
            let diagnostics = analysis.diagnose_file(file_id, token).await;
            if let Some(diagnostics) = diagnostics {
                let uri = analysis.get_uri(file_id).unwrap();
                let diagnostic_param = lsp_types::PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: None,
                };
                client.publish_diagnostics(diagnostic_param);
            }
            let _ = tx.send(file_id).await;
        });
    }

    let mut count = 0;
    while let Some(_) = rx.recv().await {
        count += 1;

        if client_id.is_vscode() {
            status_bar.report_progress(
                format!("diagnostic {}/{}", count, file_count).as_str(),
                0.75,
            );
        }
        if count == file_count {
            break;
        }
    }

    status_bar.report_progress("Finished!", 1.0);
    status_bar.set_server_status("ok", false, "Finished!");
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
