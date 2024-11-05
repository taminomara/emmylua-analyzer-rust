use emmylua_parser::{
    LuaDocBinaryType, LuaDocGenericType, LuaDocType, LuaDocUnaryType, LuaLiteralToken,
    LuaTypeBinaryOperator, LuaTypeUnaryOperator,
};

use crate::{
    db_index::{
        DbIndex, LuaExtendedType, LuaGenericType, LuaIntersectionType, LuaTupleType, LuaType,
        LuaUnionType,
    },
    FileId,
};

pub fn infer_type(db: &mut DbIndex, file_id: FileId, node: LuaDocType) -> LuaType {
    match node {
        LuaDocType::Name(name_type) => {
            if let Some(name) = name_type.get_name_text() {
                return infer_buildin_or_ref_type(db, file_id, &name);
            }
        }
        LuaDocType::Nullable(nullable_type) => {
            if let Some(inner_type) = nullable_type.get_type() {
                let t = infer_type(db, file_id, inner_type);
                if t.is_unknown() {
                    return LuaType::Unknown;
                }

                if let LuaType::Nullable(_) = t {
                    return t;
                } else {
                    return LuaType::Nullable(Box::new(t));
                }
            }
        }
        LuaDocType::Array(array_type) => {
            if let Some(inner_type) = array_type.get_type() {
                let t = infer_type(db, file_id, inner_type);
                if t.is_unknown() {
                    return LuaType::Unknown;
                }
                return LuaType::Array(Box::new(t));
            }
        }
        LuaDocType::Literal(literal) => {
            if let Some(literal_token) = literal.get_literal() {
                match literal_token {
                    LuaLiteralToken::String(str_token) => {
                        return LuaType::StringConst(Box::new(str_token.get_value()))
                    }
                    LuaLiteralToken::Number(number_token) => {
                        if number_token.is_int() {
                            return LuaType::IntegerConst(number_token.get_int_value());
                        } else {
                            return LuaType::Number;
                        }
                    }
                    LuaLiteralToken::Bool(bool_token) => {
                        return LuaType::BooleanConst(bool_token.is_true())
                    }
                    LuaLiteralToken::Nil(_) => return LuaType::Nil,
                }
            }
        }
        LuaDocType::Tuple(tuple_type) => {
            let mut types = Vec::new();
            for type_node in tuple_type.get_types() {
                let t = infer_type(db, file_id, type_node);
                if t.is_unknown() {
                    return LuaType::Unknown;
                }
                types.push(t);
            }
            return LuaType::Tuple(Box::new(LuaTupleType::new(types)));
        }
        LuaDocType::Generic(generic_type) => {
            return infer_generic_type(db, file_id, generic_type);
        }
        LuaDocType::Binary(binary_type) => {
            return infer_binary_type(db, file_id, binary_type);
        }
        LuaDocType::Unary(unary_type) => {
            return infer_unary_type(db, file_id, unary_type);
        }
        _ => {}
    }
    LuaType::Unknown
}

fn infer_buildin_or_ref_type(db: &mut DbIndex, file_id: FileId, name: &str) -> LuaType {
    match name {
        "table" => LuaType::Table,
        "userdata" => LuaType::Userdata,
        "thread" => LuaType::Thread,
        "boolean" | "bool" => LuaType::Boolean,
        "string" => LuaType::String,
        "integer" | "int" => LuaType::Integer,
        "number" => LuaType::Number,
        "io" => LuaType::Io,
        _ => {
            if let Some(name_type_decl) = db.get_type_index().find_type_decl(file_id, name) {
                return LuaType::Ref(name_type_decl.get_id());
            }
            LuaType::Unknown
        }
    }
}

