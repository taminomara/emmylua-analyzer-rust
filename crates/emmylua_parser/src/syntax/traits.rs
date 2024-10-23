use std::marker::PhantomData;

use crate::kind::{LuaSyntaxKind, LuaTokenKind};

pub use super::{
    node::*, LuaSyntaxElementChildren, LuaSyntaxNode, LuaSyntaxNodeChildren, LuaSyntaxToken,
};

pub trait LuaAstNode {
    fn syntax(&self) -> &LuaSyntaxNode;

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn child<N: LuaAstNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }

    #[allow(dead_code)]
    fn token<N: LuaAstToken>(&self) -> Option<N> {
        self.syntax()
            .children_with_tokens()
            .find_map(|it| it.into_token())
            .and_then(N::cast)
    }

    #[allow(dead_code)]
    fn token_by_kind(&self, kind: LuaTokenKind) -> Option<LuaGeneralToken> {
        let token = self
            .syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind.into())?;

        LuaGeneralToken::cast(token)
    }

    #[allow(dead_code)]
    fn tokens<N: LuaAstToken>(&self) -> LuaAstTokenChildren<N> {
        LuaAstTokenChildren::new(self.syntax())
    }

    #[allow(dead_code)]
    fn children<N: LuaAstNode>(&self) -> LuaAstChildren<N> {
        LuaAstChildren::new(self.syntax())
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("{:#?}", self.syntax());
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct LuaAstChildren<N> {
    inner: LuaSyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> LuaAstChildren<N> {
    pub fn new(parent: &LuaSyntaxNode) -> LuaAstChildren<N> {
        LuaAstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: LuaAstNode> Iterator for LuaAstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

pub trait LuaAstToken {
    #[allow(dead_code)]
    fn syntax(&self) -> &LuaSyntaxToken;

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn get_token_kind(&self) -> LuaTokenKind {
        self.syntax().kind().into()
    }
}

#[derive(Debug, Clone)]
pub struct LuaAstTokenChildren<N> {
    inner: LuaSyntaxElementChildren,
    ph: PhantomData<N>,
}

impl<N> LuaAstTokenChildren<N> {
    pub fn new(parent: &LuaSyntaxNode) -> LuaAstTokenChildren<N> {
        LuaAstTokenChildren {
            inner: parent.children_with_tokens(),
            ph: PhantomData,
        }
    }
}

impl<N: LuaAstToken> Iterator for LuaAstTokenChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(|it| it.into_token()).and_then(N::cast)
    }
}
