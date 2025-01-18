mod func_type;
mod sub_type;

use std::ops::Deref;

use func_type::infer_doc_func_type_compact;

use crate::{
    db_index::{
        DbIndex, LuaGenericType, LuaMemberKey, LuaMemberOwner, LuaObjectType, LuaTupleType,
        LuaType, LuaTypeDeclId,
    },
    LuaUnionType,
};

use super::{InferGuard, LuaInferConfig};
pub use sub_type::is_sub_type_of;

pub fn check_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source: &LuaType,
    compact_type: &LuaType,
) -> bool {
    return infer_type_compact(db, config, source, compact_type, &mut InferGuard::new());
}

fn infer_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source: &LuaType,
    compact_type: &LuaType,
    infer_guard: &mut InferGuard,
) -> bool {
    if compact_type.is_any() || compact_type.is_tpl() {
        return true;
    }

    let compact_type = if let LuaType::Ref(type_id) = compact_type {
        if let Some(escaped) = escape_alias(db, &type_id) {
            escaped
        } else {
            compact_type.clone()
        }
    } else {
        compact_type.clone()
    };

    if source == &compact_type {
        return true;
    }

    match (source, &compact_type) {
        // basic type
        (LuaType::Any, _) => true,
        (LuaType::SelfInfer, _) => true,
        (LuaType::Unknown, _) => true,
        (_, LuaType::Instance(right)) => {
            infer_type_compact(db, config, source, &right.get_base(), infer_guard)
        }
        (_, LuaType::ExistField(right)) => {
            infer_type_compact(db, config, source, &right.get_origin(), infer_guard)
        }
        (LuaType::BooleanConst(_), _) => compact_type.is_boolean(),
        (LuaType::IntegerConst(_), _) => compact_type.is_number(),
        (LuaType::StringConst(_), _) => compact_type.is_string(),
        (LuaType::Number, _) => compact_type.is_number(),
        (LuaType::Integer, _) => compact_type.is_integer(),
        (LuaType::String, _) => compact_type.is_string(),
        (LuaType::Boolean, _) => compact_type.is_boolean(),
        (LuaType::DocIntegerConst(i), _) => match compact_type {
            LuaType::IntegerConst(j) => *i == j,
            LuaType::DocIntegerConst(j) => *i == j,
            _ => false,
        },
        (LuaType::DocStringConst(s), _) => match compact_type {
            LuaType::StringConst(t) => *s == t,
            LuaType::DocStringConst(t) => *s == t,
            _ => false,
        },
        (LuaType::FloatConst(_), _) => compact_type.is_number(),
        (LuaType::Table, _) => {
            compact_type.is_table()
                || compact_type.is_array()
                || compact_type.is_def()
                || compact_type.is_ref()
        }
        (LuaType::TableGeneric(_), _) => {
            compact_type.is_table()
                || compact_type.is_array()
                || compact_type.is_def()
                || compact_type.is_ref()
                || compact_type.is_userdata()
        }
        (LuaType::Userdata, _) => {
            compact_type.is_userdata() || compact_type.is_def() || compact_type.is_ref()
        }
        (LuaType::Io, LuaType::Io) => true,
        (LuaType::Thread, LuaType::Thread) => true,
        (LuaType::Function, _) => compact_type.is_function(),
        // custom type
        // any custom type will merge from expr
        (LuaType::Def(_), _) => true,
        (LuaType::Ref(type_id), _) => {
            infer_custom_type_compact(db, config, type_id, &compact_type, infer_guard)
                .unwrap_or(false)
        }
        // complex type comb
        (LuaType::Object(object), _) => {
            infer_object_type_compact(db, config, object, &compact_type, infer_guard)
                .unwrap_or(false)
        }
        (LuaType::Array(a), LuaType::Array(b)) => infer_type_compact(db, config, a, b, infer_guard),
        // TODO implement the check for table
        (LuaType::Array(_), _) => compact_type.is_table(),
        (LuaType::Tuple(a), LuaType::Tuple(b)) => {
            infer_tuple_type_compact(db, config, a, b, infer_guard).unwrap_or(false)
        }
        // TODO implement the check for table
        (LuaType::Tuple(_), _) => compact_type.is_table(),
        (LuaType::Union(a), LuaType::Union(b)) => {
            infer_union_union_type_compact(db, config, a, b, infer_guard).unwrap_or(false)
        }
        (LuaType::Union(a), _) => a
            .get_types()
            .iter()
            .any(|t| infer_type_compact(db, config, t, &compact_type, infer_guard)),
        (LuaType::Intersection(a), _) => a
            .get_types()
            .iter()
            .all(|t| infer_type_compact(db, config, t, &compact_type, infer_guard)),
        (LuaType::DocFunction(f), _) => {
            infer_doc_func_type_compact(db, config, f, &compact_type, infer_guard)
        }
        (LuaType::Nullable(base), _) => {
            compact_type.is_nil()
                || infer_type_compact(db, config, base, &compact_type, infer_guard)
        }
        (LuaType::Generic(source_generic), LuaType::Generic(compact_generic)) => {
            infer_generic_type_compact(db, config, source_generic, compact_generic, infer_guard)
                .unwrap_or(false)
        }
        // template
        (LuaType::TplRef(_), _) => true,
        (LuaType::StrTplRef(_), _) => match compact_type {
            LuaType::StringConst(_) => true,
            LuaType::DocStringConst(_) => true,
            _ => false,
        },
        (LuaType::FuncTplRef(_), _) => true,
        // trivia
        (LuaType::Module(_), _) => false,
        (LuaType::Signature(_), _) => false,
        (LuaType::TableConst(_), _) => false,
        (LuaType::Extends(_), _) => false,
        (LuaType::MuliReturn(_), _) => false,
        (LuaType::Namespace(a), LuaType::Namespace(b)) => a == b,
        _ => false,
    }
}

