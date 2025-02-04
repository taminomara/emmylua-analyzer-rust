use emmylua_code_analysis::update_code_style;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const VCS_DIRS: [&str; 3] = [".git", ".hg", ".svn"];

pub fn load_editorconfig(workspace_folders: Vec<PathBuf>) -> Option<()> {
    let mut editorconfig_files = Vec::new();

    for workspace in workspace_folders {
        // 构建 WalkDir 迭代器，递归遍历工作区目录
        let walker = WalkDir::new(&workspace)
            .into_iter()
            .filter_entry(|e| !is_vcs_dir(e, &VCS_DIRS));

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if is_editorconfig(&entry) {
                        editorconfig_files.push(entry.path().to_path_buf());
                    }
                }
                Err(e) => {
                    log::error!("Traversal error: {:?}", e);
                }
            }
        }
    }

    if editorconfig_files.is_empty() {
        return None;
    }

    log::info!("found editorconfig files: {:?}", editorconfig_files);
    for file in editorconfig_files {
        let parent_dir = file
            .parent()
            .unwrap()
            .to_path_buf()
            .to_string_lossy()
            .to_string()
            .replace("\\", "/");
        let file_normalized = file.to_string_lossy().to_string().replace("\\", "/");
        update_code_style(&parent_dir, &file_normalized);
    }

    Some(())
}

/// 判断目录条目是否为 `.editorconfig` 文件
fn is_editorconfig(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name().to_string_lossy() == ".editorconfig"
}

/// 判断目录条目是否属于需要忽略的版本控制系统目录
fn is_vcs_dir(entry: &DirEntry, vcs_dirs: &[&str]) -> bool {
    if entry.file_type().is_dir() {
        let name = entry.file_name().to_string_lossy();
        vcs_dirs.iter().any(|&vcs| vcs == name)
    } else {
        true
    }
}
