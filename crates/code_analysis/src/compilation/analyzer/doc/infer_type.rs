use emmylua_parser::{
    LuaAstNode, LuaDocBinaryType, LuaDocFuncType, LuaDocGenericType, LuaDocObjectFieldKey,
    LuaDocObjectType, LuaDocType, LuaDocUnaryType, LuaLiteralToken, LuaTypeBinaryOperator,
    LuaTypeUnaryOperator,
};
use rowan::TextRange;

use crate::{db_index::{
    AnalyzeError, LuaExtendedType, LuaFunctionType, LuaGenericType, LuaIndexAccessKey,
    LuaIntersectionType, LuaObjectType, LuaTupleType, LuaType, LuaUnionType,
}, DiagnosticCode};

use super::DocAnalyzer;

pub fn infer_type(analyzer: &mut DocAnalyzer, node: LuaDocType) -> LuaType {
    match node {
        LuaDocType::Name(name_type) => {
            if let Some(name) = name_type.get_name_text() {
                return infer_buildin_or_ref_type(analyzer, &name, name_type.get_range());
            }
        }
        LuaDocType::Nullable(nullable_type) => {
            if let Some(inner_type) = nullable_type.get_type() {
                let t = infer_type(analyzer, inner_type);
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
                let t = infer_type(analyzer, inner_type);
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
                let t = infer_type(analyzer, type_node);
                if t.is_unknown() {
                    return LuaType::Unknown;
                }
                types.push(t);
            }
            return LuaType::Tuple(Box::new(LuaTupleType::new(types)));
        }
        LuaDocType::Generic(generic_type) => {
            return infer_generic_type(analyzer, generic_type);
        }
        LuaDocType::Binary(binary_type) => {
            return infer_binary_type(analyzer, binary_type);
        }
        LuaDocType::Unary(unary_type) => {
            return infer_unary_type(analyzer, unary_type);
        }
        LuaDocType::Func(func) => {
            return infer_func_type(analyzer, func);
        }
        LuaDocType::Object(object_type) => {
            return infer_object_type(analyzer, object_type);
        }
        _ => {} // LuaDocType::Conditional(lua_doc_conditional_type) => todo!(),
                // LuaDocType::Variadic(lua_doc_variadic_type) => todo!(),
                // LuaDocType::StrTpl(lua_doc_str_tpl_type) => todo!(),
    }
    LuaType::Unknown
}

fn infer_buildin_or_ref_type(
    analyzer: &mut DocAnalyzer,
    name: &str,
    range: TextRange,
) -> LuaType {
    let position = range.start();
    match name {
        "Unknown" => LuaType::Unknown,
        "nil" | "void" => LuaType::Nil,
        "any" => LuaType::Any,
        "table" => LuaType::Table,
        "userdata" => LuaType::Userdata,
        "thread" => LuaType::Thread,
        "boolean" | "bool" => LuaType::Boolean,
        "string" => LuaType::String,
        "integer" | "int" => LuaType::Integer,
        "number" => LuaType::Number,
        "io" => LuaType::Io,
        "self" => LuaType::SelfInfer,
        _ => {
            if let Some(size) = analyzer.generic_index.find_generic(position, name) {
                return LuaType::TplRef(size);
            }

            if let Some(name_type_decl) = analyzer
                .db
                .get_type_index_mut()
                .find_type_decl(analyzer.file_id, name)
            {
                return LuaType::Ref(name_type_decl.get_id());
            }

            analyzer
                .db
                .get_diagnostic_index_mut()
                .add_diagnostic(analyzer.file_id, AnalyzeError::new(
                    DiagnosticCode::TypeNotFound,
                    format!("Type {} not found", name),
                    range
                ));

            LuaType::Unknown
        }
    }
}

fn infer_generic_type(analyzer: &mut DocAnalyzer, generic_type: LuaDocGenericType) -> LuaType {
    if let Some(name_type) = generic_type.get_name_type() {
        if let Some(name) = name_type.get_name_text() {
            if name == "table" {
                let mut types = Vec::new();
                if let Some(generic_decl_list) = generic_type.get_generic_types() {
                    for param in generic_decl_list.get_types() {
                        let param_type = infer_type(analyzer, param);
                        types.push(param_type);
                    }
                }
                return LuaType::TableGeneric(Box::new(types));
            }

            let id = if let Some(name_type_decl) = analyzer
                .db
                .get_type_index_mut()
                .find_type_decl(analyzer.file_id, &name)
            {
                name_type_decl.get_id()
            } else {
                return LuaType::Unknown;
            };

            let mut generic_params = Vec::new();
            if let Some(generic_decl_list) = generic_type.get_generic_types() {
                for param in generic_decl_list.get_types() {
                    let param_type = infer_type(analyzer, param);
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

fn infer_binary_type(analyzer: &mut DocAnalyzer, binary_type: LuaDocBinaryType) -> LuaType {
    if let Some((left, right)) = binary_type.get_types() {
        let left_type = infer_type(analyzer, left);
        let right_type = infer_type(analyzer, right);
        if left_type.is_unknown() {
            return right_type;
        }
        if right_type.is_unknown() {
            return left_type;
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

fn infer_unary_type(analyzer: &mut DocAnalyzer, unary_type: LuaDocUnaryType) -> LuaType {
    if let Some(base_type) = unary_type.get_type() {
        let base = infer_type(analyzer, base_type);
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

fn infer_func_type(analyzer: &mut DocAnalyzer, func: LuaDocFuncType) -> LuaType {
    let mut params_result = Vec::new();
    for param in func.get_params() {
        let name = if let Some(param) = param.get_name_token() {
            param.get_name_text().to_string()
        } else {
            continue;
        };

        let type_ref = if let Some(type_ref) = param.get_type() {
            Some(infer_type(analyzer, type_ref))
        } else {
            None
        };

        params_result.push((name, type_ref));
    }

    let mut return_types = Vec::new();
    if let Some(return_type_list) = func.get_return_type_list() {
        for type_node in return_type_list.get_types() {
            let t = infer_type(analyzer, type_node);
            return_types.push(t);
        }
    }

    let is_async = func.is_async();
    LuaType::DocFunction(Box::new(LuaFunctionType::new(
        is_async,
        params_result,
        return_types,
    )))
}

fn infer_object_type(analyzer: &mut DocAnalyzer, object_type: LuaDocObjectType) -> LuaType {
    let mut fields = Vec::new();
    for field in object_type.get_fields() {
        let key = if let Some(field_key) = field.get_field_key() {
            match field_key {
                LuaDocObjectFieldKey::Name(name) => {
                    LuaIndexAccessKey::String(name.get_name_text().to_string())
                }
                LuaDocObjectFieldKey::Integer(int) => {
                    LuaIndexAccessKey::Integer(int.get_int_value())
                }
                LuaDocObjectFieldKey::String(str) => {
                    LuaIndexAccessKey::String(str.get_value().to_string())
                }
                LuaDocObjectFieldKey::Type(t) => LuaIndexAccessKey::Type(infer_type(analyzer, t)),
            }
        } else {
            continue;
        };

        let type_ref = if let Some(type_ref) = field.get_type() {
            infer_type(analyzer, type_ref)
        } else {
            LuaType::Unknown
        };

        fields.push((key, type_ref));
    }

    LuaType::Object(Box::new(LuaObjectType::new(fields)))
}
