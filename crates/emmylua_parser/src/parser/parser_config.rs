use crate::{kind::LuaLanguageLevel, lexer::LexerConfig};

pub struct ParserConfig {
    pub level: LuaLanguageLevel,
    lexer_config: LexerConfig,
}

impl ParserConfig {
    pub fn new(level: LuaLanguageLevel) -> Self {
        Self {
            level,
            lexer_config: LexerConfig { language_level: level },
        }
    }

    pub fn lexer_config(&self) -> LexerConfig {
        self.lexer_config
    }

    pub fn support_local_attrib(&self) -> bool {
        self.level == LuaLanguageLevel::Lua54
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            level: LuaLanguageLevel::Lua54,
            lexer_config: LexerConfig { language_level: LuaLanguageLevel::Lua54 },
        }
    }
}