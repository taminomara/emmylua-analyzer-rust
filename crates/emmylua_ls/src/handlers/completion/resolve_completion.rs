use emmylua_code_analysis::{DbIndex, SemanticModel};
use lsp_types::{CompletionItem, Documentation, MarkedString, MarkupContent};

use crate::handlers::hover::build_hover_content;

use super::add_completions::CompletionData;

pub fn resolve_completion(
    semantic_model: Option<&SemanticModel>,
    db: &DbIndex,
    completion_item: &mut CompletionItem,
    completion_data: CompletionData,
) -> Option<()> {
    // todo: resolve completion
    match completion_data {
        CompletionData::PropertyOwnerId(property_id) => {
            let hover_content = build_hover_content(semantic_model, db, None, property_id, false);
            if let Some(hover_content) = hover_content {
                match hover_content.type_signature {
                    MarkedString::String(s) => {
                        completion_item.detail = Some(s);
                    }
                    MarkedString::LanguageString(s) => {
                        completion_item.detail = Some(s.value);
                    }
                }
                completion_item.documentation = Some(Documentation::MarkupContent(MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: markdown_to_string(hover_content.detailed_description, true),
                }));
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
