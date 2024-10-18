mod tag;
mod types;

use tag::{parse_long_tag, parse_tag};
use types::parse_type;

use crate::{
    kind::{LuaSyntaxKind, LuaTokenKind},
    lexer::LuaDocLexerState,
    parser::{LuaDocParser, MarkerEventContainer}, parser_error::LuaParseError,
};

pub fn parse_comment(p: &mut LuaDocParser) {
    let m = p.mark(LuaSyntaxKind::Comment);

    parse_docs(p);

    m.complete(p);
}

fn parse_docs(p: &mut LuaDocParser) {
    while p.current_token() != LuaTokenKind::TkEof {
        match p.current_token() {
            LuaTokenKind::TkDocStart => {
                p.set_state(LuaDocLexerState::Tag);
                p.bump();
                parse_tag(p);
            }
            LuaTokenKind::TkDocLongStart => {
                p.set_state(LuaDocLexerState::Tag);
                p.bump();
                parse_long_tag(p);
            }
            LuaTokenKind::TkNormalStart | LuaTokenKind::TkLongCommentStart => {
                p.set_state(LuaDocLexerState::Description);
                p.bump();
                parse_description(p);
            }
            LuaTokenKind::TkDocContinueOr => {
                p.set_state(LuaDocLexerState::Normal);
                p.bump();
                parse_continue_or(p);
            }
            _ => {
                p.bump();
            }
        }

        if let Some(reader) = p.lexer.get_reader() {
            if !reader.is_eof()
                && p.current_token() != LuaTokenKind::TkDocStart
                && p.current_token() != LuaTokenKind::TkDocLongStart
            {
                p.bump_to_end();
                continue;
            }
        }

        p.set_state(LuaDocLexerState::Init);
    }
}

fn parse_description(p: &mut LuaDocParser) {
    let m = p.mark(LuaSyntaxKind::DocDescription);

    while p.current_token() == LuaTokenKind::TkDocDetail {
        p.bump();
    }

    m.complete(p);
}


fn expect_token(p: &mut LuaDocParser, token: LuaTokenKind) -> Result<(), LuaParseError> {
    if p.current_token() == token {
        p.bump();
        Ok(())
    } else {
        Err(LuaParseError::from_source_range(
            &format!("expected {:?}, but get {:?}", token, p.current_token()),
            p.current_token_range(),
        ))
    }
}

fn if_token_bump(p: &mut LuaDocParser, token: LuaTokenKind) -> bool {
    if p.current_token() == token {
        p.bump();
        true
    } else {
        false
    }
}

// ---| 1
// ---| "string"
// ---| string.CS.AAA # HELLO
fn parse_continue_or(p: &mut LuaDocParser) {
    let m = p.mark(LuaSyntaxKind::DocContinueOrField);
    
    match parse_type(p) {
        Ok(_) => {}
        Err(err) => {
            p.push_error(err);
        }
    }

    p.set_state(LuaDocLexerState::Description);
    parse_description(p);
    m.complete(p);
}