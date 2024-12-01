use std::path::PathBuf;

use code_analysis::{load_workspace_files, LuaFileInfo};

use super::client_config::ClientConfig;

pub fn collect_files(workspaces: &Vec<PathBuf>, client_config: &ClientConfig) -> Vec<LuaFileInfo> {
    let mut files = Vec::new();
    let exclude = &client_config.exclude;
    let extensions = &client_config.extensions;
    let mut match_pattern = vec!["**/*.lua".to_string()];
    for extension in extensions {
        if extension.starts_with(".") {
            match_pattern.push(format!("**/*{}", extension));
        } else if extension.starts_with("*.") {
            match_pattern.push(format!("**/{}", extension));
        } else {
            match_pattern.push(extension.clone());
        }
    }

    let encoding = &client_config.encoding;
    eprintln!(
        "collect_files from: {:?} match_pattern: {:?} exclude: {:?}",
        workspaces, match_pattern, exclude
    );
    for workspace in workspaces {
        let loaded =
            load_workspace_files(&workspace, &match_pattern, &exclude, Some(&encoding)).ok();
        if let Some(loaded) = loaded {
            files.extend(loaded);
        }
    }

    eprintln!("load files from workspace count: {:?}", files.len());
    if cfg!(debug_assertions) {
        for file in &files {
            eprintln!("loaded file: {:?}", file.path);
        }
    }
    files
}
