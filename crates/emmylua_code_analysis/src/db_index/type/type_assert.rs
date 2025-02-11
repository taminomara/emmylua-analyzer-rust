use std::{ops::Deref, sync::Arc};

use crate::{infer_expr, DbIndex, LuaInferConfig};
use emmylua_parser::{LuaAstNode, LuaExpr, LuaSyntaxId, LuaSyntaxNode};

use super::{LuaMemberPathExistType, LuaType, LuaUnionType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeAssertion {
    Exist,
    NotExist,
    Force(LuaType),
    MemberPathExist(Arc<String>),
    Add(LuaType),
    Remove(LuaType),
    Reassign(LuaSyntaxId),
    AddUnion(Vec<LuaSyntaxId>),
}

#[allow(unused)]
impl TypeAssertion {
    pub fn simple_tighten_type(&self, source: LuaType) -> LuaType {
        match self {
            TypeAssertion::Exist => remove_nil_and_not_false(source),
            TypeAssertion::NotExist => force_nil_or_false(source),
            TypeAssertion::Force(t) => narrow_down_type(source, t.clone()),
            TypeAssertion::MemberPathExist(key) => LuaType::MemberPathExist(
                LuaMemberPathExistType::new(&key.deref(), source, 0).into(),
            ),
            TypeAssertion::Add(lua_type) => add_type(source, lua_type.clone()),
            TypeAssertion::Remove(lua_type) => remove_type(source, lua_type.clone()),
            _ => source,
        }
    }

    pub fn get_negation(&self) -> Option<TypeAssertion> {
        match self {
            TypeAssertion::Exist => Some(TypeAssertion::NotExist),
            TypeAssertion::NotExist => Some(TypeAssertion::Exist),
            TypeAssertion::Force(t) => Some(TypeAssertion::Remove(t.clone())),
            _ => None,
        }
    }

    pub fn tighten_type(
        &self,
        db: &DbIndex,
        config: &mut LuaInferConfig,
        root: &LuaSyntaxNode,
        source: LuaType,
    ) -> Option<LuaType> {
        match self {
            TypeAssertion::Exist => Some(remove_nil_and_not_false(source)),
            TypeAssertion::NotExist => Some(force_nil_or_false(source)),
            TypeAssertion::Force(t) => Some(narrow_down_type(source, t.clone())),
            TypeAssertion::MemberPathExist(key) => Some(LuaType::MemberPathExist(
                LuaMemberPathExistType::new(&key.deref(), source, 0).into(),
            )),
            TypeAssertion::Add(lua_type) => Some(add_type(source, lua_type.clone())),
            TypeAssertion::Remove(lua_type) => Some(remove_type(source, lua_type.clone())),
            TypeAssertion::Reassign(syntax_id) => {
                let expr = LuaExpr::cast(syntax_id.to_node_from_root(root)?)?;
                let expr_type = infer_expr(db, config, expr)?;
                Some(narrow_down_type(source, expr_type))
            }
            TypeAssertion::AddUnion(syntax_ids) => {
                let mut typ = source;
                for syntax_id in syntax_ids {
                    let expr = LuaExpr::cast(syntax_id.to_node_from_root(root)?)?;
                    let expr_type = infer_expr(db, config, expr)?;
                    typ = add_type(typ, expr_type);
                }
                
                Some(typ)
            }
            _ => Some(source),
        }
    }
}

fn remove_nil_and_not_false(t: LuaType) -> LuaType {
    match t {
        LuaType::Nil => LuaType::Unknown,
        LuaType::Union(types) => {
            let mut new_types = Vec::new();
            for t in types.get_types() {
                let t = remove_nil_and_not_false(t.clone());
                if t != LuaType::Unknown {
                    new_types.push(t);
                }
            }
            if new_types.len() == 1 {
                new_types.pop().unwrap()
            } else {
                LuaType::Union(LuaUnionType::new(new_types).into())
            }
        }
        LuaType::Nullable(t) => remove_nil_and_not_false((*t).clone()),
        t => t,
    }
}

fn force_nil_or_false(t: LuaType) -> LuaType {
    if t.is_boolean() {
        return LuaType::BooleanConst(false);
    }

    return LuaType::Nil;
}

// need to be optimized
fn narrow_down_type(source: LuaType, target: LuaType) -> LuaType {
    match &source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            match target {
                LuaType::Number | LuaType::FloatConst(_) | LuaType::IntegerConst(_) => {
                    types.retain(|t| t.is_number());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::String | LuaType::StringConst(_) => {
                    types.retain(|t| t.is_string());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Boolean | LuaType::BooleanConst(_) => {
                    types.retain(|t| t.is_boolean());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Table | LuaType::TableConst(_) => {
                    types.retain(|t| t.is_table());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Function => {
                    types.retain(|t| t.is_function());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Thread => {
                    types.retain(|t| t.is_thread());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Userdata => {
                    types.retain(|t| t.is_userdata());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                LuaType::Nil => {
                    types.retain(|t| t.is_nil());
                    if types.len() == 1 {
                        types.pop().unwrap()
                    } else {
                        LuaType::Union(LuaUnionType::new(types).into())
                    }
                }
                _ => target,
            }
        }
        LuaType::Nullable(inner) => {
            if !target.is_nullable() {
                narrow_down_type(target, (**inner).clone())
            } else {
                LuaType::Nil
            }
        }
        LuaType::BooleanConst(_) => {
            if target.is_boolean() {
                return LuaType::Boolean;
            }

            target
        }
        LuaType::FloatConst(_) => {
            if target.is_number() {
                return LuaType::Number;
            }

            target
        }
        LuaType::IntegerConst(_) => {
            if target.is_number() {
                return LuaType::Number;
            }

            target
        }
        LuaType::StringConst(_) => {
            if target.is_string() {
                return LuaType::String;
            }

            target
        }
        _ => target,
    }
}

fn add_type(source: LuaType, added_typ: LuaType) -> LuaType {
    if added_typ.is_nil() {
        return LuaType::Nullable(source.into());
    }

    match source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            types.push(added_typ);
            LuaType::Union(LuaUnionType::new(types).into())
        }
        LuaType::Nullable(inner) => {
            let inner = add_type((*inner).clone(), added_typ);
            LuaType::Nullable(inner.into())
        }
        LuaType::Unknown | LuaType::Any => added_typ,
        _ => {
            if source.is_number() && added_typ.is_number() {
                return LuaType::Number;
            } else if source.is_string() && added_typ.is_string() {
                return LuaType::String;
            } else if source.is_boolean() && added_typ.is_boolean() {
                return LuaType::Boolean;
            } else if source.is_table() && added_typ.is_table() {
                return LuaType::Table;
            } 
            
            LuaType::Union(LuaUnionType::new(vec![source, added_typ]).into())
        },
    }
}

fn remove_type(source: LuaType, removed_type: LuaType) -> LuaType {
    if removed_type.is_nil() {
        return remove_nil_and_not_false(source);
    }

    match source {
        LuaType::Union(union) => {
            let mut types = union.get_types().to_vec();
            types.retain(|t| t != &removed_type);
            if types.len() == 1 {
                types.pop().unwrap()
            } else {
                LuaType::Union(LuaUnionType::new(types).into())
            }
        }
        LuaType::Nullable(inner) => {
            let inner = remove_type((*inner).clone(), removed_type);
            LuaType::Nullable(inner.into())
        }
        _ => source,
    }
}
