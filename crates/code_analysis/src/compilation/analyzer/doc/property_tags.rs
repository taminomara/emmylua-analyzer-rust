use emmylua_parser::{
    LuaDocDescriptionOwner, LuaDocTagDeprecated, LuaDocTagSource, LuaDocTagVersion, LuaDocTagVisibility
};


use super::{tags::get_owner_id, DocAnalyzer};

pub fn analyze_visibility(
    analyzer: &mut DocAnalyzer,
    visibility: LuaDocTagVisibility,
) -> Option<()> {
    let visibility_kind = visibility.get_visibility_token()?.get_visibility();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_visibility(analyzer.file_id, owner_id, visibility_kind);

    Some(())
}

pub fn analyze_source(analyzer: &mut DocAnalyzer, source: LuaDocTagSource) -> Option<()> {
    let source = source.get_path_token()?.get_path().to_string();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_source(analyzer.file_id, owner_id, source);

    Some(())
}

pub fn analyze_nodiscard(analyzer: &mut DocAnalyzer) -> Option<()> {
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_nodiscard(analyzer.file_id, owner_id);

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
        .get_property_index()
        .add_deprecated(analyzer.file_id, owner_id, message);

    Some(())
}

pub fn analyze_version(analyzer: &mut DocAnalyzer, version: LuaDocTagVersion) -> Option<()> {
    // let version = version.get_version_token()?.get_version().to_string();
    // let owner_id = get_owner_id(analyzer)?;

    // analyzer
    //     .db
    //     .get_property_index()
    //     .add_version(analyzer.file_id, owner_id, version);

    Some(())
}