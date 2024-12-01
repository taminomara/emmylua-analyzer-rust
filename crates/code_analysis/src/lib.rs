mod compilation;
mod config;
mod db_index;
mod diagnostic;
mod semantic;
mod vfs;

use std::{
    path::PathBuf,
    sync::Arc,
};

#[allow(unused)]
pub use compilation::*;
pub use config::Emmyrc;
#[allow(unused)]
pub use diagnostic::*;
use lsp_types::Uri;
#[allow(unused)]
pub use vfs::*;

#[derive(Debug)]
pub struct EmmyLuaAnalysis {
    pub compilation: LuaCompilation,
}

impl EmmyLuaAnalysis {
    pub fn new() -> Self {
        Self {
            compilation: LuaCompilation::new(),
        }
    }

    pub fn update_file_by_uri(&mut self, uri: &Uri, text: Option<String>) {
        let is_removed = text.is_none();
        let file_id = self
            .compilation
            .get_db_mut()
            .get_vfs_mut()
            .set_file_content(uri, text);

        self.compilation.remove_index(vec![file_id]);
        if !is_removed {
            self.compilation.update_index(vec![file_id]);
        }
    }

    pub fn update_file_by_path(&mut self, path: &PathBuf, text: Option<String>) -> Option<()> {
        let uri = file_path_to_uri(&path)?;
        self.update_file_by_uri(&uri, text);
        Some(())
    }

    pub fn update_files_by_uri(&mut self, files: Vec<(Uri, Option<String>)>) {
        let mut removed_files = Vec::new();
        let mut updated_files = Vec::new();
        for (uri, text) in files {
            let is_new_text = text.is_some();
            let file_id = self
                .compilation
                .get_db_mut()
                .get_vfs_mut()
                .set_file_content(&uri, text);
            removed_files.push(file_id);
            if is_new_text {
                updated_files.push(file_id);
            }
        }

        self.compilation.remove_index(removed_files);
        self.compilation.update_index(updated_files);
    }

    pub fn update_files_by_path(&mut self, files: Vec<(PathBuf, Option<String>)>) {
        let files = files
            .into_iter()
            .filter_map(|(path, text)| {
                let uri = file_path_to_uri(&path)?;
                Some((uri, text))
            })
            .collect();
        self.update_files_by_uri(files);
    }

    pub fn update_config(&mut self, config: Arc<Emmyrc>) {
        self.compilation.update_config(config);
    }
}

unsafe impl Send for EmmyLuaAnalysis {}
unsafe impl Sync for EmmyLuaAnalysis {}
