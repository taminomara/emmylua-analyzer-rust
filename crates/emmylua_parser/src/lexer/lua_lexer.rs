use crate::{kind::LuaTokenKind, parser_error::LuaParseError, text::Reader};

use super::{is_name_continue, is_name_start, lexer_config::LexerConfig, token_data::LuaTokenData};

pub struct LuaLexer<'a> {
    reader: Reader<'a>,
    lexer_config: LexerConfig,
    errors: &'a mut Vec<LuaParseError>,
}

impl LuaLexer<'_> {
    pub fn new<'a>(
        text: &'a str,
        lexer_config: LexerConfig,
        errors: &'a mut Vec<LuaParseError>,
    ) -> LuaLexer<'a> {
        LuaLexer {
            reader: Reader::new(text),
            lexer_config,
            errors,
        }
    }

    pub fn tokenize(&mut self) -> Vec<LuaTokenData> {
        let mut tokens = vec![];

        while !self.reader.is_eof() {
            let kind = self.lex();
            if kind == LuaTokenKind::TkEof {
                break;
            }

            tokens.push(LuaTokenData::new(kind, self.reader.saved_range()));
        }

        tokens
    }

    fn name_to_kind(&self, name: &str) -> LuaTokenKind {
        match name {
            "and" => LuaTokenKind::TkAnd,
            "break" => LuaTokenKind::TkBreak,
            "do" => LuaTokenKind::TkDo,
            "else" => LuaTokenKind::TkElse,
            "elseif" => LuaTokenKind::TkElseIf,
            "end" => LuaTokenKind::TkEnd,
            "false" => LuaTokenKind::TkFalse,
            "for" => LuaTokenKind::TkFor,
            "function" => LuaTokenKind::TkFunction,
            "goto" => {
                if self.lexer_config.support_goto() {
                    LuaTokenKind::TkGoto
                } else {
                    LuaTokenKind::TkName
                }
            }
            "if" => LuaTokenKind::TkIf,
            "in" => LuaTokenKind::TkIn,
            "local" => LuaTokenKind::TkLocal,
            "nil" => LuaTokenKind::TkNil,
            "not" => LuaTokenKind::TkNot,
            "or" => LuaTokenKind::TkOr,
            "repeat" => LuaTokenKind::TkRepeat,
            "return" => LuaTokenKind::TkReturn,
            "then" => LuaTokenKind::TkThen,
            "true" => LuaTokenKind::TkTrue,
            "until" => LuaTokenKind::TkUntil,
            "while" => LuaTokenKind::TkWhile,
            _ => LuaTokenKind::TkName,
        }
    }

    fn lex(&mut self) -> LuaTokenKind {
        self.reader.reset_buff();

        match self.reader.current_char() {
            '\n' | '\r' => self.lex_new_line(),
            ' ' | '\t' => self.lex_white_space(),
            '-' => {
                self.reader.bump();
                if self.reader.current_char() != '-' {
                    return LuaTokenKind::TkMinus;
                }

                self.reader.bump();
                if self.reader.current_char() == '[' {
                    self.reader.bump();
                    let sep = self.skip_sep();
                    if self.reader.current_char() == '[' {
                        self.reader.bump();
                        self.lex_long_string(sep);
                        return LuaTokenKind::TkLongComment;
                    }
                }

                self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                LuaTokenKind::TkShortComment
            }
            '[' => {
                self.reader.bump();
                let sep = self.skip_sep();
                if sep == 0 && self.reader.current_char() != '[' {
                    return LuaTokenKind::TkLeftBracket;
                }
                if self.reader.current_char() != '[' {
                    self.errors.push(LuaParseError::from_source_range(
                        "invalid long string delimiter",
                        self.reader.saved_range(),
                    ));
                    return LuaTokenKind::TkLongString;
                }

                self.reader.bump();
                self.lex_long_string(sep)
            }
            '=' => {
                self.reader.bump();
                if self.reader.current_char() != '=' {
                    return LuaTokenKind::TkAssign;
                }
                self.reader.bump();
                LuaTokenKind::TkEq
            }
            '<' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        LuaTokenKind::TkLe
                    }
                    '<' => {
                        if !self.lexer_config.support_integer_operation() {
                            self.errors.push(LuaParseError::from_source_range(
                                "bitwise operation is not supported",
                                self.reader.saved_range(),
                            ));
                        }

                        self.reader.bump();
                        LuaTokenKind::TkShl
                    }
                    _ => LuaTokenKind::TkLt,
                }
            }
            '>' => {
                self.reader.bump();
                match self.reader.current_char() {
                    '=' => {
                        self.reader.bump();
                        LuaTokenKind::TkGe
                    }
                    '>' => {
                        if !self.lexer_config.support_integer_operation() {
                            self.errors.push(LuaParseError::from_source_range(
                                "bitwise operation is not supported",
                                self.reader.saved_range(),
                            ));
                        }

                        self.reader.bump();
                        LuaTokenKind::TkShr
                    }
                    _ => LuaTokenKind::TkGt,
                }
            }
            '~' => {
                if !self.lexer_config.support_integer_operation() {
                    self.errors.push(LuaParseError::from_source_range(
                        "bitwise operation is not supported",
                        self.reader.saved_range(),
                    ));
                }

                self.reader.bump();
                if self.reader.current_char() != '=' {
                    return LuaTokenKind::TkBitXor;
                }
                self.reader.bump();
                LuaTokenKind::TkNe
            }
            ':' => {
                self.reader.bump();
                if self.reader.current_char() != ':' {
                    return LuaTokenKind::TkColon;
                }
                self.reader.bump();
                LuaTokenKind::TkDbColon
            }
            '"' | '\'' => {
                let quote = self.reader.current_char();
                self.reader.bump();
                while !self.reader.is_eof() {
                    let ch = self.reader.current_char();
                    if ch == quote || ch == '\n' || ch == '\r' {
                        break;
                    }

                    if ch != '\\' {
                        self.reader.bump();
                        continue;
                    }

                    self.reader.bump();
                    match self.reader.current_char() {
                        'z' => {
                            self.reader.bump();
                            self.reader
                                .eat_while(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n');
                        }
                        '\r' | '\n' => {
                            self.lex_new_line();
                        }
                        _ => {
                            self.reader.bump();
                        }
                    }
                }

                if self.reader.current_char() != quote {
                    self.errors.push(LuaParseError::from_source_range(
                        "unfinished string",
                        self.reader.saved_range(),
                    ));
                    return LuaTokenKind::TkString;
                }

                self.reader.bump();
                LuaTokenKind::TkString
            }
            '.' => {
                if self.reader.next_char().is_digit(10) {
                    return self.lex_number();
                }

                self.reader.bump();
                if self.reader.current_char() != '.' {
                    return LuaTokenKind::TkDot;
                }
                self.reader.bump();
                if self.reader.current_char() != '.' {
                    return LuaTokenKind::TkConcat;
                }
                self.reader.bump();
                LuaTokenKind::TkDots
            }
            '0'..='9' => self.lex_number(),
            '/' => {
                self.reader.bump();
                if self.reader.current_char() != '/' {
                    return LuaTokenKind::TkDiv;
                }
                if !self.lexer_config.support_integer_operation() {
                    self.errors.push(LuaParseError::from_source_range(
                        "integer division is not supported",
                        self.reader.saved_range(),
                    ));
                }

                self.reader.bump();
                LuaTokenKind::TkIDiv
            }
            '*' => {
                self.reader.bump();
                LuaTokenKind::TkMul
            }
            '+' => {
                self.reader.bump();
                LuaTokenKind::TkPlus
            }
            '%' => {
                self.reader.bump();
                LuaTokenKind::TkMod
            }
            '^' => {
                if !self.lexer_config.support_integer_operation() {
                    self.errors.push(LuaParseError::from_source_range(
                        "integer power operation is not supported",
                        self.reader.saved_range(),
                    ));
                }

                self.reader.bump();
                LuaTokenKind::TkPow
            }
            '#' => {
                self.reader.bump();
                if self.reader.current_char() != '!' {
                    return LuaTokenKind::TkLen;
                }
                self.reader.eat_while(|ch| ch != '\n' && ch != '\r');
                LuaTokenKind::TkShebang
            }
            '&' => {
                if !self.lexer_config.support_integer_operation() {
                    self.errors.push(LuaParseError::from_source_range(
                        "bitwise operation is not supported",
                        self.reader.saved_range(),
                    ));
                }

                self.reader.bump();
                LuaTokenKind::TkBitAnd
            }
            '|' => {
                if !self.lexer_config.support_integer_operation() {
                    self.errors.push(LuaParseError::from_source_range(
                        "bitwise operation is not supported",
                        self.reader.saved_range(),
                    ));
                }

                self.reader.bump();
                LuaTokenKind::TkBitOr
            }
            '(' => {
                self.reader.bump();
                LuaTokenKind::TkLeftParen
            }
            ')' => {
                self.reader.bump();
                LuaTokenKind::TkRightParen
            }
            '{' => {
                self.reader.bump();
                LuaTokenKind::TkLeftBrace
            }
            '}' => {
                self.reader.bump();
                LuaTokenKind::TkRightBrace
            }
            ']' => {
                self.reader.bump();
                LuaTokenKind::TkRightBracket
            }
            ';' => {
                self.reader.bump();
                LuaTokenKind::TkSemicolon
            }
            ',' => {
                self.reader.bump();
                LuaTokenKind::TkComma
            }
            '@' => {
                self.reader.bump();
                LuaTokenKind::TkAt
            }
            _ if self.reader.is_eof() => LuaTokenKind::TkEof,
            ch if is_name_start(ch) => {
                self.reader.bump();
                self.reader.eat_while(|c| is_name_continue(c));
                let name = self.reader.current_saved_text();
                self.name_to_kind(name)
            }
            _ => {
                self.reader.bump();
                LuaTokenKind::TkUnknown
            }
        }
    }

    fn lex_new_line(&mut self) -> LuaTokenKind {
        let mut count = 0;
        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            if ch == '\n' || ch == '\r' {
                count += 1;
                self.reader.bump();
                if ch == '\r' && self.reader.current_char() == '\n' {
                    self.reader.bump();
                }
            } else {
                break;
            }
        }

        LuaTokenKind::TkEndOfLine
    }

    fn lex_white_space(&mut self) -> LuaTokenKind {
        self.reader.eat_while(|ch| ch == ' ' || ch == '\t');
        LuaTokenKind::TkWhitespace
    }

    fn skip_sep(&mut self) -> usize {
        let mut count = 0;
        while self.reader.current_char() == '=' {
            count += 1;
            self.reader.bump();
        }
        count
    }

    fn lex_long_string(&mut self, sep: usize) -> LuaTokenKind {
        let mut end = false;
        while !self.reader.is_eof() {
            match self.reader.current_char() {
                ']' => {
                    self.reader.bump();
                    let mut count = 0;
                    while self.reader.current_char() == '=' {
                        count += 1;
                        self.reader.bump();
                    }
                    if count == sep {
                        end = true;
                        break;
                    }
                }
                '\n' | '\r' => {
                    self.lex_new_line();
                }
                _ => {
                    self.reader.bump();
                }
            }
        }

        if !end {
            self.errors.push(LuaParseError::from_source_range(
                "unfinished long string or comment",
                self.reader.saved_range(),
            ));
        }

        LuaTokenKind::TkLongString
    }

    fn lex_number(&mut self) -> LuaTokenKind {
        enum NumberState {
            Int,
            Float,
            Hex,
            HexFloat,
            WithExpo,
        }

        let mut state = NumberState::Int;
        let first = self.reader.current_char();
        self.reader.bump();
        match first {
            '0' if matches!(self.reader.current_char(), 'X' | 'x') => {
                self.reader.bump();
                state = NumberState::Hex;
            }
            '.' => {
                state = NumberState::Float;
            }
            _ => {}
        }

        while !self.reader.is_eof() {
            let ch = self.reader.current_char();
            let continue_ = match state {
                NumberState::Int => match ch {
                    '0'..='9' => true,
                    '.' => {
                        state = NumberState::Float;
                        true
                    }
                    _ if matches!(self.reader.current_char(), 'e' | 'E') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::Float => match ch {
                    '0'..='9' => true,
                    _ if matches!(self.reader.current_char(), 'e' | 'E') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::Hex => match ch {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
                    '.' => {
                        state = NumberState::HexFloat;
                        true
                    }
                    _ if matches!(self.reader.current_char(), 'P' | 'p') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::HexFloat => match ch {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
                    _ if matches!(self.reader.current_char(), 'P' | 'p') => {
                        if matches!(self.reader.next_char(), '+' | '-') {
                            self.reader.bump();
                        }
                        state = NumberState::WithExpo;
                        true
                    }
                    _ => false,
                },
                NumberState::WithExpo => matches!(ch, '0'..='9'),
            };

            if !continue_ {
                break;
            }
        }

        if self.lexer_config.support_complex_number() {
            if self.reader.current_char() == 'i' {
                self.reader.bump();
                return LuaTokenKind::TkComplex;
            }
        }

        if self.lexer_config.support_ll_integer() {
            if matches!(state, NumberState::Int | NumberState::Hex) {
                self.reader
                    .eat_while(|ch| matches!(ch, 'u' | 'U' | 'l' | 'L'));
                return LuaTokenKind::TkInt;
            }
        }

        LuaTokenKind::TkFloat
    }
}
