use crate::{kind::LuaTokenKind, syntax::traits::LuaAstToken, LuaSyntaxToken};

use super::string_token_value;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaGeneralToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaGeneralToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(_: LuaTokenKind) -> bool
    where
        Self: Sized {
        true
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized {
        Some(LuaGeneralToken { token: syntax })
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaNameToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaNameToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized {
        kind == LuaTokenKind::TkName.into()
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized {
        if Self::can_cast(syntax.kind().into()) {
            Some(LuaNameToken { token: syntax })
        } else {
            None
        }
    }
}

impl LuaNameToken {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.token.text()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaStringToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaStringToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized {
        kind == LuaTokenKind::TkString.into()
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized {
        if Self::can_cast(syntax.kind().into()) {
            Some(LuaStringToken { token: syntax })
        } else {
            None
        }
    }
}

impl LuaStringToken {
    #[allow(dead_code)]
    pub fn get_value(&self) -> String {
        match string_token_value(self.token.text(), self.token.kind().into()) {
            Ok(str) => str,
            Err(_) => String::new(),
        }
    }
}