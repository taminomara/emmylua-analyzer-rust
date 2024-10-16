
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum LuaSyntaxKind {
    None,
    // source
    Source,

    // block
    Block,

    // statements
    EmptyStat,
    LocalStat,
    LocalFuncStat,
    IfStat,
    ElseIfClauseStat,
    ElseClauseStat,
    WhileStat,
    DoStat,
    ForStat,
    ForRangeStat,
    RepeatStat,
    FuncStat,
    LabelStat,
    BreakStat,
    ReturnStat,
    GotoStat,
    ExprStat,
    AssignStat,
    UnknownStat,

    // expressions
    ParenExpr,
    LiteralExpr,
    ClosureExpr,
    UnaryExpr,
    BinaryExpr,
    TableExpr,
    CallExpr,
    IndexExpr,
    NameExpr,

    // other
    LocalName,
    ParamName,
    ParamList,
    CallArgList,
    TableFieldAssign,
    TableFieldValue,
    Attribute,

    // comment
    Comment,

    // doc tag
    DocClass,
    DocEnum,
    DocInterface,
    DocAlias,
    DocField,
    DocEnumField,
    DocType,
    DocParam,
    DocReturn,
    DocGeneric,
    DocSee,
    DocDeprecated,
    DocCast,
    DocOverload,
    DocAsync,
    DocVisibility,
    DocMeta,
    DocOther,
    DocDiagnostic,
    DocVersion,
    DocAs,
    DocNodiscard,
    DocOperator,
    DocModule,
    DocMapping,
    DocNamespace,
    DocUsing,
    DocSource,
    DocReadonly,

    // doc other
    DocDetailField,
    DocContinueOrField,

    // doc Type
    TypeArray,          // baseType []
    TypeUnion,          // aType | bType
    TypeIntersection,   // aType & bType
    TypeKeyOf,          // keyof type
    TypeExtends,        // aType extends bType
    TypeIn,             // aType in bType
    TypeConditional,    // conditionType ? trueType : falseType
    TypeIndexAccess,    // type[keyType]
    TypeMapped,         // { [p in KeyType]+? : ValueType }
    TypeMappedKeys,     // [p in KeyType]?
    TypeFun,            // fun(<paramList>): returnType
    TypeGeneric,        // name<typeList>
    TypeTuple,          // [typeList]
    TypeObject, // { a: aType, b: bType } or { [1]: aType, [2]: bType } or { a: aType, b: bType, [number]: string }
    TypeParen,  // (type)
    TypeLiteral, // "string" or <integer> or true or false
    TypeName,   // name
    TypeVariadic, // ...type
    TypeExpand, // type...
    TypeStringTemplate, // prefixName.`T`
    TypeMatch,  // not support now

    // doc parameter
    TypedParameter,
    GenericParameter,
    GenericDeclareList,
    DiagnosticNameList,
    TypeList,
    DocAttribute,
    DocOpType, // +<type>, -<type>, +?
    // start with '#' or '@'
    Description,

    // [<|>] [<framework>] <version>, <version> can be '5.1', '5.2', '5.3', '5.4', 'JIT', <framework> can be 'openresty'
    Version,
}
