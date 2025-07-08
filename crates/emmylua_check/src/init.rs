use std::{path::PathBuf, str::FromStr, sync::Arc};

use emmylua_code_analysis::{
    load_configs, load_workspace_files, update_code_style, DbIndex, EmmyLuaAnalysis, Emmyrc, FileId, LuaFileInfo
};

fn root_from_configs(config_paths: &Vec<PathBuf>, fallback: &PathBuf) -> PathBuf {
    if config_paths.len() != 1 {
        fallback.clone()
    } else {
        let config_path = &config_paths[0];
        // Need to convert to canonical path to ensure parent() is not an empty
        // string in the case the path is a relative basename.
        match config_path.canonicalize() {
            Ok(path) => path.parent().unwrap().to_path_buf(),
            Err(err) => {
                log::error!(
                    "Failed to canonicalize config path: \"{:?}\": {}",
                    config_path,
                    err
                );
                fallback.clone()
            }
        }
    }
}

pub fn load_workspace(
    workspace_folder: PathBuf,
    config_paths: Option<Vec<PathBuf>>,
    ignore: Option<Vec<String>>,
) -> Option<EmmyLuaAnalysis> {
    let mut workspace_folders = vec![workspace_folder];
    let main_path = workspace_folders.first()?.clone();
    let (config_files, config_root): (Vec<PathBuf>, PathBuf) =
        if let Some(config_paths) = config_paths {
            (
                config_paths.clone(),
                root_from_configs(&config_paths, &main_path),
            )
        } else {
            (
                vec![
                    main_path.join(".luarc.json"),
                    main_path.join(".emmyrc.json"),
                ]
                .into_iter()
                .filter(|path| path.exists())
                .collect(),
                main_path.clone(),
            )
        };

    let mut emmyrc = load_configs(config_files, None);
    log::info!(
        "Pre processing configurations using root: \"{}\"",
        config_root.display()
    );
    emmyrc.pre_process_emmyrc(&config_root);

    for lib in &emmyrc.workspace.library {
        workspace_folders.push(PathBuf::from_str(lib).unwrap());
    }

    let mut analysis = EmmyLuaAnalysis::new();

    for path in &workspace_folders {
        analysis.add_main_workspace(path.clone());
    }

    for root in &emmyrc.workspace.workspace_roots {
        analysis.add_main_workspace(PathBuf::from_str(root).unwrap());
    }

    analysis.update_config(Arc::new(emmyrc));
    analysis.init_std_lib(None);

    let file_infos = collect_files(&workspace_folders, &analysis.emmyrc, ignore);
    let files = file_infos
        .into_iter()
        .filter_map(|file| {
            if file.path.ends_with(".editorconfig") {
                let file_path = PathBuf::from(file.path);
                let parent_dir = file_path
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .to_string_lossy()
                    .to_string()
                    .replace("\\", "/");
                let file_normalized = file_path.to_string_lossy().to_string().replace("\\", "/");
                update_code_style(&parent_dir, &file_normalized);
                None
            } else {
                Some(file.into_tuple())
            }
        })
        .collect();
    analysis.update_files_by_path(files);

    Some(analysis)
}

pub fn collect_files(
    workspaces: &Vec<PathBuf>,
    emmyrc: &Emmyrc,
    ignore: Option<Vec<String>>,
) -> Vec<LuaFileInfo> {
    let mut files = Vec::new();
    let (match_pattern, exclude, exclude_dir) = calculate_include_and_exclude(emmyrc, ignore);

    let encoding = &emmyrc.workspace.encoding;

    for workspace in workspaces {
        let loaded = load_workspace_files(
            workspace,
            &match_pattern,
            &exclude,
            &exclude_dir,
            Some(encoding),
        )
        .ok();
        if let Some(loaded) = loaded {
            files.extend(loaded);
        }
    }

    files
}

pub fn calculate_include_and_exclude(
    emmyrc: &Emmyrc,
    ignore: Option<Vec<String>>,
) -> (Vec<String>, Vec<String>, Vec<PathBuf>) {
    let mut include = vec!["**/*.lua".to_string(), "**/.editorconfig".to_string()];
    let mut exclude = Vec::new();
    let mut exclude_dirs = Vec::new();

    for extension in &emmyrc.runtime.extensions {
        if extension.starts_with(".") {
            log::info!("Adding extension: **/*{}", extension);
            include.push(format!("**/*{}", extension));
        } else if extension.starts_with("*.") {
            log::info!("Adding extension: **/{}", extension);
            include.push(format!("**/{}", extension));
        } else {
            log::info!("Adding extension: {}", extension);
            include.push(extension.clone());
        }
    }

    for ignore_glob in &emmyrc.workspace.ignore_globs {
        log::info!("Adding ignore glob: {}", ignore_glob);
        exclude.push(ignore_glob.clone());
    }

    if let Some(ignore) = ignore {
        log::info!("Adding ignores from \"--ignore\": {:?}", ignore);
        exclude.extend(ignore);
    }

    for dir in &emmyrc.workspace.ignore_dir {
        log::info!("Adding ignore dir: {}", dir);
        exclude_dirs.push(PathBuf::from(dir));
    }

    // remove duplicate
    include.sort();
    include.dedup();

    // remove duplicate
    exclude.sort();
    exclude.dedup();

    (include, exclude, exclude_dirs)
}

pub fn get_need_check_ids(db: &DbIndex, files: Vec<FileId>, workspace: &PathBuf) -> Vec<FileId> {
    let mut need_check_files = Vec::new();
    for file_id in files {
        let file_path = db.get_vfs().get_file_path(&file_id).unwrap();
        if file_path.starts_with(workspace) {
            need_check_files.push(file_id);
        }
    }

    need_check_files
}
