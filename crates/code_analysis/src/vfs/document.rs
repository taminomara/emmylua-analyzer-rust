use std::{path::PathBuf, sync::Arc};

use emmylua_parser::{LineIndex, LuaSyntaxTree};
use lsp_types::Uri;
use rowan::TextSize;

use super::{uri_to_file_path, FileId};

#[derive(Debug)]
pub struct LuaDocument<'a> {
    file_id: FileId,
    uri: &'a Uri,
    text: &'a str,
    line_index: &'a LineIndex,
    tree: Arc<LuaSyntaxTree>
}

impl<'a> LuaDocument<'a> {
    pub fn new(file_id: FileId, uri: &'a Uri, text: &'a str, line_index: &'a LineIndex, tree: Arc<LuaSyntaxTree>) -> Self {
        LuaDocument {
            file_id,
            uri,
            text,
            line_index,
            tree
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_uri(&self) -> &Uri {
        self.uri
    }

    pub fn get_file_path(&self) -> Option<PathBuf> {
        uri_to_file_path(self.uri)
    }

    pub fn get_text(&self) -> &str {
        self.text
    }

    pub fn get_tree(&self) -> Arc<LuaSyntaxTree> {
        self.tree.clone()
    }

    pub fn get_line_count(&self) -> usize {
        self.line_index.line_count()
    }

    pub fn get_line(&self, offset: TextSize) -> Option<usize> {
        self.line_index.get_line(offset)
    }

    pub fn get_line_col(&self, offset: TextSize) -> Option<(usize, usize)> {
        self.line_index.get_line_col(offset, self.text)
    }

    pub fn get_col(&self, offset: TextSize) -> Option<usize> {
        self.line_index.get_col(offset, self.text)
    }

    pub fn get_offset(&self, line: usize, col: usize) -> Option<TextSize> {
        self.line_index.get_offset(line, col, self.text)
    }
}
