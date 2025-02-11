use std::{ops::Deref, sync::Arc};

use crate::{infer_expr, DbIndex, LuaInferConfig};
use emmylua_parser::{LuaAstNode, LuaExpr, LuaSyntaxId, LuaSyntaxNode};

use super::{type_ops::TypeOps, LuaMemberPathExistType, LuaType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeAssertion {
    Exist,
    NotExist,
    Narrow(LuaType),
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
            TypeAssertion::Exist => TypeOps::Remove.apply(&source, &LuaType::Nil),
            TypeAssertion::NotExist => force_nil_or_false(source),
            TypeAssertion::Narrow(t) => TypeOps::Narrow.apply(&source, t),
            TypeAssertion::MemberPathExist(key) => LuaType::MemberPathExist(
                LuaMemberPathExistType::new(&key.deref(), source, 0).into(),
            ),
            TypeAssertion::Add(lua_type) => TypeOps::Union.apply(&source, lua_type),
            TypeAssertion::Remove(lua_type) => TypeOps::Remove.apply(&source, lua_type),
            _ => source,
        }
    }

    pub fn get_negation(&self) -> Option<TypeAssertion> {
        match self {
            TypeAssertion::Exist => Some(TypeAssertion::NotExist),
            TypeAssertion::NotExist => Some(TypeAssertion::Exist),
            TypeAssertion::Narrow(t) => Some(TypeAssertion::Remove(t.clone())),
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
            TypeAssertion::Exist => Some(TypeOps::Remove.apply(&source, &LuaType::Nil)),
            TypeAssertion::NotExist => Some(force_nil_or_false(source)),
            TypeAssertion::Narrow(t) => Some(TypeOps::Narrow.apply(&source, t)),
            TypeAssertion::MemberPathExist(key) => Some(LuaType::MemberPathExist(
                LuaMemberPathExistType::new(&key.deref(), source, 0).into(),
            )),
            TypeAssertion::Add(lua_type) => Some(TypeOps::Union.apply(&source, lua_type)),
            TypeAssertion::Remove(lua_type) => Some(TypeOps::Remove.apply(&source, lua_type)),
            TypeAssertion::Reassign(syntax_id) => {
                let expr = LuaExpr::cast(syntax_id.to_node_from_root(root)?)?;
                let expr_type = infer_expr(db, config, expr)?;
                Some(TypeOps::Narrow.apply(&source, &expr_type))
            }
            TypeAssertion::AddUnion(syntax_ids) => {
                let mut typ = source;
                for syntax_id in syntax_ids {
                    let expr = LuaExpr::cast(syntax_id.to_node_from_root(root)?)?;
                    let expr_type = infer_expr(db, config, expr)?;
                    typ = TypeOps::Union.apply(&typ, &expr_type);
                }
                
                Some(typ)
            }
            _ => Some(source),
        }
    }
}


fn force_nil_or_false(t: LuaType) -> LuaType {
    if t.is_boolean() {
        return LuaType::BooleanConst(false);
    }

    return LuaType::Nil;
}

