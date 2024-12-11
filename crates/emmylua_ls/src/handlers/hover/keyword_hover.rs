use emmylua_parser::{LuaSyntaxToken, LuaTokenKind};

pub fn is_keyword(token: LuaSyntaxToken) -> bool {
    match token.kind().into() {
        LuaTokenKind::TkLocal => true,
        LuaTokenKind::TkFunction => true,
        LuaTokenKind::TkEnd => true,
        LuaTokenKind::TkIf => true,
        LuaTokenKind::TkThen => true,
        LuaTokenKind::TkElse => true,
        LuaTokenKind::TkElseIf => true,
        LuaTokenKind::TkWhile => true,
        LuaTokenKind::TkDo => true,
        LuaTokenKind::TkFor => true,
        LuaTokenKind::TkIn => true,
        LuaTokenKind::TkRepeat => true,
        LuaTokenKind::TkUntil => true,
        LuaTokenKind::TkReturn => true,
        LuaTokenKind::TkBreak => true,
        LuaTokenKind::TkGoto => true,
        _ => false
    }
}

// todo add usage
pub fn hover_keyword(token: LuaSyntaxToken) -> String {
    match token.kind().into() {
        LuaTokenKind::TkLocal => "local".to_string(),
        LuaTokenKind::TkFunction => "function".to_string(),
        LuaTokenKind::TkEnd => "end".to_string(),
        LuaTokenKind::TkIf => "if".to_string(),
        LuaTokenKind::TkThen => "then".to_string(),
        LuaTokenKind::TkElse => "else".to_string(),
        LuaTokenKind::TkElseIf => "elseif".to_string(),
        LuaTokenKind::TkWhile => "while".to_string(),
        LuaTokenKind::TkDo => "do".to_string(),
        LuaTokenKind::TkFor => "for".to_string(),
        LuaTokenKind::TkIn => "in".to_string(),
        LuaTokenKind::TkRepeat => "repeat".to_string(),
        LuaTokenKind::TkUntil => "until".to_string(),
        LuaTokenKind::TkReturn => "return".to_string(),
        LuaTokenKind::TkBreak => "break".to_string(),
        LuaTokenKind::TkGoto => "goto".to_string(),
        _ => "".to_string()
    }
}