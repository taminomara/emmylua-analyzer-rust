use rowan::TextSize;

use crate::{parser_error::LuaParseError, text::LineIndex, LuaSyntaxNode, LuaSyntaxNodePtr};

pub struct LuaSyntaxTree {
    root: LuaSyntaxNode,
    source_text: String,
    line_index: LineIndex,
    errors: Vec<LuaParseError>,
}

impl LuaSyntaxTree {
    pub fn new(root: LuaSyntaxNode, text: String, errors: Vec<LuaParseError>) -> Self {
        let line_index = LineIndex::parse(&text);
        LuaSyntaxTree {
            root,
            source_text: text,
            line_index,
            errors,
        }
    }

    pub fn get_red_root(&self) -> &LuaSyntaxNode {
        &self.root
    }

    // get line base 0
    pub fn get_line(&self, offset: TextSize) -> Option<usize> {
        self.line_index.get_line(offset)
    }

    // get col base 0
    pub fn get_col(&self, offset: TextSize) -> Option<usize> {
        let (line, start_offset) = self.line_index.get_line_with_start_offset(offset)?;
        if self.line_index.is_line_only_ascii(line.try_into().unwrap()) {
            Some(usize::from(offset - start_offset))
        } else {
            let text = &self.source_text[usize::from(start_offset)..usize::from(offset)];
            Some(text.chars().count())
        }
    }

    // get line and col base 0
    pub fn get_line_col(&self, offset: TextSize) -> Option<(usize, usize)> {
        let (line, start_offset) = self.line_index.get_line_with_start_offset(offset)?;
        if self.line_index.is_line_only_ascii(line.try_into().unwrap()) {
            Some((line, usize::from(offset - start_offset)))
        } else {
            let text = &self.source_text[usize::from(start_offset)..usize::from(offset)];
            Some((line, text.chars().count()))
        }
    }

    // get source text
    pub fn get_source_text(&self) -> &str {
        &self.source_text
    }

    // get line count
    pub fn get_line_count(&self) -> usize {
        self.line_index.line_count()
    }

    // get offset by line and col
    pub fn get_offset(&self, line: usize, col: usize) -> Option<TextSize> {
        let start_offset = self.line_index.get_line_offset(line)?;
        if col == 0 {
            return Some(start_offset);
        }

        if self.line_index.is_line_only_ascii(line.try_into().unwrap()) {
            let col = col.min(self.source_text.len());
            Some(start_offset + TextSize::from(col as u32))
        } else {
            let mut offset = 0;
            let mut col = col;
            for c in self.source_text[usize::from(start_offset)..].chars() {
                if col == 0 {
                    break;
                }

                offset += c.len_utf8();
                col -= 1;
            }
            Some(start_offset + TextSize::from(offset as u32))
        }
    }

    pub fn get_errors(&self) -> &[LuaParseError] {
        &self.errors
    }
}
