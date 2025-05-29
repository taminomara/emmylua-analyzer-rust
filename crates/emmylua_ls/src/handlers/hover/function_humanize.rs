use emmylua_code_analysis::LuaType;

pub fn is_function(typ: &LuaType) -> bool {
    typ.is_function()
        || match &typ {
            LuaType::Union(union) => union
                .get_types()
                .iter()
                .all(|t| matches!(t, LuaType::DocFunction(_) | LuaType::Signature(_))),
            _ => false,
        }
}
