use emmylua_parser::LuaDocTag;

use super::{
    type_def_tags::{analyze_alias, analyze_class, analyze_enum, analyze_func_generic},
    DocAnalyzer,
};

pub fn analyze_tag(analyzer: &mut DocAnalyzer, tag: LuaDocTag) -> Option<()> {
    match tag {
        LuaDocTag::Class(class) => {
            analyze_class(analyzer, class)?;
        }
        LuaDocTag::Generic(generic) => {
            analyze_func_generic(analyzer, generic)?;
        }
        LuaDocTag::Enum(enum_tag) => {
            analyze_enum(analyzer, enum_tag)?;
        }
        LuaDocTag::Alias(alias) => {
            analyze_alias(analyzer, alias)?;
        }
        _ => {}
    }

    Some(())
}
