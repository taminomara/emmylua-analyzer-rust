use emmylua_parser::{
    LuaAstNode, LuaDoStat, LuaForRangeStat, LuaForStat, LuaIfStat, LuaRepeatStat, LuaWhileStat,
};
use lsp_types::{FoldingRange, FoldingRangeKind};

use super::{builder::FoldingRangeBuilder, get_block_collapsed_range};

pub fn build_for_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_stat: LuaForStat,
) -> Option<()> {
    let (range, collapsed_text) = if let Some(block) = for_stat.get_block() {
        (get_block_collapsed_range(block), " .. ".to_string())
    } else {
        (for_stat.get_range(), "for .. end".to_string())
    };
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    let start_line = lsp_range.start.line;
    let end_line = lsp_range.end.line;
    if start_line == end_line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line,
        start_character: Some(lsp_range.start.character),
        end_line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(collapsed_text),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_for_range_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_range_stat: LuaForRangeStat,
) -> Option<()> {
    let (range, collapsed_text) = if let Some(block) = for_range_stat.get_block() {
        (get_block_collapsed_range(block), " .. ".to_string())
    } else {
        (for_range_stat.get_range(), "for .. end".to_string())
    };
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    let start_line = lsp_range.start.line;
    let end_line = lsp_range.end.line;

    if start_line == end_line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line,
        start_character: Some(lsp_range.start.character),
        end_line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(collapsed_text),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_while_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    while_stat: LuaWhileStat,
) -> Option<()> {
    let (range, collapsed_text) = if let Some(block) = while_stat.get_block() {
        (get_block_collapsed_range(block), " .. ".to_string())
    } else {
        (while_stat.get_range(), "while .. end".to_string())
    };
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;

    let start_line = lsp_range.start.line;
    let end_line = lsp_range.end.line;

    if start_line == end_line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line,
        start_character: Some(lsp_range.start.character),
        end_line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(collapsed_text),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_repeat_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    repeat_stat: LuaRepeatStat,
) -> Option<()> {
    let (range, collapsed_text) = if let Some(block) = repeat_stat.get_block() {
        (get_block_collapsed_range(block), " .. ".to_string())
    } else {
        (repeat_stat.get_range(), "repeat .. until".to_string())
    };
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    let start_line = lsp_range.start.line;
    let end_line = lsp_range.end.line;

    if start_line == end_line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line,
        start_character: Some(lsp_range.start.character),
        end_line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(collapsed_text),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_do_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    do_stat: LuaDoStat,
) -> Option<()> {
    let (range, collapsed_text) = if let Some(block) = do_stat.get_block() {
        (get_block_collapsed_range(block), " .. ".to_string())
    } else {
        (do_stat.get_range(), "do .. end".to_string())
    };
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    let start_line = lsp_range.start.line;
    let end_line = lsp_range.end.line;

    if start_line == end_line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line,
        start_character: Some(lsp_range.start.character),
        end_line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(collapsed_text),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_if_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    if_stat: LuaIfStat,
) -> Option<()> {
    let mut collapsed_range_text = Vec::new();
    if let Some(block) = if_stat.get_block() {
        let range = get_block_collapsed_range(block);
        collapsed_range_text.push((range, " .. ".to_string()));
    } else {
        let range = if_stat.get_range();
        collapsed_range_text.push((range, "if .. end".to_string()));
    }

    for else_if in if_stat.get_else_if_clause_list() {
        if let Some(block) = else_if.get_block() {
            let range = get_block_collapsed_range(block);
            collapsed_range_text.push((range, " .. ".to_string()));
        } else {
            let range = else_if.get_range();
            collapsed_range_text.push((range, "elseif .. end".to_string()));
        }
    }

    if let Some(else_clause) = if_stat.get_else_clause() {
        if let Some(block) = else_clause.get_block() {
            let range = get_block_collapsed_range(block);
            collapsed_range_text.push((range, " .. ".to_string()));
        } else {
            let range = else_clause.get_range();
            collapsed_range_text.push((range, "else .. end".to_string()));
        }
    }

    for (range, collapsed_text) in collapsed_range_text {
        let lsp_range = builder.get_document().to_lsp_range(range)?;
        let start_line = lsp_range.start.line;
        let end_line = lsp_range.end.line;

        if start_line == end_line {
            continue;
        }

        let folding_range = FoldingRange {
            start_line,
            start_character: Some(lsp_range.start.character),
            end_line,
            end_character: Some(0),
            kind: Some(FoldingRangeKind::Region),
            collapsed_text: Some(collapsed_text),
        };

        builder.push(folding_range);
    }

    Some(())
}
