use crate::{LuaAstNode, LuaDocDetailToken, LuaSyntaxKind, LuaSyntaxNode};

#[allow(unused)]
pub trait LuaDocDescriptionOwner: LuaAstNode {
    fn get_description(&self) -> Option<LuaDocDescription> {
        self.child()   
    }
}

#[allow(unused)]
pub trait LuaDocDetailOwner: LuaAstNode {
    fn get_detail(&self) -> Option<LuaDocDetailToken> {
        self.token()
    }

    fn get_detail_text(&self) -> Option<String> {
        self.get_detail().map(|it| it.get_detail().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaDocDescription {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaDocDescription {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::DocDescription
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == LuaSyntaxKind::DocDescription.into() {
            Some(Self { syntax })
        } else {
            None
        }
    }
}

impl LuaDocDescription {
    pub fn get_detail_text(&self) -> Option<String> {
        todo!()
    }
}
