use emmylua_parser::LuaDocTag;

use super::{
    property_tags::{analyze_deprecated, analyze_nodiscard, analyze_source, analyze_visibility},
    type_def_tags::{analyze_alias, analyze_class, analyze_enum, analyze_func_generic},
    type_ref_tags::{analyze_overload, analyze_param, analyze_return, analyze_type},
    DocAnalyzer,
};

pub fn analyze_tag(analyzer: &mut DocAnalyzer, tag: LuaDocTag) -> Option<()> {
    match tag {
        // def
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

        // ref
        LuaDocTag::Type(type_tag) => {
            analyze_type(analyzer, type_tag)?;
        }
        LuaDocTag::Param(param_tag) => {
            analyze_param(analyzer, param_tag)?;
        }
        LuaDocTag::Return(return_tag) => {
            analyze_return(analyzer, return_tag)?;
        }
        LuaDocTag::Overload(overload_tag) => {
            analyze_overload(analyzer, overload_tag)?;
        }

        // property
        LuaDocTag::Visibility(kind) => {
            analyze_visibility(analyzer, kind)?;
        }
        LuaDocTag::Source(source) => {
            analyze_source(analyzer, source)?;
        }
        LuaDocTag::Nodiscard(_) => {
            analyze_nodiscard(analyzer)?;
        }
        LuaDocTag::Deprecated(deprecated) => {
            analyze_deprecated(analyzer, deprecated)?;
        }

        _ => {}
    }

    Some(())
}
