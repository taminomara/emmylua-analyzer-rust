use emmylua_parser::{LuaAstNode, LuaNameExpr, LuaSyntaxKind};
use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails, InsertTextFormat, InsertTextMode};

use crate::handlers::completion::{completion_context::CompletionContext, data::{KEYWORD_COMPLETIONS, KEYWORD_EXPR_COMPLETIONS}};

pub fn add_completion(context: &mut CompletionContext) -> Option<()> {
    if context.is_cancelled() {
        return None;
    }

    let name_expr = LuaNameExpr::cast(context.trigger_token.parent()?)?;
    add_stat_keyword_completions(context, name_expr);

    add_expr_keyword_completions(context);
    Some(())
}

fn add_stat_keyword_completions(context: &mut CompletionContext, name_expr: LuaNameExpr) -> Option<()> {
    if name_expr.syntax().parent()?.parent()?.kind() != LuaSyntaxKind::Block.into() {
        return None;
    }

    for keyword_info in KEYWORD_COMPLETIONS {
        let item = CompletionItem {
            label: keyword_info.label.to_string(),
            kind: Some(keyword_info.kind),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some(keyword_info.detail.to_string()),
                ..CompletionItemLabelDetails::default()
            }),
            insert_text: Some(keyword_info.insert_text.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: Some(InsertTextMode::ADJUST_INDENTATION),
            ..CompletionItem::default()
        };

        context.add_completion_item(item)?;
    }

    Some(())
}

fn add_expr_keyword_completions(context: &mut CompletionContext) -> Option<()> {
    for keyword_info in KEYWORD_EXPR_COMPLETIONS {
        let item = CompletionItem {
            label: keyword_info.label.to_string(),
            kind: Some(keyword_info.kind),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some(keyword_info.detail.to_string()),
                ..CompletionItemLabelDetails::default()
            }),
            insert_text: Some(keyword_info.insert_text.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: Some(InsertTextMode::ADJUST_INDENTATION),
            ..CompletionItem::default()
        };

        context.add_completion_item(item)?;
    }

    Some(())
}