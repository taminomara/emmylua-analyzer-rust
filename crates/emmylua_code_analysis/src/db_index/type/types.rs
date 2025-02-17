use std::{collections::HashMap, hash::Hash, sync::Arc};

use internment::ArcIntern;
use rowan::TextRange;
use smol_str::SmolStr;

use crate::{
    db_index::{LuaMemberKey, LuaSignatureId},
    InFiled,
};

use super::type_decl::LuaTypeDeclId;

#[derive(Debug, Clone)]
pub enum LuaType {
    Unknown,
    Any,
    Nil,
    Table,
    Userdata,
    Function,
    Thread,
    Boolean,
    String,
    Integer,
    Number,
    Io,
    SelfInfer,
    Global,
    BooleanConst(bool),
    StringConst(ArcIntern<SmolStr>),
    IntegerConst(i64),
    FloatConst(f64),
    TableConst(InFiled<TextRange>),
    Ref(LuaTypeDeclId),
    Def(LuaTypeDeclId),
    Module(ArcIntern<SmolStr>),
    Array(Arc<LuaType>),
    Nullable(Arc<LuaType>),
    Tuple(Arc<LuaTupleType>),
    DocFunction(Arc<LuaFunctionType>),
    Object(Arc<LuaObjectType>),
    Union(Arc<LuaUnionType>),
    Intersection(Arc<LuaIntersectionType>),
    Generic(Arc<LuaGenericType>),
    TableGeneric(Arc<Vec<LuaType>>),
    TplRef(Arc<GenericTpl>),
    StrTplRef(Arc<LuaStringTplType>),
    MuliReturn(Arc<LuaMultiReturn>),
    MemberPathExist(Arc<LuaMemberPathExistType>),
    Signature(LuaSignatureId),
    Instance(Arc<LuaInstanceType>),
    DocStringConst(ArcIntern<SmolStr>),
    DocIntegerConst(i64),
    Namespace(ArcIntern<SmolStr>),
    Variadic(Arc<LuaType>),
    Call(Arc<LuaAliasCallType>)
}

impl PartialEq for LuaType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LuaType::Unknown, LuaType::Unknown) => true,
            (LuaType::Any, LuaType::Any) => true,
            (LuaType::Nil, LuaType::Nil) => true,
            (LuaType::Table, LuaType::Table) => true,
            (LuaType::Userdata, LuaType::Userdata) => true,
            (LuaType::Function, LuaType::Function) => true,
            (LuaType::Thread, LuaType::Thread) => true,
            (LuaType::Boolean, LuaType::Boolean) => true,
            (LuaType::String, LuaType::String) => true,
            (LuaType::Integer, LuaType::Integer) => true,
            (LuaType::Number, LuaType::Number) => true,
            (LuaType::Io, LuaType::Io) => true,
            (LuaType::SelfInfer, LuaType::SelfInfer) => true,
            (LuaType::Global, LuaType::Global) => true,
            (LuaType::BooleanConst(a), LuaType::BooleanConst(b)) => a == b,
            (LuaType::StringConst(a), LuaType::StringConst(b)) => a == b,
            (LuaType::IntegerConst(a), LuaType::IntegerConst(b)) => a == b,
            (LuaType::FloatConst(a), LuaType::FloatConst(b)) => a == b,
            (LuaType::TableConst(a), LuaType::TableConst(b)) => a == b,
            (LuaType::Ref(a), LuaType::Ref(b)) => a == b,
            (LuaType::Def(a), LuaType::Def(b)) => a == b,
            (LuaType::Module(a), LuaType::Module(b)) => a == b,
            (LuaType::Array(a), LuaType::Array(b)) => a == b,
            (LuaType::Call(a), LuaType::Call(b)) => a == b,
            (LuaType::Nullable(a), LuaType::Nullable(b)) => a == b,
            (LuaType::Tuple(a), LuaType::Tuple(b)) => a == b,
            (LuaType::DocFunction(a), LuaType::DocFunction(b)) => a == b,
            (LuaType::Object(a), LuaType::Object(b)) => Arc::ptr_eq(a, b),
            (LuaType::Union(a), LuaType::Union(b)) => Arc::ptr_eq(a, b),
            (LuaType::Intersection(a), LuaType::Intersection(b)) => Arc::ptr_eq(a, b),
            (LuaType::Generic(a), LuaType::Generic(b)) => Arc::ptr_eq(a, b),
            (LuaType::TableGeneric(a), LuaType::TableGeneric(b)) => a == b,
            (LuaType::TplRef(a), LuaType::TplRef(b)) => a == b,
            (LuaType::StrTplRef(a), LuaType::StrTplRef(b)) => Arc::ptr_eq(a, b),
            (LuaType::MuliReturn(a), LuaType::MuliReturn(b)) => Arc::ptr_eq(a, b),
            (LuaType::MemberPathExist(a), LuaType::MemberPathExist(b)) => a == b,
            (LuaType::Signature(a), LuaType::Signature(b)) => a == b,
            (LuaType::Instance(a), LuaType::Instance(b)) => a == b,
            (LuaType::DocStringConst(a), LuaType::DocStringConst(b)) => a == b,
            (LuaType::DocIntegerConst(a), LuaType::DocIntegerConst(b)) => a == b,
            (LuaType::Namespace(a), LuaType::Namespace(b)) => a == b,
            (LuaType::Variadic(a), LuaType::Variadic(b)) => a == b,
            _ => false, // 不同变体之间不相等
        }
    }
}

