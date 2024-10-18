use crate::{
    grammar::parse_comment,
    kind::LuaTokenKind,
    lexer::{LuaDocLexer, LuaDocLexerState, LuaTokenData},
    parser_error::LuaParseError,
    text::SourceRange,
};

use super::{LuaParser, MarkEvent, MarkerEventContainer};

pub struct LuaDocParser<'a, 'b> {
    lua_parser: &'a mut LuaParser<'b>,
    tokens: &'a [LuaTokenData],
    pub lexer: LuaDocLexer<'a>,
    current_token: LuaTokenKind,
    current_token_range: SourceRange,
    origin_token_index: usize,
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
    pub fn parse<'a, 'b>(lua_parser: &'a mut LuaParser<'b>, tokens: &[LuaTokenData]) {
        let lexer = LuaDocLexer::new(lua_parser.origin_text());

        let mut parser = LuaDocParser {
            lua_parser,
            tokens,
            lexer,
            current_token: LuaTokenKind::None,
            current_token_range: SourceRange::EMPTY,
            origin_token_index: 0,
        };

        parser.init();

        parse_comment(&mut parser);
    }

    fn init(&mut self) {
        if self.tokens.is_empty() {
            return;
        }
        self.bump();
    }

    pub fn bump(&mut self) {
        if !is_invalid_kind(self.current_token) {
            self.lua_parser.get_events().push(MarkEvent::EatToken {
                kind: self.current_token,
                range: self.current_token_range,
            });
        }

        self.calc_next_current_token();
    }

    fn calc_next_current_token(&mut self) {
        let token = self.lex_token();
        self.current_token = token.kind;
        self.current_token_range = token.range;

        if self.current_token == LuaTokenKind::TkEof {
            return;
        }

        match self.lexer.state {
            LuaDocLexerState::Normal
            | LuaDocLexerState::Description
            | LuaDocLexerState::Version => {
                while matches!(
                    self.current_token,
                    LuaTokenKind::TkDocContinue
                        | LuaTokenKind::TkEndOfLine
                        | LuaTokenKind::TkWhitespace
                ) {
                    self.eat_current_and_lex_next();
                }
            }
            LuaDocLexerState::FieldStart | LuaDocLexerState::See => {
                while matches!(self.current_token, LuaTokenKind::TkWhitespace) {
                    self.eat_current_and_lex_next();
                }
            }
            _ => {}
        }
    }

    fn eat_current_and_lex_next(&mut self) {
        self.lua_parser.get_events().push(MarkEvent::EatToken {
            kind: self.current_token,
            range: self.current_token_range,
        });

        let token = self.lex_token();
        self.current_token = token.kind;
        self.current_token_range = token.range;
    }

    fn lex_token(&mut self) -> LuaTokenData {
        #[allow(unused_assignments)]
        let mut kind = LuaTokenKind::TkEof;
        loop {
            if self.lexer.is_invalid() {
                let next_origin_index =
                    if self.origin_token_index == 0 && self.current_token == LuaTokenKind::None {
                        0
                    } else {
                        self.origin_token_index + 1
                    };
                if next_origin_index >= self.tokens.len() {
                    return LuaTokenData::new(LuaTokenKind::TkEof, SourceRange::EMPTY);
                }

                let next_origin_token = self.tokens[next_origin_index];
                self.origin_token_index = next_origin_index;
                if next_origin_token.kind == LuaTokenKind::TkEndOfLine
                    || next_origin_token.kind == LuaTokenKind::TkWhitespace
                    || next_origin_token.kind == LuaTokenKind::TkShebang
                {
                    return next_origin_token;
                }

                self.lexer
                    .reset(next_origin_token.kind, next_origin_token.range);
            }

            kind = self.lexer.lex();
            if kind != LuaTokenKind::TkEof {
                break;
            }
        }

        LuaTokenData::new(kind, self.lexer.current_token_range())
    }

    pub fn current_token(&self) -> LuaTokenKind {
        self.current_token
    }

    pub fn current_token_range(&self) -> SourceRange {
        self.current_token_range
    }

    pub fn current_token_text(&self) -> &str {
        let source_text = self.lua_parser.origin_text();
        let range = self.current_token_range;
        &source_text[range.start_offset..range.end_offset()]
    }

    #[allow(unused)]
    pub fn peek_next_token(&mut self) -> LuaTokenKind {
        let current_origin_index = self.origin_token_index;
        let current_token = self.current_token;
        let current_token_range = self.current_token_range;
        let prev_lexer = self.lexer.clone();
        self.bump();
        let next_token = self.current_token;
        self.origin_token_index = current_origin_index;
        self.current_token = current_token;
        self.current_token_range = current_token_range;
        self.lexer = prev_lexer;

        next_token
    }

    pub fn set_state(&mut self, state: LuaDocLexerState) {
        if self.current_token == LuaTokenKind::TkName {
            match state {
                LuaDocLexerState::Description => {
                    if !matches!(
                        self.current_token,
                        LuaTokenKind::TkWhitespace
                            | LuaTokenKind::TkEndOfLine
                            | LuaTokenKind::TkEof
                            | LuaTokenKind::TkDocContinueOr
                    ) {
                        self.current_token = LuaTokenKind::TkDocDetail;
                    }
                }
                LuaDocLexerState::Trivia => {
                    if !matches!(
                        self.current_token,
                        LuaTokenKind::TkWhitespace
                            | LuaTokenKind::TkEndOfLine
                            | LuaTokenKind::TkEof
                            | LuaTokenKind::TkDocContinueOr
                    ) {
                        self.current_token = LuaTokenKind::TkDocTrivia;
                    }
                }
                _ => {}
            }
        }

        self.lexer.state = state;
    }

    pub fn bump_to_end(&mut self) {
        self.set_state(LuaDocLexerState::Trivia);
        self.eat_current_and_lex_next();
        self.set_state(LuaDocLexerState::Init);
        self.bump();
    }

    pub fn push_error(&mut self, error: LuaParseError) {
        self.lua_parser.errors.push(error);
    }
}

fn is_invalid_kind(kind: LuaTokenKind) -> bool {
    match kind {
        LuaTokenKind::None | LuaTokenKind::TkEof => true,
        _ => false,
    }
}
