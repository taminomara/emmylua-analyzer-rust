use std::collections::HashMap;

use emmylua_code_analysis::{DiagnosticCode, SemanticModel};
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaComment, LuaCommentOwner, LuaDocTag, LuaDocTagDiagnostic, LuaStat,
    LuaTokenKind,
};
use lsp_types::{Position, Range, TextEdit, Uri};
use rowan::TokenAtOffset;

use crate::handlers::command::DisableAction;

pub fn build_disable_next_line_changes(
    semantic_model: &SemanticModel<'_>,
    start: Position,
    code: DiagnosticCode,
) -> Option<HashMap<Uri, Vec<TextEdit>>> {
    let document = semantic_model.get_document();
    let offset = document.get_offset(start.line as usize, start.character as usize)?;
    let root = semantic_model.get_root();
    let token = match root.syntax().token_at_offset(offset.into()) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, token) => token,
        _ => return None,
    };

    let stat = token.parent_ancestors().find_map(LuaStat::cast)?;
    let text_edit = if let Some(comment) = stat.get_left_comment() {
        if let Some(diagnostic_tag) =
            find_diagnostic_disable_tag(comment.clone(), DisableAction::DisableLine)
        {
            let new_start = if let Some(actions_list) = diagnostic_tag.get_code_list() {
                actions_list.get_range().end()
            } else {
                diagnostic_tag.get_range().end()
            };

            let (line, col) = document.get_line_col(new_start)?;
            TextEdit {
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: col as u32,
                    },
                    end: Position {
                        line: line as u32,
                        character: col as u32,
                    },
                },
                new_text: format!(", {}", code.get_name()),
            }
        } else {
            let indent_text = if let Some(prefix_token) = comment.syntax().prev_sibling_or_token() {
                if prefix_token.kind() == LuaTokenKind::TkWhitespace.into() {
                    prefix_token.into_token()?.text().to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };

            let line = document.get_line(comment.get_position())?;
            TextEdit {
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line as u32,
                        character: 0,
                    },
                },
                new_text: format!(
                    "{}---@diagnostic disable-next-line: {}\n",
                    indent_text,
                    code.get_name()
                ),
            }
        }
    } else {
        let indent_text = if let Some(prefix_token) = stat.syntax().prev_sibling_or_token() {
            if prefix_token.kind() == LuaTokenKind::TkWhitespace.into() {
                prefix_token.into_token()?.text().to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        let line = document.get_line(stat.get_position())?;
        TextEdit {
            range: Range {
                start: Position {
                    line: line as u32,
                    character: 0,
                },
                end: Position {
                    line: line as u32,
                    character: 0,
                },
            },
            new_text: format!(
                "{}---@diagnostic disable-next-line: {}\n",
                indent_text,
                code.get_name()
            ),
        }
    };

    let mut changes = HashMap::new();
    let uri = document.get_uri();
    changes.insert(uri, vec![text_edit]);

    Some(changes)
}

pub fn build_disable_file_changes(
    semantic_model: &SemanticModel<'_>,
    code: DiagnosticCode,
) -> Option<HashMap<Uri, Vec<TextEdit>>> {
    let root = semantic_model.get_root();
    let first_block = root.get_block()?;
    let first_child = first_block.children::<LuaAst>().next()?;
    let document = semantic_model.get_document();
    let text_edit = if let LuaAst::LuaComment(comment) = first_child {
        if let Some(diagnostic_tag) =
            find_diagnostic_disable_tag(comment.clone(), DisableAction::DisableFile)
        {
            let new_start = if let Some(actions_list) = diagnostic_tag.get_code_list() {
                actions_list.get_range().end()
            } else {
                diagnostic_tag.get_range().end()
            };

            let (line, col) = document.get_line_col(new_start)?;
            TextEdit {
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: col as u32,
                    },
                    end: Position {
                        line: line as u32,
                        character: col as u32,
                    },
                },
                new_text: format!(", {}", code.get_name()),
            }
        } else {
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
                new_text: format!("---@diagnostic disable: {}\n", code.get_name()),
            }
        }
    } else {
        TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: format!("---@diagnostic disable: {}\n", code.get_name()),
        }
    };

    let mut changes = HashMap::new();
    let uri = document.get_uri();
    changes.insert(uri, vec![text_edit]);

    Some(changes)
}

fn find_diagnostic_disable_tag(
    comment: LuaComment,
    action: DisableAction,
) -> Option<LuaDocTagDiagnostic> {
    let diagnostic_tags = comment.get_doc_tags().into_iter().filter_map(|tag| {
        if let LuaDocTag::Diagnostic(diagnostic) = tag {
            Some(diagnostic)
        } else {
            None
        }
    });

    for diagnostic_tag in diagnostic_tags {
        let action_token = diagnostic_tag.get_action_token()?;
        let action_token_text = action_token.get_name_text();
        match action {
            DisableAction::DisableLine => {
                if action_token_text == "disable-next-line" {
                    return Some(diagnostic_tag);
                }
            }
            DisableAction::DisableFile | DisableAction::DisableProject => {
                if action_token_text == "disable" {
                    return Some(diagnostic_tag);
                }
            }
        }
    }
    None
}