impl Eq for LuaType {}

impl Hash for LuaType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LuaType::Unknown => 0.hash(state),
            LuaType::Any => 1.hash(state),
            LuaType::Nil => 2.hash(state),
            LuaType::Table => 3.hash(state),
            LuaType::Userdata => 4.hash(state),
            LuaType::Function => 5.hash(state),
            LuaType::Thread => 6.hash(state),
            LuaType::Boolean => 7.hash(state),
            LuaType::String => 8.hash(state),
            LuaType::Integer => 9.hash(state),
            LuaType::Number => 10.hash(state),
            LuaType::Io => 11.hash(state),
            LuaType::SelfInfer => 12.hash(state),
            LuaType::Global => 13.hash(state),
            LuaType::BooleanConst(a) => (14, a).hash(state),
            LuaType::StringConst(a) => (15, a).hash(state),
            LuaType::IntegerConst(a) => (16, a).hash(state),
            LuaType::FloatConst(a) => (17, a.to_bits()).hash(state),
            LuaType::TableConst(a) => (18, a).hash(state),
            LuaType::Ref(a) => (19, a).hash(state),
            LuaType::Def(a) => (20, a).hash(state),
            LuaType::Module(a) => (21, a).hash(state),
            LuaType::Array(a) => (22, a).hash(state),
            LuaType::Call(a) => (23, a).hash(state),
            LuaType::Nullable(a) => (24, a).hash(state),
            LuaType::Tuple(a) => (25, a).hash(state),
            LuaType::DocFunction(a) => (26, a).hash(state),
            LuaType::Object(a) => {
                let ptr = Arc::as_ptr(a);
                (27, ptr).hash(state)
            }
            LuaType::Union(a) => {
                let ptr = Arc::as_ptr(a);
                (28, ptr).hash(state)
            }
            LuaType::Intersection(a) => {
                let ptr = Arc::as_ptr(a);
                (29, ptr).hash(state)
            }
            LuaType::Generic(a) => {
                let ptr = Arc::as_ptr(a);
                (30, ptr).hash(state)
            }
            LuaType::TableGeneric(a) => {
                let ptr = Arc::as_ptr(a);
                (31, ptr).hash(state)
            }
            LuaType::TplRef(a) => (32, a).hash(state),
            LuaType::StrTplRef(a) => (33, a).hash(state),
            LuaType::MuliReturn(a) => {
                let ptr = Arc::as_ptr(a);
                (34, ptr).hash(state)
            }
            LuaType::MemberPathExist(a) => (35, a).hash(state),
            LuaType::Signature(a) => (36, a).hash(state),
            LuaType::Instance(a) => (37, a).hash(state),
            LuaType::DocStringConst(a) => (38, a).hash(state),
            LuaType::DocIntegerConst(a) => (39, a).hash(state),
            LuaType::Namespace(a) => (40, a).hash(state),
            LuaType::Variadic(a) => (42, a).hash(state),
        }
    }
}

