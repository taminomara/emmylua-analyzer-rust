use super::lua_symbol_kind::LuaSymbolKind;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaSymbol {
    pub name: String,
    pub kind: LuaSymbolKind,
}