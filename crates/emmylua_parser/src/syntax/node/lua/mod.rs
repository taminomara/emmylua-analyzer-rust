mod expr;
mod stat;

use crate::{
    kind::{LuaSyntaxKind, LuaTokenKind},
    syntax::traits::{LuaAstChildren, LuaAstNode, LuaAstToken},
    LuaSyntaxNode,
};

pub use expr::*;
pub use stat::*;

use super::LuaNameToken;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaChunk {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaChunk {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Chunk
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::Chunk.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaChunk {
    pub fn get_block(&self) -> Option<LuaBlock> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBlock {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaBlock {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Block
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::Block.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaBlock {
    pub fn get_stats(&self) -> LuaAstChildren<LuaStat> {
        self.children()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalName {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalName {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LocalName
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaLocalName {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn get_attrib(&self) -> Option<LuaLocalAttribute> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaCallArgList {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaCallArgList {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::CallArgList
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaCallArgList {
    pub fn is_single_arg_no_parens(&self) -> bool {
        self.token_by_kind(LuaTokenKind::TkLeftParen).is_none()
    }

    pub fn get_args(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
    }

    pub fn get_single_arg_expr(&self) -> Option<LuaSingleArgExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLocalAttribute {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLocalAttribute {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Attribute
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaLocalAttribute {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn is_close(&self) -> bool {
        match self.get_name_token() {
            None => false,
            Some(name_token) => name_token.name() == "close",
        }
    }

    pub fn is_const(&self) -> bool {
        match self.get_name_token() {
            None => false,
            Some(name_token) => name_token.name() == "const",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaTableField {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaTableField {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::TableFieldAssign.into()
            || kind == LuaSyntaxKind::TableFieldValue.into()
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaTableField {
    pub fn is_assign_field(&self) -> bool {
        self.syntax().kind() == LuaSyntaxKind::TableFieldAssign.into()
    }

    pub fn is_value_field(&self) -> bool {
        self.syntax().kind() == LuaSyntaxKind::TableFieldValue.into()
    }

    pub fn get_key(&self) -> Option<LuaTableFieldKey> {
        if !self.is_assign_field() {
            return None;
        }

        for child in self.syntax().children_with_tokens() {
            match child {
                rowan::NodeOrToken::Node(node) => {
                    if let Some(expr) = LuaExpr::cast(node.clone()) {
                        return Some(LuaTableFieldKey::Expr(expr));
                    }
                }
                rowan::NodeOrToken::Token(token) => {
                    if let Some(name) = LuaNameToken::cast(token.clone()) {
                        return Some(LuaTableFieldKey::Name(name));
                    }
                }
            }
        }

        None
    }

    pub fn get_value_expr(&self) -> Option<LuaExpr> {
        if self.is_assign_field() {
            self.children().last()
        } else {
            self.child()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaTableFieldKey {
    Name(LuaNameToken),
    Expr(LuaExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaParamName {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaParamName {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ParamName
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaParamName {
    pub fn get_name_token(&self) -> Option<LuaNameToken> {
        self.token()
    }

    pub fn is_dots(&self) -> bool {
        self.token_by_kind(LuaTokenKind::TkDots).is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaParamList {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaParamList {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ParamList
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind().into()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaParamList {
    pub fn get_params(&self) -> LuaAstChildren<LuaParamName> {
        self.children()
    }
}
