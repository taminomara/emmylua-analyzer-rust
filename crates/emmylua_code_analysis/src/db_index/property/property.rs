use emmylua_parser::{LuaVersionCondition, VisibilityKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaDeclProperty {
    pub id: LuaPropertyId,
    pub description: Option<Box<String>>,
    pub visibility: Option<VisibilityKind>,
    pub source: Option<Box<String>>,
    pub is_deprecated: bool,
    pub deprecated_message: Option<Box<String>>,
    pub version_conds: Option<Box<Vec<LuaVersionCondition>>>,
    pub see_content: Option<Box<String>>,
    pub other_content: Option<Box<String>>,
}

impl LuaDeclProperty {
    pub fn new(id: LuaPropertyId) -> Self {
        Self {
            id,
            description: None,
            visibility: None,
            source: None,
            is_deprecated: false,
            deprecated_message: None,
            version_conds: None,
            see_content: None,
            other_content: None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct LuaPropertyId {
    id: u32,
}

impl LuaPropertyId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