fn infer_custom_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    type_id: &LuaTypeDeclId,
    compact_type: &LuaType,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    infer_guard.check(type_id)?;
    let type_decl = db.get_type_index().get_type_decl(&type_id.clone())?;
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin() {
            return Some(infer_type_compact(
                db,
                config,
                origin_type,
                compact_type,
                infer_guard,
            ));
        }
        if let Some(members) = type_decl.get_alias_union_members() {
            for member_id in members {
                let member = db.get_member_index().get_member(member_id)?;
                let member_type = member.get_decl_type();
                if infer_type_compact(db, config, member_type, compact_type, infer_guard) {
                    return Some(true);
                }
            }
        }
    } else if type_decl.is_enum() {
        let const_value = match compact_type {
            LuaType::Def(compact_id) => {
                if type_id == compact_id {
                    return Some(true);
                }

                return None;
            }
            LuaType::Ref(compact_id) => {
                if type_id == compact_id {
                    return Some(true);
                }

                return None;
            }
            LuaType::StringConst(s) => LuaMemberKey::Name(s.deref().clone()),
            LuaType::IntegerConst(i) => LuaMemberKey::Integer(*i),
            _ => return None,
        };

        let member_map = db
            .get_member_index()
            .get_member_map(LuaMemberOwner::Type(type_id.clone()))?;

        if type_decl.is_enum_key() {
            if member_map.contains_key(&const_value) {
                return Some(true);
            }
        } else {
            let compact_type = match const_value {
                LuaMemberKey::Name(name) => LuaType::StringConst(name.into()),
                LuaMemberKey::Integer(i) => LuaType::IntegerConst(i),
                LuaMemberKey::None => return None,
            };

            for (_, member_id) in member_map {
                let member = db.get_member_index().get_member(member_id)?;
                let member_type = member.get_decl_type();

                if member_type == &compact_type {
                    return Some(true);
                }
            }
        }
    } else {
        // check same id
        let compact_id = match compact_type {
            LuaType::Def(compact_id) => {
                if type_id == compact_id {
                    return Some(true);
                }

                compact_id
            }
            LuaType::Ref(compact_id) => {
                if type_id == compact_id {
                    return Some(true);
                }

                compact_id
            }
            LuaType::TableConst(range) => {
                let table_member_owner = LuaMemberOwner::Element(range.clone());
                return infer_custom_type_compact_table(
                    db,
                    config,
                    type_id,
                    table_member_owner,
                    infer_guard,
                );
            }
            _ => return Some(false),
        };

        let supers = db.get_type_index().get_super_types(compact_id)?;
        for compact_super in supers {
            if infer_custom_type_compact(db, config, type_id, &compact_super, infer_guard)? {
                return Some(true);
            }
        }
    }

    Some(false)
}

