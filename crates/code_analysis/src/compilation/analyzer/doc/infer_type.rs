use std::sync::Arc;

use emmylua_parser::{
    LuaAst, LuaAstNode, LuaDocBinaryType, LuaDocFuncType, LuaDocGenericType, LuaDocObjectFieldKey,
    LuaDocObjectType, LuaDocStrTplType, LuaDocType, LuaDocUnaryType, LuaLiteralToken,
    LuaTypeBinaryOperator, LuaTypeUnaryOperator, LuaVarExpr,
};
use rowan::TextRange;

use crate::{
    db_index::{
        AnalyzeError, LuaExtendedType, LuaFunctionType, LuaGenericType, LuaIndexAccessKey,
        LuaIntersectionType, LuaObjectType, LuaStringTplType, LuaTupleType, LuaType, LuaUnionType,
    },
    DiagnosticCode, GenericTpl,
};

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
                    return LuaType::Nullable(t.into());
                }
            }
        }
        LuaDocType::Array(array_type) => {
            if let Some(inner_type) = array_type.get_type() {
                let t = infer_type(analyzer, inner_type);
                if t.is_unknown() {
                    return LuaType::Unknown;
                }
                return LuaType::Array(t.into());
            }
        }
        LuaDocType::Literal(literal) => {
            if let Some(literal_token) = literal.get_literal() {
                match literal_token {
                    LuaLiteralToken::String(str_token) => {
                        return LuaType::DocStringConst(str_token.get_value().into())
                    }
                    LuaLiteralToken::Number(number_token) => {
                        if number_token.is_int() {
                            return LuaType::DocIntergerConst(number_token.get_int_value());
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
            return LuaType::Tuple(LuaTupleType::new(types).into());
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
        LuaDocType::StrTpl(str_tpl) => {
            return infer_str_tpl(analyzer, str_tpl);
        }
        _ => {} // LuaDocType::Conditional(lua_doc_conditional_type) => todo!(),
                // LuaDocType::Variadic(lua_doc_variadic_type) => todo!(),
    }
    LuaType::Unknown
}

fn infer_buildin_or_ref_type(analyzer: &mut DocAnalyzer, name: &str, range: TextRange) -> LuaType {
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
        "global" => LuaType::Global,
        _ => {
            if let Some((size, is_func)) = analyzer.generic_index.find_generic(position, name) {
                if is_func {
                    return LuaType::FuncTplRef(Arc::new(GenericTpl::new(
                        size,
                        name.to_string().into(),
                    )));
                } else {
                    return LuaType::TplRef(Arc::new(GenericTpl::new(
                        size,
                        name.to_string().into(),
                    )));
                };
            }

            if let Some(name_type_decl) = analyzer
                .db
                .get_type_index_mut()
                .find_type_decl(analyzer.file_id, name)
            {
                return LuaType::Ref(name_type_decl.get_id());
            }

            analyzer.db.get_diagnostic_index_mut().add_diagnostic(
                analyzer.file_id,
                AnalyzeError::new(
                    DiagnosticCode::TypeNotFound,
                    &t!("Type '%{name}' not found", name = name),
                    range,
                ),
            );

            LuaType::Unknown
        }
    }
}

fn infer_generic_type(analyzer: &mut DocAnalyzer, generic_type: LuaDocGenericType) -> LuaType {
    if let Some(name_type) = generic_type.get_name_type() {
        if let Some(name) = name_type.get_name_text() {
            if let Some(typ) = infer_special_generic_type(analyzer, &name, &generic_type) {
                return typ;
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

            return LuaType::Generic(LuaGenericType::new(id, generic_params).into());
        }
    }

    LuaType::Unknown
}

fn infer_special_generic_type(
    analyzer: &mut DocAnalyzer,
    name: &str,
    generic_type: &LuaDocGenericType,
) -> Option<LuaType> {
    match name {
        "table" => {
            let mut types = Vec::new();
            if let Some(generic_decl_list) = generic_type.get_generic_types() {
                for param in generic_decl_list.get_types() {
                    let param_type = infer_type(analyzer, param);
                    types.push(param_type);
                }
            }
            return Some(LuaType::TableGeneric(types.into()));
        }
        "namespace" => {
            let first_doc_param_type = generic_type.get_generic_types()?.get_types().next()?;
            let first_param = infer_type(analyzer, first_doc_param_type);
            if let LuaType::DocStringConst(ns_str) = first_param {
                return Some(LuaType::Namespace(ns_str));
            }
        }
        _ => {}
    }

    None
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
                        let mut left_types = left_type_union.into_types();
                        let right_types = right_type_union.into_types();
                        left_types.extend(right_types);
                        return LuaType::Union(LuaUnionType::new(left_types).into());
                    }
                    (LuaType::Union(left_type_union), right) => {
                        let mut left_types = (*left_type_union).into_types();
                        left_types.push(right);
                        return LuaType::Union(LuaUnionType::new(left_types).into());
                    }
                    (left, LuaType::Union(right_type_union)) => {
                        let mut right_types = (*right_type_union).into_types();
                        right_types.push(left);
                        return LuaType::Union(LuaUnionType::new(right_types).into());
                    }
                    (left, right) => {
                        return LuaType::Union(LuaUnionType::new(vec![left, right]).into());
                    }
                },
                LuaTypeBinaryOperator::Intersection => match (left_type, right_type) {
                    (
                        LuaType::Intersection(left_type_union),
                        LuaType::Intersection(right_type_union),
                    ) => {
                        let mut left_types = left_type_union.into_types();
                        let right_types = right_type_union.into_types();
                        left_types.extend(right_types);
                        return LuaType::Intersection(LuaIntersectionType::new(left_types).into());
                    }
                    (LuaType::Intersection(left_type_union), right) => {
                        let mut left_types = left_type_union.into_types();
                        left_types.push(right);
                        return LuaType::Intersection(LuaIntersectionType::new(left_types).into());
                    }
                    (left, LuaType::Intersection(right_type_union)) => {
                        let mut right_types = right_type_union.into_types();
                        right_types.push(left);
                        return LuaType::Intersection(LuaIntersectionType::new(right_types).into());
                    }
                    (left, right) => {
                        return LuaType::Intersection(
                            LuaIntersectionType::new(vec![left, right]).into(),
                        );
                    }
                },
                LuaTypeBinaryOperator::Extends => {
                    return LuaType::Extends(LuaExtendedType::new(left_type, right_type).into());
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
                    return LuaType::KeyOf(base.into());
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
    let is_colon = get_colon_define(analyzer).unwrap_or(false);

    LuaType::DocFunction(
        LuaFunctionType::new(is_async, is_colon, params_result, return_types).into(),
    )
}

fn get_colon_define(analyzer: &mut DocAnalyzer) -> Option<bool> {
    let owner = analyzer.comment.get_owner()?;
    match owner {
        LuaAst::LuaFuncStat(func_stat) => {
            let func_name = func_stat.get_func_name()?;
            if let LuaVarExpr::IndexExpr(index_expr) = func_name {
                return Some(index_expr.get_index_token()?.is_colon());
            }
        }
        _ => {}
    }

    None
}

fn infer_object_type(analyzer: &mut DocAnalyzer, object_type: LuaDocObjectType) -> LuaType {
    let mut fields = Vec::new();
    for field in object_type.get_fields() {
        let key = if let Some(field_key) = field.get_field_key() {
            match field_key {
                LuaDocObjectFieldKey::Name(name) => {
                    LuaIndexAccessKey::String(name.get_name_text().to_string().into())
                }
                LuaDocObjectFieldKey::Integer(int) => {
                    LuaIndexAccessKey::Integer(int.get_int_value())
                }
                LuaDocObjectFieldKey::String(str) => {
                    LuaIndexAccessKey::String(str.get_value().to_string().into())
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

    LuaType::Object(LuaObjectType::new(fields).into())
}

fn infer_str_tpl(analyzer: &mut DocAnalyzer, str_tpl: LuaDocStrTplType) -> LuaType {
    let prefix = match str_tpl.get_prefix() {
        Some(prefix) => prefix,
        None => "".to_string(),
    };

    let name = match str_tpl.get_tpl_name() {
        Some(name) => name,
        None => return LuaType::Unknown,
    };

    let tp = infer_buildin_or_ref_type(analyzer, &name, str_tpl.get_range());
    if let LuaType::FuncTplRef(tpl) = tp {
        let str_tpl_type = LuaStringTplType::new(&prefix, &tpl.get_name(), tpl.get_tpl_id());
        return LuaType::StrTplRef(str_tpl_type.into());
    }
    LuaType::Unknown
}
