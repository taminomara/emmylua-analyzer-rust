use crate::{kind::{BinaryOperator, LuaSyntaxKind}, syntax::traits::LuaAstNode, LuaSyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaExpr {
    CallExpr(LuaCallExpr),
    TableExpr(LuaTableExpr),
    LiteralExpr(LuaLiteralExpr),
    BinaryExpr(LuaBinaryExpr),
    UnaryExpr(LuaUnaryExpr),
    ClosureExpr(LuaClosureExpr),
    ParenExpr(LuaParenExpr),
    NameExpr(LuaNameExpr),
    IndexExpr(LuaIndexExpr),
}

impl LuaAstNode for LuaExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaExpr::CallExpr(node) => node.syntax(),
            LuaExpr::TableExpr(node) => node.syntax(),
            LuaExpr::LiteralExpr(node) => node.syntax(),
            LuaExpr::BinaryExpr(node) => node.syntax(),
            LuaExpr::UnaryExpr(node) => node.syntax(),
            LuaExpr::ClosureExpr(node) => node.syntax(),
            LuaExpr::ParenExpr(node) => node.syntax(),
            LuaExpr::NameExpr(node) => node.syntax(),
            LuaExpr::IndexExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            LuaSyntaxKind::CallExpr => true,
            LuaSyntaxKind::TableExpr => true,
            LuaSyntaxKind::LiteralExpr => true,
            LuaSyntaxKind::BinaryExpr => true,
            LuaSyntaxKind::UnaryExpr => true,
            LuaSyntaxKind::ClosureExpr => true,
            LuaSyntaxKind::ParenExpr => true,
            LuaSyntaxKind::NameExpr => true,
            LuaSyntaxKind::IndexExpr => true,
            _ => false,
        }
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if LuaCallExpr::can_cast(syntax.kind().into()) {
            LuaCallExpr::cast(syntax).map(LuaExpr::CallExpr)
        } else if LuaTableExpr::can_cast(syntax.kind().into()) {
            LuaTableExpr::cast(syntax).map(LuaExpr::TableExpr)
        } else if LuaLiteralExpr::can_cast(syntax.kind().into()) {
            LuaLiteralExpr::cast(syntax).map(LuaExpr::LiteralExpr)
        } else if LuaBinaryExpr::can_cast(syntax.kind().into()) {
            LuaBinaryExpr::cast(syntax).map(LuaExpr::BinaryExpr)
        } else if LuaUnaryExpr::can_cast(syntax.kind().into()) {
            LuaUnaryExpr::cast(syntax).map(LuaExpr::UnaryExpr)
        } else if LuaClosureExpr::can_cast(syntax.kind().into()) {
            LuaClosureExpr::cast(syntax).map(LuaExpr::ClosureExpr)
        } else if LuaParenExpr::can_cast(syntax.kind().into()) {
            LuaParenExpr::cast(syntax).map(LuaExpr::ParenExpr)
        } else if LuaNameExpr::can_cast(syntax.kind().into()) {
            LuaNameExpr::cast(syntax).map(LuaExpr::NameExpr)
        } else if LuaIndexExpr::can_cast(syntax.kind().into()) {
            LuaIndexExpr::cast(syntax).map(LuaExpr::IndexExpr)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaVarExpr {
    NameExpr(LuaNameExpr),
    IndexExpr(LuaIndexExpr),
}

impl LuaAstNode for LuaVarExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaVarExpr::NameExpr(node) => node.syntax(),
            LuaVarExpr::IndexExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            LuaSyntaxKind::NameExpr => true,
            LuaSyntaxKind::IndexExpr => true,
            _ => false,
        }
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if LuaNameExpr::can_cast(syntax.kind().into()) {
            LuaNameExpr::cast(syntax).map(LuaVarExpr::NameExpr)
        } else if LuaIndexExpr::can_cast(syntax.kind().into()) {
            LuaIndexExpr::cast(syntax).map(LuaVarExpr::IndexExpr)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaNameExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaNameExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::NameExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaIndexExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaIndexExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::IndexExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaCallExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaCallExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::CallExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaTableExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaTableExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::TableExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaLiteralExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaLiteralExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::LiteralExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaBinaryExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaBinaryExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::BinaryExpr
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

impl LuaBinaryExpr {
    #[allow(unused)]
    pub fn get_exprs(&self) -> Option<(LuaExpr, LuaExpr)> {
        let exprs = self.children::<LuaExpr>().collect::<Vec<_>>();
        if exprs.len() == 2 {
            Some((exprs[0].clone(), exprs[1].clone()))
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn get_op(&self) -> Option<BinaryOperator> {
        // self.tokens().find(|it| it.);

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaUnaryExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaUnaryExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::UnaryExpr
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

impl LuaUnaryExpr {
    #[allow(unused)]
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaClosureExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaClosureExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ClosureExpr
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaParenExpr {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaParenExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::ParenExpr
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

impl LuaParenExpr {
    #[allow(unused)]
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}
