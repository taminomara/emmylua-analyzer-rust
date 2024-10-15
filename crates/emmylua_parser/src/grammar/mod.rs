use crate::{parser::CompleteMarker, parser_error::LuaParseError};

mod lua;
mod doc;

type ParseResult = Result<CompleteMarker, LuaParseError>;
pub use lua::parse_chunk;

