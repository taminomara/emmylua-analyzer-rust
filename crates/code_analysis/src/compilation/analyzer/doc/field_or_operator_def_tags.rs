use emmylua_parser::{LuaDocFieldKey, LuaDocTagField};

use super::DocAnalyzer;


pub fn analyze_field(analyzer: &mut DocAnalyzer, tag: LuaDocTagField) -> Option<()> {
    let visibility_kind = if let Some(visibility_token) = tag.get_visibility_token() {
        Some(visibility_token.get_visibility())
    } else {
        None
    };

    let key = tag.get_field_key()?;
    match key {
        LuaDocFieldKey::Name(name_token) => {

        }
        LuaDocFieldKey::String(string_token) => {

        }
        LuaDocFieldKey::Integer(int_token) => {

        }
        LuaDocFieldKey::Type(doc_type) => {

        }
    }

    Some(())
}