fn infer_generic_type(
    db: &mut DbIndex,
    file_id: FileId,
    generic_type: LuaDocGenericType,
) -> LuaType {
    if let Some(name_type) = generic_type.get_name_type() {
        if let Some(name) = name_type.get_name_text() {
            let id =
                if let Some(name_type_decl) = db.get_type_index().find_type_decl(file_id, &name) {
                    name_type_decl.get_id()
                } else {
                    return LuaType::Unknown;
                };

            let mut generic_params = Vec::new();
            if let Some(generic_decl_list) = generic_type.get_generic_types() {
                for param in generic_decl_list.get_types() {
                    let param_type = infer_type(db, file_id, param);
                    if param_type.is_unknown() {
                        return LuaType::Unknown;
                    }
                    generic_params.push(param_type);
                }
            }

            return LuaType::Generic(Box::new(LuaGenericType::new(id, generic_params)));
        }
    }

    LuaType::Unknown
}

fn infer_binary_type(db: &mut DbIndex, file_id: FileId, binary_type: LuaDocBinaryType) -> LuaType {
    if let Some((left, right)) = binary_type.get_types() {
        let left_type = infer_type(db, file_id, left);
        let right_type = infer_type(db, file_id, right);
        if left_type.is_unknown() || right_type.is_unknown() {
            return LuaType::Unknown;
        }

        if let Some(op) = binary_type.get_op_token() {
            match op.get_op() {
                LuaTypeBinaryOperator::Union => match (left_type, right_type) {
                    (LuaType::Union(left_type_union), LuaType::Union(right_type_union)) => {
                        let mut left_types = (*left_type_union).into_types();
                        let right_types = (*right_type_union).into_types();
                        left_types.extend(right_types);
                        return LuaType::Union(Box::new(LuaUnionType::new(left_types)));
                    }
                    (LuaType::Union(left_type_union), right) => {
                        let mut left_types = (*left_type_union).into_types();
                        left_types.push(right);
                        return LuaType::Union(Box::new(LuaUnionType::new(left_types)));
                    }
                    (left, LuaType::Union(right_type_union)) => {
                        let mut right_types = (*right_type_union).into_types();
                        right_types.push(left);
                        return LuaType::Union(Box::new(LuaUnionType::new(right_types)));
                    }
                    (left, right) => {
                        return LuaType::Union(Box::new(LuaUnionType::new(vec![left, right])));
                    }
                },
                LuaTypeBinaryOperator::Intersection => match (left_type, right_type) {
                    (
                        LuaType::Intersection(left_type_union),
                        LuaType::Intersection(right_type_union),
                    ) => {
                        let mut left_types = (*left_type_union).into_types();
                        let right_types = (*right_type_union).into_types();
                        left_types.extend(right_types);
                        return LuaType::Intersection(Box::new(LuaIntersectionType::new(
                            left_types,
                        )));
                    }
                    (LuaType::Intersection(left_type_union), right) => {
                        let mut left_types = (*left_type_union).into_types();
                        left_types.push(right);
                        return LuaType::Intersection(Box::new(LuaIntersectionType::new(
                            left_types,
                        )));
                    }
                    (left, LuaType::Intersection(right_type_union)) => {
                        let mut right_types = (*right_type_union).into_types();
                        right_types.push(left);
                        return LuaType::Intersection(Box::new(LuaIntersectionType::new(
                            right_types,
                        )));
                    }
                    (left, right) => {
                        return LuaType::Intersection(Box::new(LuaIntersectionType::new(vec![
                            left, right,
                        ])));
                    }
                },
                LuaTypeBinaryOperator::Extends => {
                    return LuaType::Extends(Box::new(LuaExtendedType::new(left_type, right_type)));
                }
                _ => {}
            }
        }
    }

    LuaType::Unknown
}

fn infer_unary_type(db: &mut DbIndex, file_id: FileId, unary_type: LuaDocUnaryType) -> LuaType {
    if let Some(base_type) = unary_type.get_type() {
        let base = infer_type(db, file_id, base_type);
        if base.is_unknown() {
            return LuaType::Unknown;
        }

        if let Some(op) = unary_type.get_op_token() {
            match op.get_op() {
                LuaTypeUnaryOperator::Keyof => {
                    return LuaType::KeyOf(Box::new(base));
                }
                _ => {}
            }
        }
    }
    
    LuaType::Unknown
}
