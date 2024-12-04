use std::{path::PathBuf, sync::Arc};

use super::client_config::ClientConfig;
use code_analysis::Emmyrc;
use config::{Config, File};
use log::info;

pub fn init_config(config_root: Option<PathBuf>, client_config: ClientConfig) -> Arc<Emmyrc> {
    let mut config_builder = Config::builder();

    if let Some(config_root) = &config_root {
        let luarc_path = config_root.join(".luarc.json");
        if luarc_path.exists() {
            info!("load config from: {:?}", luarc_path);
            config_builder =
                config_builder.add_source(File::with_name(luarc_path.to_str().unwrap()));
        }
        let emmyrc_path = config_root.join(".emmyrc.json");
        if emmyrc_path.exists() {
            info!("load config from: {:?}", emmyrc_path);
            config_builder =
                config_builder.add_source(File::with_name(emmyrc_path.to_str().unwrap()));
        }
    }

    let config = config_builder.build().ok().unwrap();
    let mut emmyrc: Emmyrc = config.try_deserialize().ok().unwrap();
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
    std::fs::canonicalize(path)
        .unwrap()
        .to_string_lossy()
        .to_string()
}
