mod infer_binary;
mod infer_call;
mod infer_call_func;
mod infer_index;
mod infer_name;
mod infer_table;
mod infer_unary;
mod resolve_member_type;
mod test;

use std::sync::Arc;

use emmylua_parser::{
    LuaAst, LuaAstNode, LuaClosureExpr, LuaExpr, LuaLiteralExpr, LuaLiteralToken, LuaTableExpr,
};
use infer_binary::infer_binary_expr;
use infer_call::infer_call_expr;
pub use infer_call_func::infer_call_expr_func;
use infer_index::infer_index_expr;
use infer_name::{infer_name_expr, infer_param};
use infer_table::infer_table_expr;
pub use infer_table::infer_table_should_be;
use infer_unary::infer_unary_expr;
use rowan::TextRange;
use smol_str::SmolStr;

use crate::{
    db_index::{DbIndex, LuaOperator, LuaOperatorMetaMethod, LuaSignatureId, LuaType},
    InFiled, LuaMultiReturn,
};

use super::{member::infer_members, CacheEntry, CacheKey, LuaInferCache};

pub type InferResult = Option<LuaType>;

pub fn infer_expr(db: &DbIndex, cache: &mut LuaInferCache, expr: LuaExpr) -> InferResult {
    let syntax_id = expr.get_syntax_id();
    let key = CacheKey::Expr(syntax_id);
    match cache.get(&key) {
        Some(cache) => match cache {
            CacheEntry::ExprCache(ty) => return Some(ty.clone()),
            _ => return Some(LuaType::Unknown),
        },
        None => {}
    }

    // for @as
    let file_id = cache.get_file_id();
    let in_filed_syntax_id = InFiled::new(file_id, syntax_id);
    if let Some(force_type) = db.get_type_index().get_as_force_type(&in_filed_syntax_id) {
        cache.add_cache(&key, CacheEntry::ExprCache(force_type.clone()));
        return Some(force_type.clone());
    }

    cache.ready_cache(&key);
    let result_type = match expr {
        LuaExpr::CallExpr(call_expr) => infer_call_expr(db, cache, call_expr),
        LuaExpr::TableExpr(table_expr) => infer_table_expr(db, cache, table_expr),
        LuaExpr::LiteralExpr(literal_expr) => infer_literal_expr(db, cache, literal_expr),
        LuaExpr::BinaryExpr(binary_expr) => infer_binary_expr(db, cache, binary_expr),
        LuaExpr::UnaryExpr(unary_expr) => infer_unary_expr(db, cache, unary_expr),
        LuaExpr::ClosureExpr(closure_expr) => infer_closure_expr(db, cache, closure_expr),
        LuaExpr::ParenExpr(paren_expr) => infer_expr(db, cache, paren_expr.get_expr()?),
        LuaExpr::NameExpr(name_expr) => infer_name_expr(db, cache, name_expr),
        LuaExpr::IndexExpr(index_expr) => infer_index_expr(db, cache, index_expr),
    };

    if let Some(result_type) = &result_type {
        cache.add_cache(&key, CacheEntry::ExprCache(result_type.clone()));
    } else {
        cache.remove(&key);
    }

    result_type
}

fn infer_literal_expr(db: &DbIndex, config: &LuaInferCache, expr: LuaLiteralExpr) -> InferResult {
    match expr.get_literal()? {
        LuaLiteralToken::Nil(_) => Some(LuaType::Nil),
        LuaLiteralToken::Bool(bool) => Some(LuaType::BooleanConst(bool.is_true())),
        LuaLiteralToken::Number(num) => {
            if num.is_int() {
                Some(LuaType::IntegerConst(num.get_int_value()))
            } else if num.is_float() {
                Some(LuaType::FloatConst(num.get_float_value()))
            } else {
                Some(LuaType::Number)
            }
        }
        LuaLiteralToken::String(str) => {
            Some(LuaType::StringConst(SmolStr::new(str.get_value()).into()))
        }
        LuaLiteralToken::Dots(_) => {
            let file_id = config.get_file_id();
            let range = expr.get_range();

            let decl_id = db
                .get_reference_index()
                .get_local_reference(&file_id)
                .and_then(|file_ref| file_ref.get_decl_id(&range));

            let decl_type = match decl_id.and_then(|id| db.get_decl_index().get_decl(&id)) {
                Some(decl) if decl.is_global() => LuaType::Any,
                Some(decl) if decl.is_param() => {
                    let base = infer_param(db, decl).unwrap_or(LuaType::Unknown);
                    LuaType::MuliReturn(LuaMultiReturn::Base(base).into())
                }
                _ => LuaType::Any, // 默认返回 Any
            };

            Some(decl_type)
        }
        _ => None,
    }
}

