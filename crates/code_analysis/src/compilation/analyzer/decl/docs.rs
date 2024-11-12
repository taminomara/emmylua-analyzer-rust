use emmylua_parser::{
    LuaAstToken, LuaAstTokenChildren, LuaDocTagAlias, LuaDocTagClass, LuaDocTagEnum, LuaDocTagMeta,
    LuaDocTagNamespace, LuaDocTagUsing, LuaNameToken,
};
use flagset::FlagSet;

use crate::{db_index::{AnalyzeError, LuaDeclTypeKind, LuaTypeAttribute}, DiagnosticCode};

use super::DeclAnalyzer;

pub fn analyze_doc_tag_class(analyzer: &mut DeclAnalyzer, class: LuaDocTagClass) {
    let (name, range) = if let Some(name_token) = class.get_name_token() {
        (
            name_token.get_name_text().to_string(),
            name_token.syntax().text_range(),
        )
    } else {
        return;
    };

    let attrib = if let Some(attrib) = class.get_attrib() {
        get_attrib_value(attrib.get_attrib_tokens())
    } else {
        None
    };

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
            AnalyzeError::new(
                DiagnosticCode::DuplicateType,
                e,
                range,
            ),
        );
    }
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
            "global" => {
                attr |= LuaTypeAttribute::Global;
            }
            "exact" => {
                attr |= LuaTypeAttribute::Exact;
            }
            _ => {}
        }
    }
    Some(attr)
}

pub fn analyze_doc_tag_enum(analyzer: &mut DeclAnalyzer, enum_: LuaDocTagEnum) {
    let (name, range) = if let Some(name_token) = enum_.get_name_token() {
        (
            name_token.get_name_text().to_string(),
            name_token.syntax().text_range(),
        )
    } else {
        return;
    };

    let attrib = if let Some(attrib) = enum_.get_attrib() {
        get_attrib_value(attrib.get_attrib_tokens())
    } else {
        None
    };

    let file_id = analyzer.get_file_id();
    let r = analyzer
        .db
        .get_type_index_mut()
        .add_type_decl(file_id, range, name, LuaDeclTypeKind::Enum, attrib);

    if let Err(e) = r {
        analyzer.db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(
                DiagnosticCode::DuplicateType,
                e,
                range,
            ),
        );
    }
}

pub fn analyze_doc_tag_alias(analyzer: &mut DeclAnalyzer, alias: LuaDocTagAlias) {
    let (name, range) = if let Some(name_token) = alias.get_name_token() {
        (
            name_token.get_name_text().to_string(),
            name_token.syntax().text_range(),
        )
    } else {
        return;
    };

    let file_id = analyzer.get_file_id();
    let r = analyzer
        .db
        .get_type_index_mut()
        .add_type_decl(file_id, range, name, LuaDeclTypeKind::Alias, None);

    if let Err(e) = r {
        analyzer.db.get_diagnostic_index_mut().add_diagnostic(
            file_id,
            AnalyzeError::new(
                DiagnosticCode::DuplicateType,
                e,
                range,
            ),
        );
    }
}

pub fn analyze_doc_tag_namespace(analyzer: &mut DeclAnalyzer, namespace: LuaDocTagNamespace) {
    let name = if let Some(name_token) = namespace.get_name_token() {
        name_token.get_name_text().to_string()
    } else {
        return;
    };

    let file_id = analyzer.get_file_id();
    analyzer
        .db
        .get_type_index_mut()
        .add_file_namespace(file_id, name);
}

pub fn analyze_doc_tag_using(analyzer: &mut DeclAnalyzer, using: LuaDocTagUsing) {
    let name = if let Some(name_token) = using.get_name_token() {
        name_token.get_name_text().to_string()
    } else {
        return;
    };

    let file_id = analyzer.get_file_id();
    analyzer
        .db
        .get_type_index_mut()
        .add_file_using_namespace(file_id, name)
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
