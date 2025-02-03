use emmylua_code_analysis::DbIndex;
use lsp_types::{CompletionItem, Documentation, MarkupContent};

use super::add_completions::CompletionData;

pub fn resolve_completion(
    db: &DbIndex,
    completion_item: &mut CompletionItem,
    completion_data: CompletionData,
) -> Option<()> {
    // todo: resolve completion
    match completion_data {
        CompletionData::PropertyOwnerId(property_id) => {
            let property = db.get_property_index().get_property(property_id)?;
            if let Some(des) = &property.description {
                completion_item.documentation = Some(Documentation::MarkupContent(MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: des.to_string(),
                }));
            }
        }
        _ => {}
    }

    Some(())
}
