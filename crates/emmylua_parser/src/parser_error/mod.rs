use rowan::TextRange;

use crate::text::SourceRange;

#[derive(Debug, Clone, PartialEq)]
pub struct LuaParseError {
    pub message: String,
    pub range: TextRange,
}

impl LuaParseError {
    pub fn new(message: &str, range: TextRange) -> Self {
        LuaParseError {
            message: message.to_string(),
            range,
        }
    }

    pub fn from_source_range(message: &str, range: SourceRange) -> Self {
        LuaParseError {
            message: message.to_string(),
            range: range.into(),
        }
    }
}
