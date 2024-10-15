use crate::{
    grammar::parse_chunk, kind::{LuaSyntaxKind, LuaTokenKind}, lexer::{LuaLexer, LuaTokenData}, parser_error::LuaParseError, text::SourceRange, LuaSyntaxTree
};

use super::{
    marker::{MarkEvent, Marker, MarkerEventContainer},
    parser_config::ParserConfig,
};

#[allow(unused)]
pub struct LuaParser<'a> {
    text: &'a str,
    events: Vec<MarkEvent>,
    tokens: Vec<LuaTokenData>,
    token_index: usize,
    current_token: LuaTokenKind,
    mark_level: usize,
    pub parse_config: ParserConfig,
    pub(crate) errors: &'a mut Vec<LuaParseError>,
}

impl MarkerEventContainer for LuaParser<'_> {
    fn get_mark_level(&self) -> usize {
        self.mark_level
    }

    fn incr_mark_level(&mut self) {
        self.mark_level += 1;
    }

    fn decr_mark_level(&mut self) {
        self.mark_level -= 1;
    }

    fn get_events(&mut self) -> &mut Vec<MarkEvent> {
        &mut self.events
    }
}

impl<'a> LuaParser<'a> {
    #[allow(unused)]
    fn parse(text: &'a str, config: ParserConfig) -> LuaSyntaxTree {
        let mut errors: Vec<LuaParseError> = Vec::new();
        let tokens = {
            let mut lexer = LuaLexer::new(text, config.lexer_config(), &mut errors);
            lexer.tokenize()
        };

        let mut parser = LuaParser {
            text,
            events: Vec::new(),
            tokens,
            token_index: 0,
            current_token: LuaTokenKind::None,
            parse_config: config,
            mark_level: 0,
            errors: &mut errors,
        };
        parser.init();

        parse_chunk(&mut parser);

        LuaSyntaxTree {}
    }

    fn init(&mut self) {
        if self.tokens.is_empty() {
            self.current_token = LuaTokenKind::None;
        } else {
            self.current_token = self.tokens[0].kind;
        }

        if is_trivia_kind(self.current_token) {
            self.bump();
        }
    }

    pub fn current_token(&self) -> LuaTokenKind {
        self.current_token
    }

    pub fn current_token_index(&self) -> usize {
        self.token_index
    }

    pub fn current_token_range(&self) -> SourceRange {
        self.tokens[self.token_index].range
    }

    pub fn current_token_text(&self) -> &str {
        let range = &self.tokens[self.token_index].range;
        &self.text[range.start_offset..range.end_offset()]
    }

    pub fn bump(&mut self) {
        let mut next_index = self.token_index + 1;
        self.skip_trivia(&mut next_index);
        if next_index < self.tokens.len() {
            self.parse_trivia_tokens(next_index);
            self.token_index = next_index;
        } else {
            self.token_index = self.tokens.len();
        }

        if self.token_index >= self.tokens.len() {
            self.current_token = LuaTokenKind::None;
            return;
        }
        self.current_token = self.tokens[self.token_index].kind;
    }

    pub fn next_token(&self) -> LuaTokenKind {
        let mut next_index = self.token_index + 1;
        self.skip_trivia(&mut next_index);

        if next_index >= self.tokens.len() {
            LuaTokenKind::None
        } else {
            self.tokens[next_index].kind
        }
    }

    fn skip_trivia(&self, index: &mut usize) {
        if index >= &mut self.tokens.len() {
            return;
        }

        let mut kind = self.tokens[*index].kind;
        while is_trivia_kind(kind) {
            *index += 1;
            if *index >= self.tokens.len() {
                break;
            }
            kind = self.tokens[*index].kind;
        }
    }

    fn parse_trivia_tokens(&mut self, next_index: usize) {
        let start = self.token_index;
    }

    fn parse_comments(&mut self, comment_tokens: Vec<LuaTokenData>) {

    }

}

fn is_trivia_kind(kind: LuaTokenKind) -> bool {
    matches!(
        kind,
        LuaTokenKind::TkShortComment
            | LuaTokenKind::TkLongComment
            | LuaTokenKind::TkEndOfLine
            | LuaTokenKind::TkWhitespace
            | LuaTokenKind::TkShebang
    )
}