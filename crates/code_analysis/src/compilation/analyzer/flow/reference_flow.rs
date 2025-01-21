use emmylua_parser::{
    BinaryOperator, LuaAst, LuaAstNode, LuaBinaryExpr, LuaBlock, LuaCallArgList, LuaCallExpr,
    LuaExpr, LuaIndexKey, LuaLiteralToken, LuaNameExpr, LuaStat, LuaSyntaxId, LuaSyntaxKind,
    UnaryOperator,
};
use rowan::TextRange;

use crate::db_index::{LuaFlowChain, LuaMemberKey, LuaType, TypeAssertion};

use super::FlowAnalyzer;

pub fn analyze(analyzer: &mut FlowAnalyzer) -> Option<()> {
    let references_index = analyzer.db.get_reference_index();
    let refs_map = references_index
        .get_local_references_map(&analyzer.file_id)?
        .clone();
    let root = analyzer.root.syntax();
    let file_id = analyzer.file_id;

    for (decl_id, ranges) in refs_map {
        let mut flow_chains = LuaFlowChain::new(decl_id);
        for range in ranges {
            let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), range.clone());
            if let Some(node) = LuaNameExpr::cast(syntax_id.to_node_from_root(root)?) {
                infer_name_expr(analyzer, &mut flow_chains, node);
            }
        }
        analyzer
            .db
            .get_flow_index_mut()
            .add_flow_chain(file_id, flow_chains);
    }

    Some(())
}

fn infer_name_expr(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let parent = name_expr.get_parent::<LuaAst>()?;
    broadcast_up(
        analyzer,
        flow_chains,
        parent,
        LuaAst::LuaNameExpr(name_expr),
        TypeAssertion::Exist,
    );
    Some(())
}

fn broadcast_up(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    parent: LuaAst,
    origin: LuaAst,
    type_assert: TypeAssertion,
) -> Option<()> {
    match parent {
        LuaAst::LuaIfStat(if_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            if let Some(block) = if_stat.get_block() {
                flow_chains.add_type_assert(type_assert.clone(), block.get_range());
            }

            if let Some(ne_type_assert) = type_assert.get_negation() {
                if let Some(else_stat) = if_stat.get_else_clause() {
                    let range = else_stat.get_range();
                    flow_chains.add_type_assert(ne_type_assert, range);
                } else if is_block_has_return(if_stat.get_block()?).unwrap_or(false) {
                    let parent_block = if_stat.get_parent::<LuaBlock>()?;
                    let parent_range = parent_block.get_range();
                    let if_range = if_stat.get_range();
                    if if_range.end() < parent_range.end() {
                        let range = TextRange::new(if_range.end(), parent_range.end());
                        flow_chains.add_type_assert(ne_type_assert, range);
                    }
                }
            }
        }
        LuaAst::LuaWhileStat(while_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = while_stat.get_block()?;
            flow_chains.add_type_assert(type_assert, block.get_range());
        }
        LuaAst::LuaElseIfClauseStat(else_if_clause_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = else_if_clause_stat.get_block()?;
            flow_chains.add_type_assert(type_assert, block.get_range());
        }
        LuaAst::LuaIndexExpr(index_expr) => {
            let key = index_expr.get_index_key()?;
            let reference_key = match key {
                LuaIndexKey::Integer(i) => {
                    if i.is_int() {
                        LuaMemberKey::Integer(i.get_int_value())
                    } else {
                        return None;
                    }
                }
                LuaIndexKey::Name(name) => {
                    LuaMemberKey::Name(name.get_name_text().to_string().into())
                }
                LuaIndexKey::String(string) => LuaMemberKey::Name(string.get_value().into()),
                _ => return None,
            };

            let type_assert = TypeAssertion::FieldExist(reference_key.into());
            broadcast_up(
                analyzer,
                flow_chains,
                index_expr.get_parent::<LuaAst>()?,
                LuaAst::LuaIndexExpr(index_expr),
                type_assert,
            );
        }
        LuaAst::LuaBinaryExpr(binary_expr) => {
            let op = binary_expr.get_op_token()?;
            match op.get_op() {
                BinaryOperator::OpAnd => {
                    let (left, right) = binary_expr.get_exprs()?;
                    if left.get_position() == origin.get_position() {
                        flow_chains.add_type_assert(type_assert.clone(), right.get_range());
                    }

                    broadcast_up(
                        analyzer,
                        flow_chains,
                        binary_expr.get_parent::<LuaAst>()?,
                        LuaAst::LuaBinaryExpr(binary_expr),
                        type_assert,
                    );
                }
                BinaryOperator::OpEq => {
                    if origin.syntax().kind() != LuaSyntaxKind::NameExpr.into() {
                        return None;
                    }

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
                            LuaLiteralToken::Number(_) => {
                                TypeAssertion::Force(LuaType::Number)
                            }
                            LuaLiteralToken::String(_) => {
                                TypeAssertion::Force(LuaType::String)
                            }
                            _ => return None,
                        };

                        broadcast_up(
                            analyzer,
                            flow_chains,
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
            if type_assert == TypeAssertion::Exist {
                infer_call_arg_list(analyzer, flow_chains, call_args_list)?;
            }
        }
        LuaAst::LuaUnaryExpr(unary_expr) => {
            let op = unary_expr.get_op_token()?;
            match op.get_op() {
                UnaryOperator::OpNot => {
                    if let Some(ne_type_assert) = type_assert.get_negation() {
                        broadcast_up(
                            analyzer,
                            flow_chains,
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

fn infer_call_arg_list(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    call_arg: LuaCallArgList,
) -> Option<()> {
    let parent = call_arg.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaCallExpr(call_expr) => {
            if let LuaExpr::NameExpr(prefix_expr) = call_expr.get_prefix_expr()? {
                let name_text = prefix_expr.get_name_text()?;
                if name_text == "type" {
                    infer_lua_type_assert(analyzer, flow_chains, call_expr);
                }
            }
        }
        _ => {}
    }

    Some(())
}

fn infer_lua_type_assert(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let binary_expr = call_expr.get_parent::<LuaBinaryExpr>()?;
    let op = binary_expr.get_op_token()?;
    match op.get_op() {
        BinaryOperator::OpEq => {}
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

    let type_assert = match type_literal.as_str() {
        "number" => TypeAssertion::Force(LuaType::Number),
        "string" => TypeAssertion::Force(LuaType::String),
        "boolean" => TypeAssertion::Force(LuaType::Boolean),
        "table" => TypeAssertion::Force(LuaType::Table),
        "function" => TypeAssertion::Force(LuaType::Function),
        "thread" => TypeAssertion::Force(LuaType::Thread),
        "userdata" => TypeAssertion::Force(LuaType::Userdata),
        "nil" => TypeAssertion::Force(LuaType::Nil),
        _ => {
            return None;
        }
    };

    broadcast_up(
        analyzer,
        flow_chains,
        binary_expr.get_parent::<LuaAst>()?,
        LuaAst::LuaBinaryExpr(binary_expr),
        type_assert,
    );

    Some(())
}

fn is_block_has_return(block: LuaBlock) -> Option<bool> {
    for stat in block.get_stats() {
        match stat {
            LuaStat::ReturnStat(_) => return Some(true),
            LuaStat::DoStat(do_stat) => {
                if is_block_has_return(do_stat.get_block()?).unwrap_or(false) {
                    return Some(true);
                }
            }
            _ => {}
        }
    }

    Some(false)
}
