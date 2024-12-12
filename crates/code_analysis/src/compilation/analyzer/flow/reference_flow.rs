use emmylua_parser::{
    BinaryOperator, LuaAst, LuaAstNode, LuaBinaryExpr, LuaCallArgList, LuaCallExpr, LuaExpr,
    LuaIndexKey, LuaLiteralToken, LuaNameExpr, LuaSyntaxId, LuaSyntaxKind,
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
            let node = LuaNameExpr::cast(syntax_id.to_node_from_root(root)?)?;
            infer_name_expr(analyzer, &mut flow_chains, node);
        }
        analyzer
            .db
            .get_flow_index_mut()
            .add_flow_chain(file_id, flow_chains);
    }

    Some(())
}

fn get_effect_range(check_expr: LuaExpr) -> Option<TextRange> {
    let parent = check_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIfStat(if_stat) => {
            let range = if_stat.get_block()?.get_range();
            Some(range)
        }
        LuaAst::LuaWhileStat(while_stat) => {
            let range = while_stat.get_block()?.get_range();
            Some(range)
        }
        LuaAst::LuaElseIfClauseStat(else_if_clause_stat) => {
            let range = else_if_clause_stat.get_block()?.get_range();
            Some(range)
        }
        LuaAst::LuaParenExpr(paren_expr) => get_effect_range(LuaExpr::ParenExpr(paren_expr)),
        LuaAst::LuaBinaryExpr(binary_expr) => {
            let op = binary_expr.get_op_token()?;
            let basic_range = match op.get_op() {
                BinaryOperator::OpAnd => {
                    let range = binary_expr.get_range();
                    let check_range = check_expr.get_range();
                    if check_range.start() == range.start() {
                        let start = check_range.end();
                        let end = range.end();
                        if start < end {
                            Some(TextRange::new(start, end))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => return None,
            };

            let parent_effect_range = get_effect_range(LuaExpr::BinaryExpr(binary_expr));
            match (basic_range, parent_effect_range) {
                (Some(basic_range), Some(parent_effect_range)) => {
                    let start = basic_range.start().min(parent_effect_range.start());
                    let end = basic_range.end().max(parent_effect_range.end());
                    Some(TextRange::new(start, end))
                }
                (Some(basic_range), None) => Some(basic_range),
                (None, Some(parent_effect_range)) => Some(parent_effect_range),
                _ => None,
            }
        }
        _ => None,
    }
}

fn infer_name_expr(
    analyzer: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let parent = name_expr.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaIfStat(if_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = if_stat.get_block()?;
            flow_chains.add_type_assert(TypeAssertion::Exist, block.get_range());
        }
        LuaAst::LuaWhileStat(while_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = while_stat.get_block()?;
            flow_chains.add_type_assert(TypeAssertion::Exist, block.get_range());
        }
        LuaAst::LuaElseIfClauseStat(else_if_clause_stat) => {
            // this mean the name_expr is a condition and the name_expr is not nil and is not false
            let block = else_if_clause_stat.get_block()?;
            flow_chains.add_type_assert(TypeAssertion::Exist, block.get_range());
        }
        LuaAst::LuaIndexExpr(index_expr) => {
            let key = index_expr.get_index_key()?;
            let range = get_effect_range(LuaExpr::IndexExpr(index_expr))?;
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

            flow_chains.add_type_assert(TypeAssertion::FieldExist(reference_key.into()), range);
        }
        LuaAst::LuaBinaryExpr(binary_expr) => {
            let op = binary_expr.get_op_token()?;
            match op.get_op() {
                BinaryOperator::OpAnd => {
                    let range = get_effect_range(LuaExpr::NameExpr(name_expr))?;
                    flow_chains.add_type_assert(TypeAssertion::Exist, range);
                }
                _ => {}
            }
        }
        LuaAst::LuaCallArgList(call_args_list) => {
            infer_call_arg_list(analyzer, flow_chains, call_args_list)?;
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
    _: &FlowAnalyzer,
    flow_chains: &mut LuaFlowChain,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let parent = call_expr.get_parent::<LuaBinaryExpr>()?;
    let op = parent.get_op_token()?;
    match op.get_op() {
        BinaryOperator::OpEq => {}
        _ => return None,
    };

    let operands = parent.get_exprs()?;
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
        "number" => TypeAssertion::IsNativeLuaType(LuaType::Number),
        "string" => TypeAssertion::IsNativeLuaType(LuaType::String),
        "boolean" => TypeAssertion::IsNativeLuaType(LuaType::Boolean),
        "table" => TypeAssertion::IsNativeLuaType(LuaType::Table),
        "function" => TypeAssertion::IsNativeLuaType(LuaType::Function),
        "thread" => TypeAssertion::IsNativeLuaType(LuaType::Thread),
        "userdata" => TypeAssertion::IsNativeLuaType(LuaType::Userdata),
        "nil" => TypeAssertion::IsNativeLuaType(LuaType::Nil),
        _ => {
            return None;
        }
    };

    let range = get_effect_range(LuaExpr::BinaryExpr(parent))?;
    flow_chains.add_type_assert(type_assert, range);

    Some(())
}
