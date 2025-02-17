use std::path::PathBuf;

use emmylua_parser::LineIndex;
use lsp_types::Uri;
use rowan::{TextRange, TextSize};

use super::{file_path_to_uri, FileId};

#[derive(Debug)]
pub struct LuaDocument<'a> {
    file_id: FileId,
    path: &'a PathBuf,
    text: &'a str,
    line_index: &'a LineIndex,
}

impl<'a> LuaDocument<'a> {
    pub fn new(
        file_id: FileId,
        path: &'a PathBuf,
        text: &'a str,
        line_index: &'a LineIndex,
    ) -> Self {
        LuaDocument {
            file_id,
            path,
            text,
            line_index,
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_file_name(&self) -> Option<String> {
        self.path.file_name()?.to_str().map(|s| s.to_string())
    }

    pub fn get_uri(&self) -> Uri {
        file_path_to_uri(self.path).unwrap()
    }

    pub fn get_file_path(&self) -> &PathBuf {
        self.path
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
            uri: self.get_uri(),
            range: self.to_lsp_range(range)?,
        })
    }

    pub fn to_lsp_position(&self, offset: TextSize) -> Option<lsp_types::Position> {
        let line_col = self.get_line_col(offset)?;
        Some(lsp_types::Position {
            line: line_col.0 as u32,
            character: line_col.1 as u32,
        })
    }

    pub fn to_rowan_range(&self, range: lsp_types::Range) -> Option<TextRange> {
        let start = self.get_offset(range.start.line as usize, range.start.character as usize)?;
        let end = self.get_offset(range.end.line as usize, range.end.character as usize)?;
        Some(TextRange::new(start, end))
    }

    pub fn get_document_lsp_range(&self) -> lsp_types::Range {
        lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: self.get_line_count() as u32,
                character: 0,
            },
        }
    }

    pub fn get_range_span(&self, range: lsp_types::Range) -> Option<(usize, usize)> {
        let start = self.get_offset(range.start.line as usize, range.start.character as usize)?;
        let end = self.get_offset(range.end.line as usize, range.end.character as usize)?;
        Some((start.into(), end.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use lsp_types::{Position, Range};

    use crate::{Emmyrc, Vfs};

    use super::*;

    fn create_vfs() -> Vfs {
        let mut vfs = Vfs::new();
        vfs.update_config(Emmyrc::default().into());
        vfs
    }

    static TEST_URI: &str = "file:///C:/Users/username/Documents/test.lua";

    #[test]
    fn test_basic() {
        let code = r#"
        local a = 1
        print(a)
        "#;
        let mut vfs = create_vfs();
        let uri = Uri::from_str(TEST_URI).unwrap();
        let id = vfs.set_file_content(&uri, Some(code.to_string()));
        let document = vfs.get_document(&id).unwrap();

        assert_eq!(document.get_file_id(), id);
        assert_eq!(document.get_file_name(), Some("test.lua".to_string()));
        assert_eq!(document.get_uri(), uri);
        assert_eq!(
            *document.get_file_path(),
            PathBuf::from("C:/Users/username/Documents/test.lua")
        );
        assert!(document.get_line_count() > 0, "Document should have lines");
    }

    #[test]
    fn test_get_text_methods() {
        // Define a simple Lua code snippet without extra whitespace.
        let code = "local a = 1\nprint(a)";
        let mut vfs = create_vfs();
        let uri = Uri::from_str(TEST_URI).unwrap();
        let id = vfs.set_file_content(&uri, Some(code.to_string()));
        let document = vfs.get_document(&id).unwrap();

        // Test get_text returns the full document text.
        assert_eq!(document.get_text(), code);

        // Test get_text_slice: extract "local" (first 5 characters).
        let start = TextSize::from(0);
        let end = TextSize::from(5);
        let slice = document.get_text_slice(TextRange::new(start, end));
        assert_eq!(slice, "local");
    }

    #[test]
    fn test_lsp_conversion_methods() {
        let code = "local a = 1\nprint(a)";
        let mut vfs = create_vfs();
        let uri = Uri::from_str(TEST_URI).unwrap();
        let id = vfs.set_file_content(&uri, Some(code.to_string()));
        let document = vfs.get_document(&id).unwrap();

        // Test conversion of an offset to an LSP position.
        let lsp_position = document.to_lsp_position(TextSize::from(0)).unwrap();
        assert_eq!(lsp_position.line, 0);
        assert_eq!(lsp_position.character, 0);

        // Test conversion of a text range (first 5 characters) to an LSP range.
        let text_range = TextRange::new(TextSize::from(0), TextSize::from(5));
        let lsp_range = document.to_lsp_range(text_range).unwrap();
        assert_eq!(
            lsp_range.start,
            Position {
                line: 0,
                character: 0
            }
        );
        // Assuming "local" occupies 5 characters on line 0.
        assert_eq!(
            lsp_range.end,
            Position {
                line: 0,
                character: 5
            }
        );

        // Test conversion to an LSP location.
        let lsp_location = document.to_lsp_location(text_range).unwrap();
        assert_eq!(lsp_location.uri, uri);
        assert_eq!(lsp_location.range.start.line, 0);
    }

    #[test]
    fn test_to_rowan_range_and_range_span() {
        let code = "local a = 1\nprint(a)";
        let mut vfs = create_vfs();
        let uri = Uri::from_str(TEST_URI).unwrap();
        let id = vfs.set_file_content(&uri, Some(code.to_string()));
        let document = vfs.get_document(&id).unwrap();

        // Create an LSP range for the word "local" in the first line.
        let lsp_range = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 5,
            },
        };

        // Test conversion from LSP range to rowan TextRange.
        let text_range = document.to_rowan_range(lsp_range).unwrap();
        assert_eq!(text_range.start(), TextSize::from(0));
        assert_eq!(text_range.end(), TextSize::from(5));

        // Test getting the range span as (start_offset, end_offset).
        let span = document.get_range_span(lsp_range).unwrap();
        assert_eq!(span.0, 0);
        assert_eq!(span.1, 5);
    }

    #[test]
    fn test_get_document_lsp_range() {
        let code = "local a = 1\nprint(a)";
        let mut vfs = create_vfs();
        let uri = Uri::from_str(TEST_URI).unwrap();
        let id = vfs.set_file_content(&uri, Some(code.to_string()));
        let document = vfs.get_document(&id).unwrap();

        let doc_range = document.get_document_lsp_range();
        // The start of the document range should be at line 0, character 0.
        assert_eq!(
            doc_range.start,
            Position {
                line: 0,
                character: 0
            }
        );
        // The end line should equal the total number of lines in the document.
        assert_eq!(doc_range.end.line, document.get_line_count() as u32);
        assert_eq!(doc_range.end.character, 0);
    }
}
