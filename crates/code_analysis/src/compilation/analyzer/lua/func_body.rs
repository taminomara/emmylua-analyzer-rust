use emmylua_parser::{
    LuaBlock, LuaCallExpr, LuaCallExprStat, LuaDoStat, LuaExpr, LuaForRangeStat, LuaForStat,
    LuaIfStat, LuaNameExpr, LuaRepeatStat, LuaReturnStat, LuaStat, LuaWhileStat,
};

use super::LuaAnalyzer;

#[derive(Debug)]
pub enum LuaReturnPoint {
    Expr(LuaExpr),
    MuliExpr(Vec<LuaExpr>),
    Nil,
    Error,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ChangeFlow {
    None,
    Break,
    Error,
    Return,
}

pub fn analyze_func_body_returns(
    analyzer: &mut LuaAnalyzer,
    body: LuaBlock,
) -> Vec<LuaReturnPoint> {
    let mut returns = Vec::new();

    let flow = analyze_block_returns(analyzer, body, &mut returns);
    match flow {
        Some(ChangeFlow::Break) | Some(ChangeFlow::None) => {
            returns.push(LuaReturnPoint::Nil);
        }
        _ => {}
    }

    returns
}

fn analyze_block_returns(
    analyzer: &mut LuaAnalyzer,
    block: LuaBlock,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    for stat in block.get_stats() {
        match stat {
            LuaStat::DoStat(do_stat) => {
                analyze_do_stat_returns(analyzer, do_stat, returns);
            }
            LuaStat::WhileStat(while_stat) => {
                analyze_while_stat_returns(analyzer, while_stat, returns);
            }
            LuaStat::RepeatStat(repeat_stat) => {
                analyze_repeat_stat_returns(analyzer, repeat_stat, returns);
            }
            LuaStat::IfStat(if_stat) => {
                analyze_if_stat_returns(analyzer, if_stat, returns);
            }
            LuaStat::ForStat(for_stat) => {
                analyze_for_stat_returns(analyzer, for_stat, returns);
            }
            LuaStat::ForRangeStat(for_range_stat) => {
                analyze_for_range_stat_returns(analyzer, for_range_stat, returns);
            }
            LuaStat::CallExprStat(call_expr) => {
                analyze_call_expr_stat_returns(call_expr, returns);
            }
            LuaStat::BreakStat(_) => {
                return Some(ChangeFlow::Break);
            }
            LuaStat::ReturnStat(return_stat) => {
                return analyze_return_stat_returns(analyzer, return_stat, returns);
            }
            _ => {}
        };
    }

    Some(ChangeFlow::None)
}

fn analyze_do_stat_returns(
    analyzer: &mut LuaAnalyzer,
    do_stat: LuaDoStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    analyze_block_returns(analyzer, do_stat.get_block()?, returns)
}

fn analyze_while_stat_returns(
    analyzer: &mut LuaAnalyzer,
    while_stat: LuaWhileStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let flow = analyze_block_returns(analyzer, while_stat.get_block()?, returns);
    match flow {
        Some(ChangeFlow::Break) => Some(ChangeFlow::None),
        _ => flow,
    }
}

fn analyze_repeat_stat_returns(
    analyzer: &mut LuaAnalyzer,
    repeat_stat: LuaRepeatStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let flow = analyze_block_returns(analyzer, repeat_stat.get_block()?, returns);
    match flow {
        Some(ChangeFlow::Break) => Some(ChangeFlow::None),
        _ => flow,
    }
}

fn analyze_for_stat_returns(
    analyzer: &mut LuaAnalyzer,
    for_stat: LuaForStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let flow = analyze_block_returns(analyzer, for_stat.get_block()?, returns);
    match flow {
        Some(ChangeFlow::Break) => Some(ChangeFlow::None),
        _ => flow,
    }
}

// todo
fn analyze_if_stat_returns(
    analyzer: &mut LuaAnalyzer,
    if_stat: LuaIfStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    analyze_block_returns(analyzer, if_stat.get_block()?, returns);
    for clause in if_stat.get_all_clause() {
        analyze_block_returns(analyzer, clause.get_block()?, returns);
    }

    Some(ChangeFlow::None)
}

fn analyze_for_range_stat_returns(
    analyzer: &mut LuaAnalyzer,
    for_range_stat: LuaForRangeStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let flow = analyze_block_returns(analyzer, for_range_stat.get_block()?, returns);
    match flow {
        Some(ChangeFlow::Break) => Some(ChangeFlow::None),
        _ => flow,
    }
}

fn analyze_call_expr_stat_returns(
    call_expr_stat: LuaCallExprStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let prefix_expr = call_expr_stat.get_call_expr()?.get_prefix_expr()?;
    if let LuaExpr::NameExpr(name) = prefix_expr {
        if name.get_name_text()? == "error" {
            returns.push(LuaReturnPoint::Error);
            return Some(ChangeFlow::Error);
        }
    }
    Some(ChangeFlow::None)
}

fn analyze_return_stat_returns(
    analyzer: &mut LuaAnalyzer,
    return_stat: LuaReturnStat,
    returns: &mut Vec<LuaReturnPoint>,
) -> Option<ChangeFlow> {
    let exprs: Vec<LuaExpr> = return_stat.get_expr_list().collect();
    match exprs.len() {
        0 => returns.push(LuaReturnPoint::Nil),
        1 => returns.push(LuaReturnPoint::Expr(exprs[0].clone())),
        _ => returns.push(LuaReturnPoint::MuliExpr(exprs)),
    }

    Some(ChangeFlow::Return)
}
