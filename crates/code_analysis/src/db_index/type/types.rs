use super::type_decl::LuaTypeDeclId;

#[derive(Debug, Clone, Hash)]
pub struct LuaTypeRef {
    pub value: LuaTypeDeclId,
}