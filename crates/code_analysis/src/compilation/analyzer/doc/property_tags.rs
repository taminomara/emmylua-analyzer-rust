use emmylua_parser::{
    BinaryOperator, LuaDocDescriptionOwner, LuaDocTagDeprecated, LuaDocTagSource, LuaDocTagVersion,
    LuaDocTagVisibility,
};

use crate::db_index::{LuaVersionCond, LuaVersionCondOp};

use super::{tags::get_owner_id, DocAnalyzer};

pub fn analyze_visibility(
    analyzer: &mut DocAnalyzer,
    visibility: LuaDocTagVisibility,
) -> Option<()> {
    let visibility_kind = visibility.get_visibility_token()?.get_visibility();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index_mut()
        .add_visibility(analyzer.file_id, owner_id, visibility_kind);

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
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index_mut()
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
        .get_property_index_mut()
        .add_deprecated(analyzer.file_id, owner_id, message);

    Some(())
}

pub fn analyze_version(analyzer: &mut DocAnalyzer, version: LuaDocTagVersion) -> Option<()> {
    let owner_id = get_owner_id(analyzer)?;

    let mut version_set = Vec::new();
    for version in version.get_version_list() {
        let version_number = if let Some(version_number) = version.get_version() {
            version_number.get_version_number()
        } else {
            continue;
        };

        let version_op = if let Some(version_op) = version.get_op() {
            match version_op.get_op() {
                BinaryOperator::OpGt => LuaVersionCondOp::Gt,
                BinaryOperator::OpLt => LuaVersionCondOp::Lt,
                _ => LuaVersionCondOp::Eq,
            }
        } else {
            LuaVersionCondOp::Eq
        };

        if let Some(version_number) = version_number {
            version_set.push(LuaVersionCond::new(version_number, version_op));
        }
    }

    analyzer
        .db
        .get_property_index_mut()
        .add_version(analyzer.file_id, owner_id, version_set);

    Some(())
}
