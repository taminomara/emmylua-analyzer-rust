use std::io::Error;

use stat::parse_stats;

use crate::{
    kind::{LuaSyntaxKind, LuaTokenKind},
    parser::{LuaParser, MarkerEventContainer},
    parser_error::LuaParseError,
};

use super::ParseResult;

mod expr;
mod stat;

pub fn parse_chunk(p: &mut LuaParser) {
    let m = p.mark(LuaSyntaxKind::Block);

    while p.current_token() != LuaTokenKind::TkEof {
        parse_stats(p);
    }

    m.complete(p);
}

fn parse_block(p: &mut LuaParser) -> ParseResult {
    let m = p.mark(LuaSyntaxKind::Block);

    parse_stats(p);

    Ok(m.complete(p))
}

fn expect_token(p: &mut LuaParser, token: LuaTokenKind) -> Result<(), LuaParseError> {
    if p.current_token() == token {
        Ok(())
    } else {
        Err(LuaParseError::from_source_range(
            &format!("expected {:?}, but get {:?}", token, p.current_token()),
            p.current_token_range(),
        ))
    }
}

fn if_token_bump(p: &mut LuaParser, token: LuaTokenKind) -> bool {
    if p.current_token() == token {
        p.bump();
        true
    } else {
        false
    }
}