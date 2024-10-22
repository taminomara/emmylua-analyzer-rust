use crate::{kind::LuaTokenKind, parser_error::LuaParseError};

pub fn string_token_value(text: &str, kind: LuaTokenKind) -> Result<String, LuaParseError> {
    Ok("".into())
}