#[allow(unused)]
impl LuaType {
    pub fn is_unknown(&self) -> bool {
        matches!(self, LuaType::Unknown)
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, LuaType::Nil)
    }

    pub fn is_table(&self) -> bool {
        matches!(
            self,
            LuaType::Table | LuaType::TableGeneric(_) | LuaType::TableConst(_) | LuaType::Global | LuaType::Tuple(_)
        )
    }

    pub fn is_userdata(&self) -> bool {
        matches!(self, LuaType::Userdata)
    }

    pub fn is_thread(&self) -> bool {
        matches!(self, LuaType::Thread)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, LuaType::BooleanConst(_) | LuaType::Boolean)
    }

    pub fn is_string(&self) -> bool {
        matches!(
            self,
            LuaType::StringConst(_) | LuaType::String | LuaType::DocStringConst(_)
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            LuaType::IntegerConst(_) | LuaType::Integer | LuaType::DocIntegerConst(_)
        )
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self,
            LuaType::Number | LuaType::Integer | LuaType::IntegerConst(_) | LuaType::FloatConst(_)
        )
    }

    pub fn is_io(&self) -> bool {
        matches!(self, LuaType::Io)
    }

    pub fn is_ref(&self) -> bool {
        matches!(self, LuaType::Ref(_))
    }

    pub fn is_def(&self) -> bool {
        matches!(self, LuaType::Def(_))
    }

    pub fn is_custom_type(&self) -> bool {
        matches!(self, LuaType::Ref(_) | LuaType::Def(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, LuaType::Array(_))
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, LuaType::Nullable(_))
    }

    pub fn is_optional(&self) -> bool {
        match self {
            LuaType::Nullable(_) | LuaType::Nil => true,
            LuaType::Union(u) => u.types.iter().any(|t| t.is_optional()),
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, LuaType::Tuple(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(
            self,
            LuaType::DocFunction(_) | LuaType::Function | LuaType::Signature(_)
        )
    }

    pub fn is_signature(&self) -> bool {
        matches!(self, LuaType::Signature(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, LuaType::Object(_))
    }

    pub fn is_union(&self) -> bool {
        matches!(self, LuaType::Union(_))
    }

    pub fn is_intersection(&self) -> bool {
        matches!(self, LuaType::Intersection(_))
    }

    pub fn is_call(&self) -> bool {
        matches!(self, LuaType::Call(_))
    }

    pub fn is_generic(&self) -> bool {
        matches!(self, LuaType::Generic(_) | LuaType::TableGeneric(_))
    }

    pub fn is_table_generic(&self) -> bool {
        matches!(self, LuaType::TableGeneric(_))
    }

    pub fn is_class_tpl(&self) -> bool {
        matches!(self, LuaType::TplRef(_))
    }

    pub fn is_str_tpl_ref(&self) -> bool {
        matches!(self, LuaType::StrTplRef(_))
    }

    pub fn is_tpl(&self) -> bool {
        matches!(self, LuaType::TplRef(_) | LuaType::StrTplRef(_))
    }

    pub fn is_self_infer(&self) -> bool {
        matches!(self, LuaType::SelfInfer)
    }

    pub fn is_any(&self) -> bool {
        matches!(self, LuaType::Any)
    }

    pub fn is_const(&self) -> bool {
        matches!(
            self,
            LuaType::BooleanConst(_)
                | LuaType::StringConst(_)
                | LuaType::IntegerConst(_)
                | LuaType::FloatConst(_)
                | LuaType::TableConst(_)
                | LuaType::DocStringConst(_)
                | LuaType::DocIntegerConst(_)
        )
    }

    pub fn is_module(&self) -> bool {
        matches!(self, LuaType::Module(_))
    }

    pub fn is_multi_return(&self) -> bool {
        matches!(self, LuaType::MuliReturn(_))
    }

    pub fn is_exist_field(&self) -> bool {
        matches!(self, LuaType::MemberPathExist(_))
    }

    pub fn is_global(&self) -> bool {
        matches!(self, LuaType::Global)
    }

    pub fn contain_tpl(&self) -> bool {
        match self {
            LuaType::Array(base) => base.contain_tpl(),
            LuaType::Call(base) => base.contain_tpl(),
            LuaType::Nullable(base) => base.contain_tpl(),
            LuaType::Tuple(base) => base.contain_tpl(),
            LuaType::DocFunction(base) => base.contain_tpl(),
            LuaType::Object(base) => base.contain_tpl(),
            LuaType::Union(base) => base.contain_tpl(),
            LuaType::Intersection(base) => base.contain_tpl(),
            LuaType::Generic(base) => base.contain_tpl(),
            LuaType::MuliReturn(multi) => multi.contain_tpl(),
            LuaType::MemberPathExist(field) => field.contain_tpl(),
            LuaType::TableGeneric(params) => params.iter().any(|p| p.contain_tpl()),
            LuaType::Variadic(inner) => inner.contain_tpl(),
            LuaType::TplRef(_) => true,
            LuaType::StrTplRef(_) => true,
            _ => false,
        }
    }

    pub fn is_namespace(&self) -> bool {
        matches!(self, LuaType::Namespace(_))
    }

    pub fn is_variadic(&self) -> bool {
        matches!(self, LuaType::Variadic(_))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaTupleType {
    types: Vec<LuaType>,
}

impl LuaTupleType {
    pub fn new(types: Vec<LuaType>) -> Self {
        Self { types }
    }

    pub fn get_types(&self) -> &[LuaType] {
        &self.types
    }

    pub fn get_type(&self, idx: usize) -> Option<&LuaType> {
        self.types.get(idx)
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn contain_tpl(&self) -> bool {
        self.types.iter().any(|t| t.contain_tpl())
    }
}

impl From<LuaTupleType> for LuaType {
    fn from(t: LuaTupleType) -> Self {
        LuaType::Tuple(t.into())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaFunctionType {
    is_async: bool,
    is_colon_define: bool,
    params: Vec<(String, Option<LuaType>)>,
    ret: Vec<LuaType>,
}

impl LuaFunctionType {
    pub fn new(
        is_async: bool,
        is_colon_define: bool,
        params: Vec<(String, Option<LuaType>)>,
        ret: Vec<LuaType>,
    ) -> Self {
        Self {
            is_async,
            is_colon_define,
            params,
            ret,
        }
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }

    pub fn is_colon_define(&self) -> bool {
        self.is_colon_define
    }

    pub fn get_params(&self) -> &[(String, Option<LuaType>)] {
        &self.params
    }

    pub fn get_ret(&self) -> &[LuaType] {
        &self.ret
    }

    pub fn contain_tpl(&self) -> bool {
        self.params
            .iter()
            .any(|(_, t)| t.as_ref().map_or(false, |t| t.contain_tpl()))
            || self.ret.iter().any(|t| t.contain_tpl())
    }
}

impl From<LuaFunctionType> for LuaType {
    fn from(t: LuaFunctionType) -> Self {
        LuaType::DocFunction(t.into())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LuaIndexAccessKey {
    Integer(i64),
    String(SmolStr),
    Type(LuaType),
}

#[derive(Debug, Clone)]
pub struct LuaObjectType {
    fields: HashMap<LuaMemberKey, LuaType>,
    index_access: Vec<(LuaType, LuaType)>,
}

impl LuaObjectType {
    pub fn new(object_fields: Vec<(LuaIndexAccessKey, LuaType)>) -> Self {
        let mut fields = HashMap::new();
        let mut index_access = Vec::new();
        for (key, value_type) in object_fields.into_iter() {
            match key {
                LuaIndexAccessKey::Integer(i) => {
                    fields.insert(LuaMemberKey::Integer(i), value_type);
                }
                LuaIndexAccessKey::String(s) => {
                    fields.insert(LuaMemberKey::Name(s.clone()), value_type.clone());
                }
                LuaIndexAccessKey::Type(t) => {
                    index_access.push((t, value_type));
                }
            }
        }

        Self {
            fields,
            index_access,
        }
    }

    pub fn new_with_fields(
        fields: HashMap<LuaMemberKey, LuaType>,
        index_access: Vec<(LuaType, LuaType)>,
    ) -> Self {
        Self {
            fields,
            index_access,
        }
    }

    pub fn get_fields(&self) -> &HashMap<LuaMemberKey, LuaType> {
        &self.fields
    }

    pub fn get_index_access(&self) -> &[(LuaType, LuaType)] {
        &self.index_access
    }

    pub fn get_field(&self, key: &LuaMemberKey) -> Option<&LuaType> {
        self.fields.get(key)
    }

    pub fn contain_tpl(&self) -> bool {
        self.fields.values().any(|t| t.contain_tpl())
            || self
                .index_access
                .iter()
                .any(|(k, v)| k.contain_tpl() || v.contain_tpl())
    }
}

impl From<LuaObjectType> for LuaType {
    fn from(t: LuaObjectType) -> Self {
        LuaType::Object(t.into())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaUnionType {
    types: Vec<LuaType>,
}

impl LuaUnionType {
    pub fn new(types: Vec<LuaType>) -> Self {
        Self { types }
    }

    pub fn get_types(&self) -> &[LuaType] {
        &self.types
    }

    pub(crate) fn into_types(&self) -> Vec<LuaType> {
        self.types.clone()
    }

    pub fn contain_tpl(&self) -> bool {
        self.types.iter().any(|t| t.contain_tpl())
    }
}

impl From<LuaUnionType> for LuaType {
    fn from(t: LuaUnionType) -> Self {
        LuaType::Union(t.into())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaIntersectionType {
    types: Vec<LuaType>,
}

impl LuaIntersectionType {
    pub fn new(types: Vec<LuaType>) -> Self {
        Self { types }
    }

    pub fn get_types(&self) -> &[LuaType] {
        &self.types
    }

    pub(crate) fn into_types(&self) -> Vec<LuaType> {
        self.types.clone()
    }

    pub fn contain_tpl(&self) -> bool {
        self.types.iter().any(|t| t.contain_tpl())
    }
}

impl From<LuaIntersectionType> for LuaType {
    fn from(t: LuaIntersectionType) -> Self {
        LuaType::Intersection(t.into())
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LuaAliasCallKind {
    KeyOf,
    Index,
    Extends,
    Add,
    Sub
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaAliasCallType {
    source: LuaType,
    call_kind: LuaAliasCallKind,
    operand: Option<LuaType>,
}

impl LuaAliasCallType {
    pub fn new(source: LuaType, call_kind: LuaAliasCallKind, operand: Option<LuaType>) -> Self {
        Self {
            source,
            call_kind,
            operand,
        }
    }

    pub fn get_source(&self) -> &LuaType {
        &self.source
    }

    pub fn get_operand(&self) -> Option<&LuaType> {
        self.operand.as_ref()
    }

    pub fn get_call_kind(&self) -> LuaAliasCallKind {
        self.call_kind
    }

    pub fn contain_tpl(&self) -> bool {
        self.source.contain_tpl() || self.operand.as_ref().map_or(false, |o| o.contain_tpl())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaGenericType {
    base: LuaTypeDeclId,
    params: Vec<LuaType>,
}

impl LuaGenericType {
    pub fn new(base: LuaTypeDeclId, params: Vec<LuaType>) -> Self {
        Self { base, params }
    }

    pub fn get_base_type(&self) -> LuaType {
        LuaType::Ref(self.base.clone())
    }

    pub fn get_base_type_id(&self) -> LuaTypeDeclId {
        self.base.clone()
    }

    pub fn get_params(&self) -> &Vec<LuaType> {
        &self.params
    }

    pub fn contain_tpl(&self) -> bool {
        self.params.iter().any(|t| t.contain_tpl())
    }
}

impl From<LuaGenericType> for LuaType {
    fn from(t: LuaGenericType) -> Self {
        LuaType::Generic(t.into())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LuaMultiReturn {
    Multi(Vec<LuaType>),
    Base(LuaType),
}

impl LuaMultiReturn {
    pub fn get_type(&self, idx: usize) -> Option<&LuaType> {
        match self {
            LuaMultiReturn::Multi(types) => types.get(idx),
            LuaMultiReturn::Base(t) => Some(t),
        }
    }

    pub fn get_len(&self) -> usize {
        match self {
            LuaMultiReturn::Multi(types) => types.len(),
            LuaMultiReturn::Base(_) => 1,
        }
    }

    pub fn contain_tpl(&self) -> bool {
        match self {
            LuaMultiReturn::Multi(types) => types.iter().any(|t| t.contain_tpl()),
            LuaMultiReturn::Base(t) => t.contain_tpl(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaMemberPathExistType {
    member_path: SmolStr,
    origin: LuaType,
    current_path_idx: usize,
}

impl LuaMemberPathExistType {
    pub fn new(member_path: &str, origin: LuaType, current_path_idx: usize) -> Self {
        Self {
            member_path: SmolStr::from(member_path),
            origin,
            current_path_idx,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.member_path
    }

    pub fn get_origin(&self) -> &LuaType {
        &self.origin
    }

    pub fn get_current_path_idx(&self) -> usize {
        self.current_path_idx
    }

    pub fn get_current_path(&self) -> &str {
        let paths: Vec<&str> = self.member_path.split('.').collect();
        if paths.len() > self.current_path_idx {
            paths[self.current_path_idx]
        } else {
            ""
        }
    }

    pub fn is_final_path(&self) -> bool {
        let paths: Vec<&str> = self.member_path.split('.').collect();
        paths.len() == self.current_path_idx + 1
    }

    pub fn contain_tpl(&self) -> bool {
        self.origin.contain_tpl()
    }
}

impl From<SmolStr> for LuaType {
    fn from(s: SmolStr) -> Self {
        let str: &str = s.as_ref();
        match str {
            "nil" => LuaType::Nil,
            "table" => LuaType::Table,
            "userdata" => LuaType::Userdata,
            "function" => LuaType::Function,
            "thread" => LuaType::Thread,
            "boolean" => LuaType::Boolean,
            "string" => LuaType::String,
            "integer" => LuaType::Integer,
            "number" => LuaType::Number,
            "io" => LuaType::Io,
            "global" => LuaType::Global,
            "self" => LuaType::SelfInfer,
            _ => LuaType::Ref(LuaTypeDeclId::new_by_id(s.into())),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaInstanceType {
    base: LuaType,
    range: InFiled<TextRange>,
}

impl LuaInstanceType {
    pub fn new(base: LuaType, range: InFiled<TextRange>) -> Self {
        Self { base, range }
    }

    pub fn get_base(&self) -> &LuaType {
        &self.base
    }

    pub fn get_range(&self) -> &InFiled<TextRange> {
        &self.range
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GenericTplId {
    Type(u32),
    Func(u32),
}

impl GenericTplId {
    pub fn get_idx(&self) -> usize {
        match self {
            GenericTplId::Type(idx) => *idx as usize,
            GenericTplId::Func(idx) => *idx as usize,
        }
    }

    pub fn is_func(&self) -> bool {
        matches!(self, GenericTplId::Func(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, GenericTplId::Type(_))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GenericTpl {
    tpl_id: GenericTplId,
    name: ArcIntern<SmolStr>,
}

impl GenericTpl {
    pub fn new(tpl_id: GenericTplId, name: ArcIntern<SmolStr>) -> Self {
        Self { tpl_id, name }
    }

    pub fn get_tpl_id(&self) -> GenericTplId {
        self.tpl_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LuaStringTplType {
    prefix: ArcIntern<String>,
    tpl_id: GenericTplId,
    name: ArcIntern<String>,
}

impl LuaStringTplType {
    pub fn new(prefix: &str, name: &str, tpl_id: GenericTplId) -> Self {
        Self {
            prefix: ArcIntern::new(prefix.to_string()),
            tpl_id,
            name: ArcIntern::new(name.to_string()),
        }
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub fn get_tpl_id(&self) -> GenericTplId {
        self.tpl_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
