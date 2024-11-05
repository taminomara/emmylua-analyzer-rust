use emmylua_parser::{LuaDocGenericDeclList, LuaDocTag, LuaDocTagClass};

use crate::db_index::LuaType;

use super::{infer_type::infer_type, DocAnalyzer};

pub fn analyze_tag(analyzer: &mut DocAnalyzer, tag: LuaDocTag) {
    match tag {
        LuaDocTag::Class(class) => {
            analyze_class(analyzer, class);
        }
        _ => {}
    }
}

fn analyze_class(analyzer: &mut DocAnalyzer, tag: LuaDocTagClass) {
    let file_id = analyzer.file_id;
    // let class_decl = analyzer.db.get_type_index().find_type_decl(file_id, name)
    if let Some(generic_params) = tag.get_generic_decl() {}
}

fn get_generic_params(analyzer: &mut DocAnalyzer, params: LuaDocGenericDeclList) -> Vec<(String, Option<LuaType>)> {
    let mut params_result = Vec::new();
    for param in params.get_generic_decl() {
        let name = if let Some(param) = param.get_name_token() {
            param.get_name_text().to_string()
        } else {
            continue;
        };

        let type_ref = if let Some(type_ref) = param.get_type() {
            Some(infer_type(&mut analyzer.db, analyzer.file_id, type_ref))
        } else {
            None
        };

        params_result.push((name, type_ref));
    }

    params_result
}
