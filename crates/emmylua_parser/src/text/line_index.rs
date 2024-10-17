use rowan::TextSize;

// TODO: fix this implementation, it's not correct
pub struct LineIndex {
    line_offsets: Vec<TextSize>,
}

impl LineIndex {
    pub fn parse(text: &str) -> LineIndex {
        let mut line_offsets = Vec::new();
        let mut offset = 0;

        line_offsets.push(TextSize::from(offset as u32));

        for (i, c) in text.char_indices() {
            if c == '\n' {
                offset = i + 1; // 记录每行的字节偏移量
                line_offsets.push(TextSize::from(offset as u32));
            }
        }

        LineIndex { line_offsets }
    }

    pub fn get_offset(&self, line: TextSize, column: TextSize) -> Option<TextSize> {
        let line_index = usize::from(line);
        if line_index < self.line_offsets.len() {
            let line_offset = self.line_offsets[line_index];
            Some(line_offset + column)
        } else {
            None
        }
    }

    pub fn get_line(&self, offset: TextSize) -> Option<TextSize> {
        let offset_value = usize::from(offset);
        match self.line_offsets.binary_search(&TextSize::from(offset_value as u32)) {
            Ok(line) => Some(TextSize::from(line as u32)),
            Err(line) => {
                if line > 0 {
                    Some(TextSize::from((line - 1) as u32))
                } else {
                    None
                }
            }
        }
    }

    pub fn get_column(&self, offset: TextSize) -> Option<TextSize> {
        if let Some(line) = self.get_line(offset) {
            let line_index = usize::from(line);
            let line_offset = self.line_offsets[line_index];
            Some(offset - line_offset)
        } else {
            None
        }
    }
}