fn infer_closure_expr(_: &DbIndex, config: &LuaInferCache, closure: LuaClosureExpr) -> InferResult {
    let signature_id = LuaSignatureId::from_closure(config.get_file_id(), &closure);
    Some(LuaType::Signature(signature_id))
}

fn get_custom_type_operator(
    db: &DbIndex,
    operand_type: LuaType,
    op: LuaOperatorMetaMethod,
) -> Option<Vec<&LuaOperator>> {
    if operand_type.is_custom_type() {
        let type_id = match operand_type {
            LuaType::Ref(type_id) => type_id,
            LuaType::Def(type_id) => type_id,
            _ => return None,
        };
        let ops = db.get_operator_index().get_operators_by_type(&type_id)?;
        let op_ids = ops.get(&op)?;
        let operators = op_ids
            .iter()
            .filter_map(|id| db.get_operator_index().get_operator(id))
            .collect();

        Some(operators)
    } else {
        None
    }
}

/// 获取赋值时所有右值类型或调用时所有参数类型或返回时所有返回值类型
pub fn infer_multi_value_adjusted_expression_types(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    exprs: &[LuaExpr],
) -> Option<Vec<(LuaType, TextRange)>> {
    fn handle_multi_return(
        value_types: &mut Vec<(LuaType, TextRange)>,
        multi: Arc<LuaMultiReturn>,
        is_last: bool,
        range: TextRange,
    ) -> Option<()> {
        match &*multi {
            LuaMultiReturn::Multi(types) => {
                if is_last {
                    // 展开所有类型
                    for (idx, typ) in types.iter().enumerate() {
                        let is_last_in_loop = idx == types.len() - 1;
                        handle_type(value_types, typ, is_last_in_loop, range)?;
                    }
                } else if let Some(first) = types.first() {
                    // 只处理第一个类型
                    handle_type(value_types, first, is_last, range)?;
                }
            }
            LuaMultiReturn::Base(typ) => {
                handle_type(value_types, typ, is_last, range)?;
            }
        }
        Some(())
    }

    fn handle_type(
        value_types: &mut Vec<(LuaType, TextRange)>,
        typ: &LuaType,
        is_last: bool,
        range: TextRange,
    ) -> Option<()> {
        match typ {
            LuaType::MuliReturn(multi) => {
                handle_multi_return(value_types, multi.clone(), is_last, range)?;
            }
            _ => {
                value_types.push((typ.clone(), range));
            }
        }
        Some(())
    }

    let mut value_types = Vec::new();
    // 处理最后一个表达式是多返回值的情况
    for (idx, expr) in exprs.iter().enumerate() {
        let is_last = idx == exprs.len() - 1;
        let expr_type = infer_expr(db, cache, expr.clone())?;
        match expr_type {
            LuaType::MuliReturn(multi) => {
                handle_multi_return(&mut value_types, multi.clone(), is_last, expr.get_range())?;
            }
            _ => {
                handle_type(&mut value_types, &expr_type, is_last, expr.get_range())?;
            }
        }
    }

    Some(value_types)
}

/// 从右值推断左值已绑定的类型
pub fn infer_left_value_type_from_right_value(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    expr: LuaExpr,
) -> Option<LuaType> {
    let ast = expr.syntax().parent().map(LuaAst::cast).flatten()?;

    let typ = match ast {
        LuaAst::LuaAssignStat(assign) => {
            let (vars, exprs) = assign.get_var_and_expr_list();
            let mut typ = None;
            for (idx, assign_expr) in exprs.iter().enumerate() {
                if expr == *assign_expr {
                    let var = vars.get(idx);
                    if let Some(var) = var {
                        typ = Some(infer_expr(db, cache, var.clone().into())?);
                    }
                }
            }
            typ
        }
        LuaAst::LuaTableField(table_field) => {
            let field_key = table_field.get_field_key()?;
            let table_expr = table_field.get_parent::<LuaTableExpr>()?;
            let table_type = infer_table_should_be(db, cache, table_expr.clone())?;
            let member_infos = infer_members(db, &table_type)?;
            let mut typ = None;
            for member_info in member_infos.iter() {
                if member_info.key.to_path() == field_key.get_path_part() {
                    typ = Some(member_info.typ.clone());
                }
            }
            typ
        }
        _ => None,
    };

    typ
}
