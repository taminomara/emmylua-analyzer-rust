mod global_gen;
mod index_gen;
mod mod_gen;
mod typ_gen;

use emmylua_code_analysis::{DbIndex, LuaDeprecated, LuaSemanticDeclId};
pub use global_gen::generate_global_markdown;
pub use index_gen::generate_index;
pub use mod_gen::generate_module_markdown;
pub use typ_gen::generate_type_markdown;

use super::markdown_types::Property;

fn collect_property(db: &DbIndex, semantic_decl: LuaSemanticDeclId) -> Property {
    let mut doc_property = Property::default();
    let property = db.get_property_index().get_property(&semantic_decl);
    if let Some(property) = property {
        if let Some(description) = property.description.clone() {
            doc_property.description = Some(description.to_string());
        }

        if let Some(see) = property.see_content.clone() {
            doc_property.see = Some(see.to_string());
        }

        if let Some(deprecated) = &property.deprecated {
            match deprecated {
                LuaDeprecated::Deprecated => {
                    doc_property.deprecated = Some("Deprecated".to_string())
                }
                LuaDeprecated::DeprecatedWithMessage(message) => {
                    doc_property.deprecated = Some(message.to_string())
                }
            }
        }
        if let Some(other) = property.other_content.clone() {
            doc_property.other = Some(other.to_string());
        }
    }

    doc_property
}
