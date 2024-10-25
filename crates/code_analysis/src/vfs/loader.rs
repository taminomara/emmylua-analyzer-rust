use std::{error::Error, fs, path::Path};

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LuaFileInfo {
    pub path: String,
    pub content: String,
}

pub fn load_workspace_files(
    root: &Path,
    match_pattern: Vec<String>,
    ignore_pattern: Vec<String>,
) -> Result<Vec<LuaFileInfo>, Box<dyn Error>> {
    let mut files = Vec::new();

    let mut match_builder = GlobSetBuilder::new();
    for pattern in match_pattern {
        match_builder.add(Glob::new(&pattern)?);
    }
    let match_set = match_builder.build()?;

    let mut ignore_builder = GlobSetBuilder::new();
    for pattern in ignore_pattern {

        ignore_builder.add(Glob::new(&pattern)?);
    }
    let ignore_set = ignore_builder.build()?;

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();
        if ignore_set.is_match(relative_path) {
            continue;
        }

        if match_set.is_match(relative_path) {
            if let Ok(content) = fs::read_to_string(path) {
                files.push(LuaFileInfo {
                    path: path.to_string_lossy().to_string(),
                    content,
                });
            }
        }
    }

    Ok(files)
}
