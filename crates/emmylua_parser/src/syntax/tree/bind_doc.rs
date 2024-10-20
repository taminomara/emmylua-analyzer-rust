use std::collections::HashMap;

use crate::kind::{LuaKind, LuaSyntaxKind, LuaTokenKind};
use crate::{LuaSyntaxNode, LuaSyntaxNodePtr};

pub fn bind_doc(
    root: &LuaSyntaxNode,
) -> (
    HashMap<LuaSyntaxNodePtr, Vec<LuaSyntaxNodePtr>>,
    HashMap<LuaSyntaxNodePtr, LuaSyntaxNodePtr>,
) {
    let mut comments: HashMap<LuaSyntaxNodePtr, Vec<LuaSyntaxNodePtr>> = HashMap::new();
    let mut comment_owner: HashMap<LuaSyntaxNodePtr, LuaSyntaxNodePtr> = HashMap::new();

    for node in root
        .descendants()
        .filter(|it| it.kind() == LuaKind::Syntax(LuaSyntaxKind::Comment))
    {
        let comment_ptr = LuaSyntaxNodePtr::new(&node);
        if let Some(inline_node) = find_inline_comment(&node) {
            let owner_ptr = LuaSyntaxNodePtr::new(&inline_node);
            comment_owner.insert(comment_ptr, owner_ptr);
            comments
                .entry(owner_ptr)
                .or_insert_with(Vec::new)
                .push(comment_ptr);
        } else if let Some(attached_node) = find_attached_comment(&node) {
            let owner_ptr = LuaSyntaxNodePtr::new(&attached_node);
            comment_owner.insert(comment_ptr, owner_ptr);
            comments
                .entry(owner_ptr)
                .or_insert_with(Vec::new)
                .push(comment_ptr);
        }
    }
    (comments, comment_owner)
}

fn find_inline_comment(comment: &LuaSyntaxNode) -> Option<LuaSyntaxNode> {
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

fn find_attached_comment(comment: &LuaSyntaxNode) -> Option<LuaSyntaxNode> {
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
