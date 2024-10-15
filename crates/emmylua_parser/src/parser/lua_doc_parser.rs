use crate::{grammar::parse_doc, kind::LuaTokenKind, lexer::{LuaDocLexer, LuaTokenData}};

use super::{LuaParser, MarkEvent, MarkerEventContainer};

pub struct LuaDocParser<'a, 'b> {
    lua_parser: &'a mut LuaParser<'b>,
    tokens: &'a [LuaTokenData],
    pub lexer: LuaDocLexer<'a>,
}

impl MarkerEventContainer for LuaDocParser<'_, '_> {
    fn get_mark_level(&self) -> usize {
        self.lua_parser.get_mark_level()
    }

    fn incr_mark_level(&mut self) {
        self.lua_parser.incr_mark_level()
    }

    fn decr_mark_level(&mut self) {
        self.lua_parser.decr_mark_level()
    }

    fn get_events(&mut self) -> &mut Vec<MarkEvent> {
        self.lua_parser.get_events()
    }
}

impl LuaDocParser<'_, '_> {
    pub fn parse<'a, 'b>(
        lua_parser: &'a mut LuaParser<'b>,
        tokens: &[LuaTokenData],
    ) {
        let lexer = LuaDocLexer::new(lua_parser.origin_text());
    
        let mut parser = LuaDocParser {
            lua_parser,
            tokens,
            lexer
        };

        parser.init();

        parse_doc(&mut parser);
    }

    fn init(&mut self) {

    }

    pub fn bump(&mut self) {
        self.lua_parser.bump();
    }

    pub fn current_token(&self) -> LuaTokenKind {
        LuaTokenKind::None
    }
}
