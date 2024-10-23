use crate::{kind::{LuaSyntaxKind, LuaTokenKind}, syntax::traits::LuaAstNode, LuaKind, LuaSyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaComment {
    syntax: LuaSyntaxNode,
}

impl LuaAstNode for LuaComment {
    fn syntax(&self) -> &LuaSyntaxNode {
        &self.syntax
    }

    fn can_cast(kind: LuaSyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == LuaSyntaxKind::Comment
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

impl LuaComment {
    pub fn get_owner(&self) -> Option<LuaSyntaxNode> {
        if let Some(inline_node) = find_inline_node(&self.syntax) {
            Some(inline_node)
        } else if let Some(attached_node) = find_attached_node(&self.syntax) {
            Some(attached_node)
        }
        else {
            None
        }
    }

    pub fn get_description_text(&self) -> Option<String> {
        // let descriptions = self.children::<Lua>()
        todo!()
    }

    // pub fn get_doc_tags(&self) -> LuaAstChildren<LuaTag> {
    //     todo!()
    // }
}

fn find_inline_node(comment: &LuaSyntaxNode) -> Option<LuaSyntaxNode> {
    let mut prev_sibling = comment.next_sibling_or_token();
    loop {
        if prev_sibling.is_none() {
            return None;
        }

        if let Some(sibling) = prev_sibling {
            match sibling.kind() {
                LuaKind::Token(
                    LuaTokenKind::TkWhitespace | LuaTokenKind::TkComma | LuaTokenKind::TkSemicolon,
                ) => {}
                LuaKind::Token(LuaTokenKind::TkEndOfLine)
                | LuaKind::Syntax(LuaSyntaxKind::Comment) => {
                    return None;
                }
                LuaKind::Token(k) if k != LuaTokenKind::TkName => {
                    return Some(comment.parent()?);
                }
                _ => match sibling {
                    rowan::NodeOrToken::Node(node) => {
                        return Some(node);
                    }
                    rowan::NodeOrToken::Token(token) => {
                        return Some(token.parent()?);
                    }
                },
            }
            prev_sibling = sibling.prev_sibling_or_token();
        } else {
            return None;
        }
    }
}

fn find_attached_node(comment: &LuaSyntaxNode) -> Option<LuaSyntaxNode> {
    let mut meet_end_of_line = false;
    
    let mut next_sibling = comment.next_sibling_or_token();
    loop {
        if next_sibling.is_none() {
            return None;
        }

        if let Some(sibling) = next_sibling {
            match sibling.kind() {
                LuaKind::Token(LuaTokenKind::TkEndOfLine) => {
                    if meet_end_of_line {
                        return None;
                    }

                    meet_end_of_line = true;
                }
                LuaKind::Token(LuaTokenKind::TkWhitespace) => {}
                LuaKind::Syntax(LuaSyntaxKind::Comment) => {
                    return None;
                }
                LuaKind::Syntax(LuaSyntaxKind::Block) => {
                    let first_child = comment.first_child()?;
                    if first_child.kind() == LuaKind::Syntax(LuaSyntaxKind::Comment) {
                        return None
                    }
                    return Some(first_child);
                }
                _ => {
                    match sibling {
                        rowan::NodeOrToken::Node(node) => {
                            return Some(node);
                        }
                        rowan::NodeOrToken::Token(token) => {
                            return Some(token.parent()?);
                        }
                    }
                }
            }
            next_sibling = sibling.next_sibling_or_token();
        }
    }
}