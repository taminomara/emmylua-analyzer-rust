use std::path::PathBuf;

use emmylua_parser::LineIndex;
use lsp_types::Uri;
use rowan::{TextRange, TextSize};

use super::{uri_to_file_path, FileId};

#[derive(Debug)]
pub struct LuaDocument<'a> {
    file_id: FileId,
    uri: &'a Uri,
    text: &'a str,
    line_index: &'a LineIndex,
}

impl<'a> LuaDocument<'a> {
    pub fn new(
        file_id: FileId,
        uri: &'a Uri,
        text: &'a str,
        line_index: &'a LineIndex,
    ) -> Self {
        LuaDocument {
            file_id,
            uri,
            text,
            line_index,
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_file_name(&self) -> Option<String> {
        uri_to_file_path(self.uri)?.file_name()?.to_str().map(|s| s.to_string())
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

    pub fn get_text_slice(&self, range: TextRange) -> &str {
        &self.text[range.start().into()..range.end().into()]
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

    pub fn to_lsp_range(&self, range: TextRange) -> Option<lsp_types::Range> {
        let start = self.get_line_col(range.start())?;
        let end = self.get_line_col(range.end())?;
        Some(lsp_types::Range {
            start: lsp_types::Position {
                line: start.0 as u32,
                character: start.1 as u32,
            },
            end: lsp_types::Position {
                line: end.0 as u32,
                character: end.1 as u32,
            },
        })
    }

    pub fn to_lsp_location(&self, range: TextRange) -> Option<lsp_types::Location> {
        Some(lsp_types::Location {
            uri: self.uri.clone(),
            range: self.to_lsp_range(range)?,
        })
    }

    pub fn to_rowan_range(&self, range: lsp_types::Range) -> Option<TextRange> {
        let start = self.get_offset(range.start.line as usize, range.start.character as usize)?;
        let end = self.get_offset(range.end.line as usize, range.end.character as usize)?;
        Some(TextRange::new(start, end))
    }

}
