mod token_data;
mod lua_lexer;
mod lexer_config;

pub use token_data::LuaTokenData;
pub use lexer_config::LexerConfig;
pub use lua_lexer::LuaLexer;

fn is_name_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_name_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}