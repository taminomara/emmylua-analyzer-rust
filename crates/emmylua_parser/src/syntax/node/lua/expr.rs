use crate::{
    kind::LuaSyntaxKind,
    syntax::{
        node::{LuaBinaryOpToken, LuaNameToken, LuaUnaryOpToken},
        traits::{LuaAstChildren, LuaAstNode},
    },
    LuaIndexToken, LuaLiteralToken, LuaSyntaxNode,
};

use super::LuaCallArgList;

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
            LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr
            | LuaSyntaxKind::TableEmptyExpr => true,
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
        match syntax.kind().into() {
            LuaSyntaxKind::CallExpr => LuaCallExpr::cast(syntax).map(LuaExpr::CallExpr),
            LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr
            | LuaSyntaxKind::TableEmptyExpr => LuaTableExpr::cast(syntax).map(LuaExpr::TableExpr),
            LuaSyntaxKind::LiteralExpr => LuaLiteralExpr::cast(syntax).map(LuaExpr::LiteralExpr),
            LuaSyntaxKind::BinaryExpr => LuaBinaryExpr::cast(syntax).map(LuaExpr::BinaryExpr),
            LuaSyntaxKind::UnaryExpr => LuaUnaryExpr::cast(syntax).map(LuaExpr::UnaryExpr),
            LuaSyntaxKind::ClosureExpr => LuaClosureExpr::cast(syntax).map(LuaExpr::ClosureExpr),
            LuaSyntaxKind::ParenExpr => LuaParenExpr::cast(syntax).map(LuaExpr::ParenExpr),
            LuaSyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaExpr::NameExpr),
            LuaSyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaExpr::IndexExpr),
            _ => None,
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
        match syntax.kind().into() {
            LuaSyntaxKind::NameExpr => LuaNameExpr::cast(syntax).map(LuaVarExpr::NameExpr),
            LuaSyntaxKind::IndexExpr => LuaIndexExpr::cast(syntax).map(LuaVarExpr::IndexExpr),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaSingleArgExpr {
    TableExpr(LuaTableExpr),
    LiteralExpr(LuaLiteralExpr),
}

impl LuaAstNode for LuaSingleArgExpr {
    fn syntax(&self) -> &LuaSyntaxNode {
        match self {
            LuaSingleArgExpr::TableExpr(node) => node.syntax(),
            LuaSingleArgExpr::LiteralExpr(node) => node.syntax(),
        }
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr
            | LuaSyntaxKind::TableEmptyExpr => true,
            LuaSyntaxKind::LiteralExpr => true,
            _ => false,
        }
    }

    fn cast(syntax: LuaSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind().into() {
            LuaSyntaxKind::TableArrayExpr
            | LuaSyntaxKind::TableObjectExpr
            | LuaSyntaxKind::TableEmptyExpr => {
                LuaTableExpr::cast(syntax).map(LuaSingleArgExpr::TableExpr)
            }
            LuaSyntaxKind::LiteralExpr => {
                LuaLiteralExpr::cast(syntax).map(LuaSingleArgExpr::LiteralExpr)
            }
            _ => None,
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

impl LuaNameExpr {
    pub fn get_name(&self) -> Option<LuaNameToken> {
        self.token()
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

impl LuaIndexExpr {
    pub fn get_prefix_expr(&self) -> Option<LuaVarExpr> {
        self.child()
    }

    pub fn get_indexed_expr(&self) -> Option<LuaExpr> {
        self.children().nth(1)
    }

    pub fn get_index_token(&self) -> Option<LuaIndexToken> {
        self.token()
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

impl LuaCallExpr {
    pub fn get_prefix_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_args_list(&self) -> Option<LuaCallArgList> {
        self.child()
    }
}

/// In Lua, tables are a fundamental data structure that can be used to represent arrays, objects, 
/// and more. To facilitate parsing and handling of different table structures, we categorize tables 
/// into three types: `TableArrayExpr`, `TableObjectExpr`, and `TableEmptyExpr`.
///
/// - `TableArrayExpr`: Represents a table used as an array, where elements are indexed by integers.
/// - `TableObjectExpr`: Represents a table used as an object, where elements are indexed by strings or other keys.
/// - `TableEmptyExpr`: Represents an empty table with no elements.
///
/// This categorization helps in accurately parsing and processing Lua code by distinguishing between 
/// different uses of tables, thereby enabling more precise syntax analysis and manipulation.
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
        kind == LuaSyntaxKind::TableArrayExpr || kind == LuaSyntaxKind::TableObjectExpr || kind == LuaSyntaxKind::TableEmptyExpr
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

impl LuaTableExpr {
    pub fn is_empty(&self) -> bool {
        self.syntax().kind() == LuaSyntaxKind::TableEmptyExpr.into()
    }

    pub fn is_array(&self) -> bool {
        self.syntax().kind() == LuaSyntaxKind::TableArrayExpr.into()
    }

    pub fn is_object(&self) -> bool {
        self.syntax().kind() == LuaSyntaxKind::TableObjectExpr.into()
    }

    pub fn get_fields(&self) -> LuaAstChildren<LuaExpr> {
        self.children()
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

impl LuaLiteralExpr {
    pub fn get_literal(&self) -> Option<LuaLiteralToken> {
        self.token()
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
    pub fn get_exprs(&self) -> Option<(LuaExpr, LuaExpr)> {
        let exprs = self.children::<LuaExpr>().collect::<Vec<_>>();
        if exprs.len() == 2 {
            Some((exprs[0].clone(), exprs[1].clone()))
        } else {
            None
        }
    }

    pub fn get_op_token(&self) -> Option<LuaBinaryOpToken> {
        self.token()
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
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }

    pub fn get_op_token(&self) -> Option<LuaUnaryOpToken> {
        self.token()
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
    pub fn get_expr(&self) -> Option<LuaExpr> {
        self.child()
    }
}
