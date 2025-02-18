use emmylua_code_analysis::{DbIndex, SemanticModel};
use lsp_types::{CompletionItem, Documentation, MarkedString, MarkupContent};

use crate::{
    context::ClientId,
    handlers::hover::{build_hover_content, HoverContent},
};

use super::add_completions::CompletionData;

pub fn resolve_completion(
    semantic_model: Option<&SemanticModel>,
    db: &DbIndex,
    completion_item: &mut CompletionItem,
    completion_data: CompletionData,
    client_id: ClientId,
) -> Option<()> {
    // todo: resolve completion
    match completion_data {
        CompletionData::PropertyOwnerId(property_id) => {
            let hover_content = build_hover_content(semantic_model, db, None, property_id, false);
            if let Some(hover_content) = hover_content {
                if client_id.is_vscode() {
                    match hover_content.type_signature {
                        MarkedString::String(s) => {
                            completion_item.detail = Some(s);
                        }
                        MarkedString::LanguageString(s) => {
                            completion_item.detail = Some(s.value);
                        }
                    }
                    completion_item.documentation =
                        Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: markdown_to_string(hover_content.detailed_description, true),
                        }));
                } else {
                    build_other(completion_item, hover_content);
                }
            }
        }
        _ => {}
    }
    Some(())
}

fn markdown_to_string(marked_strings: Vec<MarkedString>, remove_first_underscore: bool) -> String {
    let mut result = String::new();
    let mut first_line = true;
    for marked_string in marked_strings {
        match marked_string {
            MarkedString::String(s) => {
                if first_line && remove_first_underscore && s == "---" {
                    first_line = false;
                } else {
                    result.push_str(&format!("\n{}\n", s));
                }
            }
            MarkedString::LanguageString(s) => {
                result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
            }
        }
    }
    result
}

fn build_other(completion_item: &mut CompletionItem, hover_content: HoverContent) -> Option<()> {
    let mut result = String::new();
    match hover_content.type_signature {
        MarkedString::String(s) => {
            result.push_str(&format!("\n{}\n", s));
        }
        MarkedString::LanguageString(s) => {
            result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
        }
    }
    if let Some(location_path) = hover_content.location_path {
        match location_path {
            MarkedString::String(s) => {
                result.push_str(&format!("\n{}\n", s));
            }
            _ => {}
        }
    }
    for marked_string in hover_content.detailed_description {
        match marked_string {
            MarkedString::String(s) => {
                result.push_str(&format!("\n{}\n", s));
            }
            MarkedString::LanguageString(s) => {
                result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
            }
        }
    }
    completion_item.documentation = Some(Documentation::MarkupContent(MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value: result,
    }));
    Some(())
}
