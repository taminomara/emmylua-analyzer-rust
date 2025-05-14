use std::sync::Arc;

use emmylua_parser::{
    BinaryOperator, LuaAst, LuaAstNode, LuaBinaryExpr, LuaCallArgList, LuaCallExpr,
    LuaCallExprStat, LuaExpr, LuaLiteralToken, UnaryOperator,
};
use smol_str::SmolStr;

use crate::{DbIndex, LuaType, TypeAssertion};

use super::{
    broadcast_down::broadcast_down_after_node, broadcast_inside::broadcast_inside_condition_block,
    unresolve_trace::UnResolveTraceId, var_trace_info::VarTraceInfo, VarTrace,
};

pub fn broadcast_up(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    current: LuaAst,
) -> Option<()> {
    match current {
        LuaAst::LuaIfStat(if_stat) => {
            if let Some(block) = if_stat.get_block() {
                broadcast_inside_condition_block(db, var_trace, trace_info.clone(), block, true);
            }

            // todo
            if !trace_info.check_cover_all_branch() {
                return Some(());
            }

            if let Some(ne_type_assert) = trace_info.type_assertion.get_negation() {
                if let Some(else_stat) = if_stat.get_else_clause() {
                    broadcast_inside_condition_block(
                        db,
                        var_trace,
                        trace_info.with_type_assertion(ne_type_assert.clone()),
                        else_stat.get_block()?,
                        true,
                    );
                }

                for else_if_clause in if_stat.get_else_if_clause_list() {
                    let range = else_if_clause.get_range();
                    var_trace.add_assert(ne_type_assert.clone(), range);
                }
            }
        }
        LuaAst::LuaWhileStat(while_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = while_stat.get_block()?;
            broadcast_inside_condition_block(db, var_trace, trace_info, block, false);
        }
        LuaAst::LuaElseIfClauseStat(else_if_clause_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            if let Some(block) = else_if_clause_stat.get_block() {
                broadcast_inside_condition_block(db, var_trace, trace_info, block, false);
            }
        }
        LuaAst::LuaParenExpr(paren_expr) => {
            broadcast_up(
                db,
                var_trace,
                trace_info,
                paren_expr.get_parent::<LuaAst>()?,
            );
        }
        LuaAst::LuaBinaryExpr(binary_expr) => {
            let op = binary_expr.get_op_token()?;
            match op.get_op() {
                BinaryOperator::OpAnd => {
                    broadcast_up_and(db, var_trace, trace_info, binary_expr.clone());
                }
                BinaryOperator::OpOr => {
                    broadcast_up_or(db, var_trace, trace_info, binary_expr.clone());
                }
                BinaryOperator::OpEq => {
                    if !trace_info.type_assertion.is_exist() {
                        return None;
                    }

                    let (left, right) = binary_expr.get_exprs()?;
                    let expr = if left.get_position() == trace_info.node.get_position() {
                        right
                    } else {
                        left
                    };

                    if let LuaExpr::LiteralExpr(literal) = expr {
                        let type_assert = match literal.get_literal()? {
                            LuaLiteralToken::Nil(_) => TypeAssertion::Force(LuaType::Nil),
                            LuaLiteralToken::Bool(b) => {
                                if b.is_true() {
                                    TypeAssertion::Force(LuaType::BooleanConst(true))
                                } else {
                                    TypeAssertion::Force(LuaType::BooleanConst(false))
                                }
                            }
                            LuaLiteralToken::Number(i) => {
                                if i.is_int() {
                                    TypeAssertion::Force(LuaType::IntegerConst(i.get_int_value()))
                                } else {
                                    TypeAssertion::Force(LuaType::Number)
                                }
                            }
                            LuaLiteralToken::String(s) => TypeAssertion::Force(
                                LuaType::StringConst(SmolStr::new(s.get_value()).into()),
                            ),
                            _ => return None,
                        };

                        broadcast_up(
                            db,
                            var_trace,
                            VarTraceInfo::new(
                                type_assert,
                                LuaAst::cast(binary_expr.syntax().clone())?,
                            )
                            .into(),
                            binary_expr.get_parent::<LuaAst>()?,
                        );
                    }
                }
                BinaryOperator::OpNe => {
                    if !trace_info.type_assertion.is_exist() {
                        return None;
                    }

                    let (left, right) = binary_expr.get_exprs()?;
                    let expr = if left.get_position() == trace_info.node.get_position() {
                        right
                    } else {
                        left
                    };

                    if let LuaExpr::LiteralExpr(literal) = expr {
                        let type_assert = match literal.get_literal()? {
                            LuaLiteralToken::Nil(_) => TypeAssertion::Remove(LuaType::Nil),
                            LuaLiteralToken::Bool(b) => {
                                if b.is_true() {
                                    TypeAssertion::Remove(LuaType::BooleanConst(true))
                                } else {
                                    TypeAssertion::Remove(LuaType::BooleanConst(false))
                                }
                            }
                            LuaLiteralToken::Number(i) => {
                                if i.is_int() {
                                    TypeAssertion::Remove(LuaType::IntegerConst(i.get_int_value()))
                                } else {
                                    TypeAssertion::Remove(LuaType::Number)
                                }
                            }
                            LuaLiteralToken::String(s) => TypeAssertion::Remove(
                                LuaType::StringConst(SmolStr::new(s.get_value()).into()),
                            ),
                            _ => return None,
                        };

                        broadcast_up(
                            db,
                            var_trace,
                            VarTraceInfo::new(
                                type_assert,
                                LuaAst::cast(binary_expr.syntax().clone())?,
                            )
                            .into(),
                            binary_expr.get_parent::<LuaAst>()?,
                        );
                    }
                }

                _ => {}
            }
        }
        LuaAst::LuaCallArgList(call_args_list) => {
            broadcast_up_call_arg_list(db, var_trace, trace_info, call_args_list)?;
        }
        // self:IsXXX()
        LuaAst::LuaIndexExpr(index_expr) => {
            if !trace_info.type_assertion.is_exist() {
                return None;
            }

            let call_expr = index_expr.get_parent::<LuaCallExpr>()?;
            let param_idx = -1;

            broadcast_up(
                db,
                var_trace,
                VarTraceInfo::new(
                    TypeAssertion::Call {
                        id: call_expr.get_syntax_id(),
                        param_idx,
                    },
                    LuaAst::cast(call_expr.syntax().clone())?,
                )
                .into(),
                call_expr.get_parent::<LuaAst>()?,
            );
        }
        LuaAst::LuaUnaryExpr(unary_expr) => {
            let op = unary_expr.get_op_token()?;
            match op.get_op() {
                UnaryOperator::OpNot => {
                    if let Some(ne_type_assert) = trace_info.type_assertion.get_negation() {
                        broadcast_up(
                            db,
                            var_trace,
                            VarTraceInfo::new(
                                ne_type_assert,
                                LuaAst::cast(unary_expr.syntax().clone())?,
                            )
                            .into(),
                            unary_expr.get_parent::<LuaAst>()?,
                        );
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    Some(())
}

pub fn broadcast_up_and(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    binary_expr: LuaBinaryExpr,
) -> Option<()> {
    let (left, right) = binary_expr.get_exprs()?;
    if left.get_range().contains(trace_info.node.get_position()) {
        var_trace.add_assert(trace_info.type_assertion.clone(), right.get_range());

        if var_trace.check_var_use_in_range(right.get_range()) {
            let trace_id = UnResolveTraceId::Expr(LuaExpr::cast(trace_info.node.syntax().clone())?);
            var_trace.add_unresolve_trace(trace_id, trace_info);
            return Some(());
        }
    } else {
        let left_id = UnResolveTraceId::Expr(left);
        if let Some(left_unresolve_trace_info) = var_trace.pop_unresolve_trace(&left_id) {
            let left_trace_info = left_unresolve_trace_info.1.get_trace_info()?;
            let new_assert = left_trace_info
                .type_assertion
                .and_assert(trace_info.type_assertion.clone());

            broadcast_up(
                db,
                var_trace,
                VarTraceInfo::new(new_assert, LuaAst::cast(binary_expr.syntax().clone())?).into(),
                binary_expr.get_parent::<LuaAst>()?,
            );

            return Some(());
        }
    }

    broadcast_up(
        db,
        var_trace,
        VarTraceInfo::new(
            trace_info.type_assertion.clone(),
            LuaAst::cast(binary_expr.syntax().clone())?,
        )
        .into(),
        binary_expr.get_parent::<LuaAst>()?,
    );

    Some(())
}

pub fn broadcast_up_or(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    binary_expr: LuaBinaryExpr,
) -> Option<()> {
    let (left, right) = binary_expr.get_exprs()?;
    if left.get_range().contains(trace_info.node.get_position()) {
        if let Some(ne) = trace_info.type_assertion.get_negation() {
            var_trace.add_assert(ne, right.get_range());
        }

        if var_trace.check_var_use_in_range(right.get_range()) {
            let trace_id = UnResolveTraceId::Expr(LuaExpr::cast(trace_info.node.syntax().clone())?);
            var_trace.add_unresolve_trace(trace_id, trace_info);
            return Some(());
        }
    } else {
        let left_id = UnResolveTraceId::Expr(left);
        if let Some(left_unresolve_trace_info) = var_trace.pop_unresolve_trace(&left_id) {
            let left_trace_info = left_unresolve_trace_info.1.get_trace_info()?;
            let new_assert = left_trace_info
                .type_assertion
                .or_assert(trace_info.type_assertion.clone());
            broadcast_up(
                db,
                var_trace,
                VarTraceInfo::new(new_assert, LuaAst::cast(binary_expr.syntax().clone())?).into(),
                binary_expr.get_parent::<LuaAst>()?,
            );

            return Some(());
        }
    }

    broadcast_up(
        db,
        var_trace,
        VarTraceInfo::new(
            trace_info.type_assertion.clone(),
            LuaAst::cast(binary_expr.syntax().clone())?,
        )
        .into(),
        binary_expr.get_parent::<LuaAst>()?,
    );

    Some(())
}

fn broadcast_up_call_arg_list(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    trace_info: Arc<VarTraceInfo>,
    call_arg: LuaCallArgList,
) -> Option<()> {
    let parent = call_arg.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaCallExpr(call_expr) => {
            if call_expr.is_type() && trace_info.type_assertion.is_exist() {
                broadcast_up_type_assert(db, var_trace, call_expr);
            } else if call_expr.is_assert() {
                broadcast_down_after_node(
                    db,
                    var_trace,
                    trace_info,
                    LuaAst::LuaCallExprStat(call_expr.get_parent::<LuaCallExprStat>()?),
                    true,
                );
            } else if trace_info.type_assertion.is_exist() {
                let current_pos = trace_info.node.get_position();
                let param_idx = call_arg
                    .get_args()
                    .position(|it| it.get_position() == current_pos)?
                    as i32;

                broadcast_up(
                    db,
                    var_trace,
                    VarTraceInfo::new(
                        TypeAssertion::Call {
                            id: call_expr.get_syntax_id(),
                            param_idx,
                        },
                        LuaAst::cast(call_expr.syntax().clone())?,
                    )
                    .into(),
                    call_expr.get_parent::<LuaAst>()?,
                );
            }
        }
        _ => {}
    }

    Some(())
}

fn broadcast_up_type_assert(
    db: &mut DbIndex,
    var_trace: &mut VarTrace,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let binary_expr = call_expr.get_parent::<LuaBinaryExpr>()?;
    let op = binary_expr.get_op_token()?;
    let mut is_eq = true;
    match op.get_op() {
        BinaryOperator::OpEq => {}
        BinaryOperator::OpNe => {
            is_eq = false;
        }
        _ => return None,
    };

    let operands = binary_expr.get_exprs()?;
    let literal_expr = if let LuaExpr::LiteralExpr(literal) = operands.0 {
        literal
    } else if let LuaExpr::LiteralExpr(literal) = operands.1 {
        literal
    } else {
        return None;
    };

    let type_literal = match literal_expr.get_literal()? {
        LuaLiteralToken::String(string) => string.get_value(),
        _ => return None,
    };

    let mut type_assert = match type_literal.as_str() {
        "number" => TypeAssertion::Narrow(LuaType::Number),
        "string" => TypeAssertion::Narrow(LuaType::String),
        "boolean" => TypeAssertion::Narrow(LuaType::Boolean),
        "table" => TypeAssertion::Narrow(LuaType::Table),
        "function" => TypeAssertion::Narrow(LuaType::Function),
        "thread" => TypeAssertion::Narrow(LuaType::Thread),
        "userdata" => TypeAssertion::Narrow(LuaType::Userdata),
        "nil" => TypeAssertion::Narrow(LuaType::Nil),
        _ => return None,
    };

    if !is_eq {
        type_assert = type_assert.get_negation()?;
    }

    broadcast_up(
        db,
        var_trace,
        VarTraceInfo::new(type_assert, LuaAst::cast(binary_expr.syntax().clone())?).into(),
        binary_expr.get_parent::<LuaAst>()?,
    );

    Some(())
}
