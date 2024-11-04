#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaSymbolKind {
    Module,
    Function,
    Variable,
    Parameter,
    TableStruct,
    Field,
    Method,
    Class,
    Enum,
    EnumMember,
    TypeAlias,
    Namespace,
    Property,
    Operator,
    Constant,
}
