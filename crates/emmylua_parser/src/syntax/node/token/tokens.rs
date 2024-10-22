use crate::{kind::{BinaryOperator, LuaTokenKind, UnaryOperator}, syntax::traits::LuaAstToken, LuaOpKind, LuaSyntaxToken};

use super::{float_token_value, int_token_value, string_token_value};

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
        Self: Sized,
    {
        true
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
    {
        kind == LuaTokenKind::TkName.into()
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
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
        Self: Sized,
    {
        kind == LuaTokenKind::TkString.into() || kind == LuaTokenKind::TkLongString.into()
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
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
        match string_token_value(&self.token) {
            Ok(str) => str,
            Err(_) => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaNumberToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaNumberToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaTokenKind::TkFloat.into() || kind == LuaTokenKind::TkInt.into()
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(LuaNumberToken { token: syntax })
        } else {
            None
        }
    }
}

impl LuaNumberToken {
    #[allow(dead_code)]
    pub fn is_float(&self) -> bool {
        self.token.kind() == LuaTokenKind::TkFloat.into()
    }

    #[allow(dead_code)]
    pub fn is_int(&self) -> bool {
        self.token.kind() == LuaTokenKind::TkInt.into()
    }

    #[allow(dead_code)]
    pub fn get_float_value(&self) -> f64 {
        if !self.is_float() {
            return 0.0;
        }
        match float_token_value(&self.token) {
            Ok(float) => float,
            Err(_) => 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn get_int_value(&self) -> i64 {
        if !self.is_int() {
            return 0;
        }
        match int_token_value(&self.token) {
            Ok(int) => int,
            Err(_) => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBinaryOpToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaBinaryOpToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized,
    {
        LuaOpKind::to_binary_operator(kind) != BinaryOperator::OpNop
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(LuaBinaryOpToken { token: syntax })
        } else {
            None
        }
    }
}

impl LuaBinaryOpToken {
    #[allow(dead_code)]
    pub fn get_op(&self) -> BinaryOperator {
        LuaOpKind::to_binary_operator(self.token.kind().into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaUnaryOpToken {
    token: LuaSyntaxToken,
}

impl LuaAstToken for LuaUnaryOpToken {
    fn syntax(&self) -> &LuaSyntaxToken {
        &self.token
    }

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized,
    {
        LuaOpKind::to_unary_operator(kind) != UnaryOperator::OpNop
    }

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(LuaUnaryOpToken { token: syntax })
        } else {
            None
        }
    }
}

impl LuaUnaryOpToken {
    #[allow(dead_code)]
    pub fn get_op(&self) -> UnaryOperator {
        LuaOpKind::to_unary_operator(self.token.kind().into())
    }
}

