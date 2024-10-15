use std::str::Chars;

use super::text_range::SourceRange;
pub const EOF: char = '\0';

pub struct Reader<'a> {
    text: &'a str,
    valid_range: SourceRange,
    chars: Chars<'a>,
    buffer_byte_pos: usize,
    buffer_byte_len: usize,
    current: char
}

impl<'a> Reader<'a> {
    pub fn new(text: &'a str) -> Self {
        Self::new_with_range(text, SourceRange::new(0, text.len()))
    }

    pub fn new_with_range(text: &'a str, range: SourceRange) -> Self {
        let text = text[range.start_offset..range.length].as_ref();
        Self {
            text,
            valid_range: range,
            chars: text.chars(),
            buffer_byte_pos: 0,
            buffer_byte_len: 0,
            current: EOF
        }
    }

    pub fn bump(&mut self) {
        if let Some(c) = self.chars.next() {
            self.current = c;
            self.buffer_byte_len += self.current.len_utf8();
        }
        else {
            self.current = EOF;
        }
    }

    pub fn reset_buff(&mut self) {
        self.buffer_byte_pos += self.buffer_byte_len;
        self.buffer_byte_len = 0;
        self.bump();
    }

    // pub fn reset(&mut self, range: SourceRange) {
    //     self.valid_range = range;
    //     self.text = &self.text[range.start_offset..range.length];
    //     self.chars = self.text.chars();
    //     self.buffer_byte_pos = 0;
    //     self.buffer_byte_len = 0;
    //     self.current = EOF;
    // }

    pub fn is_eof(&self) -> bool {
        self.current == EOF
    }

    pub fn current_char(&self) -> char {
        self.current
    }

    pub fn next_char(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    pub fn saved_range(&self) -> SourceRange {
        SourceRange::new(self.valid_range.start_offset + self.buffer_byte_pos, self.buffer_byte_len)
    }

    pub fn current_saved_text(&self) -> &str {
        &self.text[self.buffer_byte_pos..(self.buffer_byte_pos + self.buffer_byte_len)]
    }

    pub fn eat_when(&mut self, ch: char) -> usize {
        let mut count = 0;
        while !self.is_eof() && self.current_char() == ch {
            count += 1;
            self.bump();
        }
        count
    }

    pub fn eat_while<F>(&mut self, func: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut count = 0;
        while !self.is_eof() && func(self.current_char()) {
            count += 1;
            self.bump();
        }
        count
    }
}
