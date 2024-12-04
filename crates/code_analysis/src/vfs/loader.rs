use std::{error::Error, fs, path::{Path, PathBuf}};

use encoding_rs::{Encoding, UTF_8};
use globset::{Glob, GlobSetBuilder};
use log::error;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LuaFileInfo {
    pub path: String,
    pub content: String,
}

impl LuaFileInfo {
    pub fn into_tuple(self) -> (PathBuf, Option<String>) {
        (PathBuf::from(self.path), Some(self.content))
    }
}

#[allow(unused)]
pub fn load_workspace_files(
    root: &Path,
    match_pattern: &Vec<String>,
    ignore_pattern: &Vec<String>,
    encoding: Option<&str>,
) -> Result<Vec<LuaFileInfo>, Box<dyn Error>> {
    let encoding = encoding.unwrap_or("utf-8");
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
            if let Some(content) = read_file_with_encoding(path, encoding) {
                files.push(LuaFileInfo {
                    path: path.to_string_lossy().to_string(),
                    content,
                });
            }
        }
    }

    Ok(files)
}

pub fn read_file_with_encoding(path: &Path, encoding: &str) -> Option<String> {
    let content = fs::read(path).ok()?;
    let encoding = Encoding::for_label(encoding.as_bytes()).unwrap_or(UTF_8);

    let (content, _, has_error) = encoding.decode(&content);
    if has_error{
        error!("Error decoding file: {:?}", path);
        return None;
    }

    // Remove BOM
    let content_str = if encoding == UTF_8 && content.starts_with("\u{FEFF}") {
        &content[3..]
    } else {
        &content
    };

    Some(content_str.to_string())
}
