use crate::LuaSignatureId;

use super::{
    tags::{find_owner_closure, get_owner_id},
    DocAnalyzer,
};
use emmylua_parser::{
    LuaDocDescriptionOwner, LuaDocTagDeprecated, LuaDocTagSource, LuaDocTagVersion,
    LuaDocTagVisibility,
};

pub fn analyze_visibility(
    analyzer: &mut DocAnalyzer,
    visibility: LuaDocTagVisibility,
) -> Option<()> {
    let visibility_kind = visibility.get_visibility_token()?.get_visibility();
    let owner_id = get_owner_id(analyzer)?;

    analyzer.db.get_property_index_mut().add_visibility(
        analyzer.file_id,
        owner_id,
        visibility_kind,
    );

    Some(())
}

pub fn analyze_source(analyzer: &mut DocAnalyzer, source: LuaDocTagSource) -> Option<()> {
    let source = source.get_path_token()?.get_path().to_string();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index_mut()
        .add_source(analyzer.file_id, owner_id, source);

    Some(())
}

pub fn analyze_nodiscard(analyzer: &mut DocAnalyzer) -> Option<()> {
    let closure = find_owner_closure(analyzer)?;
    let signature_id = LuaSignatureId::from_closure(analyzer.file_id, &closure);
    let signature = analyzer
        .db
        .get_signature_index_mut()
        .get_mut(&signature_id)?;

    signature.is_nodiscard = true;

    Some(())
}

pub fn analyze_deprecated(analyzer: &mut DocAnalyzer, tag: LuaDocTagDeprecated) -> Option<()> {
    let message = if let Some(desc) = tag.get_description() {
        Some(desc.get_description_text().to_string())
    } else {
        None
    };
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index_mut()
        .add_deprecated(analyzer.file_id, owner_id, message);

    Some(())
}

pub fn analyze_version(analyzer: &mut DocAnalyzer, version: LuaDocTagVersion) -> Option<()> {
    let owner_id = get_owner_id(analyzer)?;

    let mut version_set = Vec::new();
    for version in version.get_version_list() {
        if let Some(version_condition) = version.get_version_condition() {
            version_set.push(version_condition);
        }
    }

    analyzer
        .db
        .get_property_index_mut()
        .add_version(analyzer.file_id, owner_id, version_set);

    Some(())
}

pub fn analyze_async(analyzer: &mut DocAnalyzer) -> Option<()> {
    let closure = find_owner_closure(analyzer)?;
    let signature_id = LuaSignatureId::from_closure(analyzer.file_id, &closure);
    let signature = analyzer
        .db
        .get_signature_index_mut()
        .get_mut(&signature_id)?;

    signature.is_async = true;

    Some(())
}
