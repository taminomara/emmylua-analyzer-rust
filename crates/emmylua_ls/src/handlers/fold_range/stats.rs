use std::fmt::format;

use emmylua_parser::{
    LuaAstNode, LuaDoStat, LuaForRangeStat, LuaForStat, LuaFuncStat, LuaLocalFuncStat,
    LuaRepeatStat, LuaWhileStat,
};
use lsp_types::{FoldingRange, FoldingRangeKind};

use super::builder::FoldingRangeBuilder;

pub fn build_for_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_stat: LuaForStat,
) -> Option<()> {
    let range = for_stat.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("fori .. end".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_for_range_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_range_stat: LuaForRangeStat,
) -> Option<()> {
    let range = for_range_stat.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("for .. end".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_while_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    while_stat: LuaWhileStat,
) -> Option<()> {
    let range = while_stat.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("while .. end".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_repeat_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    repeat_stat: LuaRepeatStat,
) -> Option<()> {
    let range = repeat_stat.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("repeat .. until".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_do_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    do_stat: LuaDoStat,
) -> Option<()> {
    let range = do_stat.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("do .. end".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_local_func_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    local_func: LuaLocalFuncStat,
) -> Option<()> {
    let range = local_func.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let func_name = local_func.get_local_name()?;
    let func_name_text = func_name.syntax().text().to_string();

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(format!("local function {} .. end", func_name_text)),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_func_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    func: LuaFuncStat,
) -> Option<()> {
    let range = func.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let func_name = func.get_func_name()?;
    let func_name_text = func_name.syntax().text().to_string();

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(format!("function {} .. end", func_name_text)),
    };

    builder.push(folding_range);
    Some(())
}
