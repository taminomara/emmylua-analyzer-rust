use std::marker::PhantomData;

use crate::kind::{LuaSyntaxKind, LuaTokenKind};

use super::{
    node::LuaComment, LuaSyntaxNode, LuaSyntaxNodeChildren, LuaSyntaxNodePtr, LuaSyntaxToken,
    LuaSyntaxTree,
};

pub trait LuaNode {
    fn syntax(&self) -> &LuaSyntaxNode;

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn child<N: LuaNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }

    fn token(&self, kind: LuaTokenKind) -> Option<LuaSyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind.into())
    }

    fn tokens(&self, kind: LuaTokenKind) -> Vec<LuaSyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .filter(|it| it.kind() == kind.into())
            .collect()
    }

    fn children<N: LuaNode>(&self) -> LuaNodeChildren<N> {
        LuaNodeChildren::new(self.syntax())
    }

    fn dump(&self) {
        println!("{:#?}", self.syntax());
    }
}

pub trait LuaToken {
    fn syntax(&self) -> &LuaSyntaxToken;

    fn can_cast(kind: LuaTokenKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: LuaSyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn get_text(&self) -> &str {
        self.syntax().text()
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct LuaNodeChildren<N> {
    inner: LuaSyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> LuaNodeChildren<N> {
    pub fn new(parent: &LuaSyntaxNode) -> LuaNodeChildren<N> {
        LuaNodeChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: LuaNode> Iterator for LuaNodeChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

pub trait LuaCommentOwner: LuaNode {
    fn get_comments(&self, t: &LuaSyntaxTree) -> Option<Vec<LuaComment>> {
        let ptr = LuaSyntaxNodePtr::new(self.syntax());
        let root = t.get_red_root();
        match t.get_comments(ptr) {
            Some(comments) => Some(
                comments
                    .iter()
                    .map(|it| LuaComment::cast(it.to_node(&root)).unwrap())
                    .collect(),
            ),
            None => None,
        }
    }
}
