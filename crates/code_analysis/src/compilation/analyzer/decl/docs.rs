use emmylua_parser::{
    LuaAstToken, LuaAstTokenChildren, LuaDocTagAlias, LuaDocTagClass, LuaDocTagEnum, LuaDocTagMeta,
    LuaDocTagNamespace, LuaDocTagUsing, LuaNameToken,
};
use flagset::FlagSet;

use crate::{
    db_index::{AnalyzeError, LuaDeclTypeKind, LuaTypeAttribute},
    DiagnosticCode,
};

use super::DeclAnalyzer;

pub fn analyze_doc_tag_class(analyzer: &mut DeclAnalyzer, class: LuaDocTagClass) -> Option<()> {
    let name_token = class.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    let range = name_token.syntax().text_range();

    let attrib = get_attrib_value(class.get_attrib()?.get_attrib_tokens());

    let file_id = analyzer.get_file_id();
    let r = analyzer.db.get_type_index_mut().add_type_decl(
        file_id,
        range,
        name,
        LuaDeclTypeKind::Class,
        attrib,
    );

    if let Err(e) = r {
        analyzer.db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(DiagnosticCode::DuplicateType, &e, range),
        );
    }

    Some(())
}

fn get_attrib_value(
    attrib: LuaAstTokenChildren<LuaNameToken>,
) -> Option<FlagSet<LuaTypeAttribute>> {
    let mut attr: FlagSet<LuaTypeAttribute> = LuaTypeAttribute::None.into();

    for token in attrib {
        match token.get_name_text() {
            "partial" => {
                attr |= LuaTypeAttribute::Partial;
            }
            "key" => {
                attr |= LuaTypeAttribute::Key;
            }
            // "global" => {
            //     attr |= LuaTypeAttribute::Global;
            // }
            "exact" => {
                attr |= LuaTypeAttribute::Exact;
            }
            _ => {}
        }
    }
    Some(attr)
}

pub fn analyze_doc_tag_enum(analyzer: &mut DeclAnalyzer, enum_: LuaDocTagEnum) -> Option<()> {
    let name_token = enum_.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    let range = name_token.syntax().text_range();

    let attrib = get_attrib_value(enum_.get_attrib()?.get_attrib_tokens());

    let file_id = analyzer.get_file_id();
    let r = analyzer.db.get_type_index_mut().add_type_decl(
        file_id,
        range,
        name,
        LuaDeclTypeKind::Enum,
        attrib,
    );

    if let Err(e) = r {
        analyzer.db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(DiagnosticCode::DuplicateType, &e, range),
        );
    }

    Some(())
}

pub fn analyze_doc_tag_alias(analyzer: &mut DeclAnalyzer, alias: LuaDocTagAlias) -> Option<()> {
    let name_token = alias.get_name_token()?;
    let name = name_token.get_name_text().to_string();
    let range = name_token.syntax().text_range();

    let file_id = analyzer.get_file_id();
    let r = analyzer.db.get_type_index_mut().add_type_decl(
        file_id,
        range,
        name,
        LuaDeclTypeKind::Alias,
        None,
    );

    if let Err(e) = r {
        analyzer.db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(DiagnosticCode::DuplicateType, &e, range),
        );
    }

    Some(())
}

pub fn analyze_doc_tag_namespace(
    analyzer: &mut DeclAnalyzer,
    namespace: LuaDocTagNamespace,
) -> Option<()> {
    let name = namespace.get_name_token()?.get_name_text().to_string();

    let file_id = analyzer.get_file_id();
    analyzer
        .db
        .get_type_index_mut()
        .add_file_namespace(file_id, name);

    Some(())
}

pub fn analyze_doc_tag_using(analyzer: &mut DeclAnalyzer, using: LuaDocTagUsing) -> Option<()> {
    let name = using.get_name_token()?.get_name_text().to_string();

    let file_id = analyzer.get_file_id();
    analyzer
        .db
        .get_type_index_mut()
        .add_file_using_namespace(file_id, name);

    Some(())
}

pub fn analyze_doc_tag_meta(analyzer: &mut DeclAnalyzer, tag: LuaDocTagMeta) {
    let file_id = analyzer.get_file_id();
    analyzer.db.get_meta_file_mut().add_meta_file(file_id);

    if let Some(name_token) = tag.get_name_token() {
        if name_token.get_name_text() == "no-require" {
            analyzer
                .db
                .get_module_index_mut()
                .set_module_visibility(file_id, false);
        } else {
            analyzer
                .db
                .get_module_index_mut()
                .add_module_by_module_path(file_id, name_token.get_name_text().to_string());
        }
    }
}