fn infer_custom_type_compact_table(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    type_id: &LuaTypeDeclId,
    table_owner: LuaMemberOwner,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    let member_index = db.get_member_index();
    let members = member_index.get_member_map(table_owner.clone())?;
    let type_member_owner = LuaMemberOwner::Type(type_id.clone());
    let type_members = member_index.get_member_map(type_member_owner)?;
    for (key, type_member_id) in type_members {
        let table_member_id = members.get(key)?;
        let table_member = member_index.get_member(table_member_id)?;
        let type_member = member_index.get_member(type_member_id)?;
        let type_member_type = type_member.get_decl_type();
        let table_member_type = table_member.get_decl_type();
        if !infer_type_compact(db, config, type_member_type, table_member_type, infer_guard) {
            return Some(false);
        }
    }

    let supers = db.get_type_index().get_super_types(type_id);
    if let Some(supers) = supers {
        for super_type in supers {
            let table_type = LuaType::TableConst(table_owner.get_element_range()?.clone());
            if !infer_type_compact(db, config, &super_type, &table_type, infer_guard) {
                return Some(false);
            }
        }
    }

    Some(true)
}

fn escape_alias(db: &DbIndex, type_id: &LuaTypeDeclId) -> Option<LuaType> {
    let type_decl = db.get_type_index().get_type_decl(type_id)?;
    if type_decl.is_alias() {
        if let Some(origin_type) = type_decl.get_alias_origin() {
            return Some(origin_type.clone());
        }
    }

    None
}

fn infer_object_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source: &LuaObjectType,
    compact_type: &LuaType,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    match compact_type {
        LuaType::TableConst(range) => {
            let table_owner = LuaMemberOwner::Element(range.clone());
            let member_index = db.get_member_index();
            let members = member_index.get_member_map(table_owner.clone())?;
            let fields = source.get_fields();
            for (key, source_type) in fields {
                let table_member_id = members.get(key)?;
                let table_member = member_index.get_member(table_member_id)?;
                let table_member_type = table_member.get_decl_type();
                if !infer_type_compact(db, config, source_type, table_member_type, infer_guard) {
                    return Some(false);
                }
            }
            Some(true)
        }
        // TODO: implement the rest of the cases
        // LuaType::Def(type_id) => {

        // }
        // LuaType::Ref(type_id) => {

        // }
        _ => Some(false),
    }
}

fn infer_tuple_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source: &LuaTupleType,
    compact_type: &LuaTupleType,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    let source_types = source.get_types();
    let target_types = compact_type.get_types();
    if source_types.len() > target_types.len() {
        return Some(false);
    }

    let source_types_len = source_types.len();
    for i in 0..source_types_len {
        let source_type = &source_types[i];
        let target_type = &target_types[i];
        if !infer_type_compact(db, config, source_type, target_type, infer_guard) {
            return Some(false);
        }
    }

    Some(true)
}

fn infer_generic_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source_generic: &LuaGenericType,
    compact_generic: &LuaGenericType,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    let source_base_id = source_generic.get_base_type_id();
    infer_guard.check(&source_base_id)?;

    let compact_base_id = compact_generic.get_base_type_id();
    if source_base_id != compact_base_id {
        return Some(false);
    }

    let source_params = source_generic.get_params();
    let compact_params = compact_generic.get_params();
    if source_params.len() != compact_params.len() {
        return Some(false);
    }

    for i in 0..source_params.len() {
        let source_param = &source_params[i];
        let compact_param = &compact_params[i];
        if !infer_type_compact(
            db,
            config,
            source_param,
            compact_param,
            &mut InferGuard::new(),
        ) {
            return Some(false);
        }
    }

    Some(true)
}

// a diffcult compare
fn infer_union_union_type_compact(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    source: &LuaUnionType,
    compact_type: &LuaUnionType,
    infer_guard: &mut InferGuard,
) -> Option<bool> {
    let a_types = source.get_types();
    let b_types = compact_type.get_types();

    for a_type in a_types {
        if !b_types
            .iter()
            .any(|b_type| infer_type_compact(db, config, a_type, b_type, infer_guard))
        {
            return Some(false);
        }
    }

    for b_type in b_types {
        if !a_types
            .iter()
            .any(|a_type| infer_type_compact(db, config, a_type, b_type, infer_guard))
        {
            return Some(false);
        }
    }

    Some(false)
}
