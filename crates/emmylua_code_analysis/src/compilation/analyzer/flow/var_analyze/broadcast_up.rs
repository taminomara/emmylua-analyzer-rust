use emmylua_parser::{
    BinaryOperator, LuaAst, LuaAstNode, LuaBlock, LuaExpr, LuaLiteralToken, UnaryOperator,
};
use rowan::TextRange;
use smol_str::SmolStr;

use crate::{DbIndex, LuaFlowChain, LuaType, TypeAssertion, VarRefId};

use super::{infer_call_arg_list, is_block_has_return};

pub fn broadcast_up(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    var_ref_id: &VarRefId,
    parent: LuaAst,
    origin: LuaAst,
    type_assert: TypeAssertion,
) -> Option<()> {
    let actual_range = origin.get_range();
    match parent {
        LuaAst::LuaIfStat(if_stat) => {
            if let Some(block) = if_stat.get_block() {
                flow_chain.add_type_assert(
                    var_ref_id,
                    type_assert.clone(),
                    block.get_range(),
                    actual_range,
                );
            }

            // workaround we need virtual execute
            if let Some(ne_type_assert) = type_assert.get_negation() {
                if let Some(else_stat) = if_stat.get_else_clause() {
                    let block_range = else_stat.get_range();
                    flow_chain.add_type_assert(
                        var_ref_id,
                        ne_type_assert.clone(),
                        block_range,
                        actual_range,
                    );
                } else if is_block_has_return(if_stat.get_block()).unwrap_or(false) {
                    let parent_block = if_stat.get_parent::<LuaBlock>()?;
                    let parent_range = parent_block.get_range();
                    let if_range = if_stat.get_range();
                    if if_range.end() < parent_range.end() {
                        let range = TextRange::new(if_range.end(), parent_range.end());
                        flow_chain.add_type_assert(
                            var_ref_id,
                            ne_type_assert.clone(),
                            range,
                            actual_range,
                        );
                    }
                }
                for else_if_clause in if_stat.get_else_if_clause_list() {
                    let block_range = else_if_clause.get_range();
                    flow_chain.add_type_assert(
                        var_ref_id,
                        ne_type_assert.clone(),
                        block_range,
                        actual_range,
                    );
                }
            }
        }
        LuaAst::LuaWhileStat(while_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = while_stat.get_block()?;
            flow_chain.add_type_assert(var_ref_id, type_assert, block.get_range(), actual_range);
        }
        LuaAst::LuaElseIfClauseStat(else_if_clause_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = else_if_clause_stat.get_block()?;
            flow_chain.add_type_assert(var_ref_id, type_assert, block.get_range(), actual_range);
        }
        LuaAst::LuaParenExpr(paren_expr) => {
            broadcast_up(
                db,
                flow_chain,
                var_ref_id,
                paren_expr.get_parent::<LuaAst>()?,
                LuaAst::LuaParenExpr(paren_expr),
                type_assert,
            );
        }
        LuaAst::LuaBinaryExpr(binary_expr) => {
            let op = binary_expr.get_op_token()?;
            match op.get_op() {
                BinaryOperator::OpAnd => {
                    let (left, right) = binary_expr.get_exprs()?;
                    if left.get_position() == origin.get_position() {
                        flow_chain.add_type_assert(
                            var_ref_id,
                            type_assert.clone(),
                            right.get_range(),
                            actual_range,
                        );
                    }

                    broadcast_up(
                        db,
                        flow_chain,
                        var_ref_id,
                        binary_expr.get_parent::<LuaAst>()?,
                        LuaAst::LuaBinaryExpr(binary_expr),
                        type_assert,
                    );
                }
                BinaryOperator::OpOr => {
                    let (left, right) = binary_expr.get_exprs()?;
                    if left.get_position() == origin.get_position() {
                        if let Some(ne) = type_assert.get_negation() {
                            flow_chain.add_type_assert(
                                var_ref_id,
                                ne,
                                right.get_range(),
                                actual_range,
                            );
                        }
                    }
                    broadcast_up(
                        db,
                        flow_chain,
                        var_ref_id,
                        binary_expr.get_parent::<LuaAst>()?,
                        LuaAst::LuaBinaryExpr(binary_expr),
                        type_assert,
                    );
                }
                BinaryOperator::OpEq => {
                    let (left, right) = binary_expr.get_exprs()?;
                    let expr = if left.get_position() == origin.get_position() {
                        right
                    } else {
                        left
                    };

                    if let LuaExpr::LiteralExpr(literal) = expr {
                        let type_assert = match literal.get_literal()? {
                            LuaLiteralToken::Nil(_) => TypeAssertion::NotExist,
                            LuaLiteralToken::Bool(b) => {
                                if b.is_true() {
                                    TypeAssertion::Exist
                                } else {
                                    TypeAssertion::NotExist
                                }
                            }
                            LuaLiteralToken::Number(i) => {
                                if i.is_int() {
                                    TypeAssertion::Narrow(LuaType::IntegerConst(i.get_int_value()))
                                } else {
                                    TypeAssertion::Narrow(LuaType::Number)
                                }
                            }
                            LuaLiteralToken::String(s) => TypeAssertion::Narrow(
                                LuaType::StringConst(SmolStr::new(s.get_value()).into()),
                            ),
                            _ => return None,
                        };

                        broadcast_up(
                            db,
                            flow_chain,
                            var_ref_id,
                            binary_expr.get_parent::<LuaAst>()?,
                            LuaAst::LuaBinaryExpr(binary_expr),
                            type_assert,
                        );
                    }
                }
                BinaryOperator::OpNe => {
                    let (left, right) = binary_expr.get_exprs()?;
                    let expr = if left.get_position() == origin.get_position() {
                        right
                    } else {
                        left
                    };

                    if let LuaExpr::LiteralExpr(literal) = expr {
                        let type_assert = match literal.get_literal()? {
                            LuaLiteralToken::Nil(_) => TypeAssertion::Exist,
                            LuaLiteralToken::Bool(b) => {
                                if b.is_true() {
                                    TypeAssertion::NotExist
                                } else {
                                    TypeAssertion::Exist
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
                            flow_chain,
                            var_ref_id,
                            binary_expr.get_parent::<LuaAst>()?,
                            LuaAst::LuaBinaryExpr(binary_expr),
                            type_assert,
                        );
                    }
                }

                _ => {}
            }
        }
        LuaAst::LuaCallArgList(call_args_list) => {
            infer_call_arg_list(db, flow_chain, type_assert, var_ref_id, call_args_list)?;
        }
        LuaAst::LuaUnaryExpr(unary_expr) => {
            let op = unary_expr.get_op_token()?;
            match op.get_op() {
                UnaryOperator::OpNot => {
                    if let Some(ne_type_assert) = type_assert.get_negation() {
                        broadcast_up(
                            db,
                            flow_chain,
                            var_ref_id,
                            unary_expr.get_parent::<LuaAst>()?,
                            LuaAst::LuaUnaryExpr(unary_expr),
                            ne_type_assert,
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
