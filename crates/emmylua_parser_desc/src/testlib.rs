use crate::util::sort_result;
use crate::{DescItem, LuaDescParser};
use emmylua_parser::{
    LuaAstNode, LuaDocDescription, LuaKind, LuaParser, LuaSyntaxKind, ParserConfig,
};

pub fn test(code: &str, mut parser: Box<dyn LuaDescParser>, expected: &str) {
    let tree = LuaParser::parse(code, ParserConfig::default());
    let Some(desc) = tree
        .get_red_root()
        .descendants()
        .filter(|node| matches!(node.kind(), LuaKind::Syntax(LuaSyntaxKind::DocDescription)))
        .next()
    else {
        panic!("No desc found in {:?}", tree.get_red_root());
    };
    let ranges = parser.parse(code, LuaDocDescription::cast(desc).unwrap());
    let result = format_result(code, ranges);

    let result_trimmed = result.trim();
    let expected_trimmed = expected.trim();

    if result_trimmed != expected_trimmed {
        // Split strings by lines
        let result_lines: Vec<&str> = result_trimmed.lines().collect();
        let expected_lines: Vec<&str> = expected_trimmed.lines().collect();

        println!("Strings do not match! Detailed comparison:");
        println!(
            "Actual result has {} lines, expected result has {} lines",
            result_lines.len(),
            expected_lines.len()
        );
        println!();

        let max_lines = result_lines.len().max(expected_lines.len());
        for i in 0..max_lines {
            let actual_line = result_lines.get(i).unwrap_or(&"<line does not exist>");
            let expected_line = expected_lines.get(i).unwrap_or(&"<line does not exist>");

            if actual_line != expected_line {
                println!("Line {} does not match:", i + 1);
                println!("  Actual:   {:?}", actual_line);
                println!("  Expected: {:?}", expected_line);
                println!();
            }
        }

        panic!("Test failed: actual result does not match expected result");
    }
}

#[allow(unused)]
pub fn print_result(code: &str, mut parser: Box<dyn LuaDescParser>) {
    let tree = LuaParser::parse(code, ParserConfig::default());
    let Some(desc) = tree
        .get_red_root()
        .descendants()
        .filter(|node| matches!(node.kind(), LuaKind::Syntax(LuaSyntaxKind::DocDescription)))
        .next()
    else {
        panic!("No desc found in {:?}", tree.get_red_root());
    };
    let ranges = parser.parse(code, LuaDocDescription::cast(desc).unwrap());
    let result = format_result(code, ranges);
    println!("{}", result);
}

pub fn format_result(text: &str, mut items: Vec<DescItem>) -> String {
    sort_result(&mut items);

    let mut pos = 0;
    let mut cur_items: Vec<DescItem> = Vec::new();
    let mut res = String::new();

    fn pop_cur_itemss(
        text: &str,
        cur_itemss: &mut Vec<DescItem>,
        pos: &mut usize,
        end: usize,
        res: &mut String,
    ) {
        while let Some(cur_items) = cur_itemss.last() {
            let cur_end: usize = cur_items.range.end().into();
            if cur_end <= end {
                *res += &text[*pos..cur_end];
                *pos = cur_end;
                *res += &format!("</{:?}>", cur_items.kind);
                cur_itemss.pop();
            } else {
                break;
            }
        }

        *res += &text[*pos..end];
        *pos = end;
    }

    for next_item in items {
        pop_cur_itemss(
            text,
            &mut cur_items,
            &mut pos,
            next_item.range.start().into(),
            &mut res,
        );
        res += &text[pos..next_item.range.start().into()];
        pos = next_item.range.start().into();
        res += &format!("<{:?}>", next_item.kind);
        cur_items.push(next_item);
    }

    pop_cur_itemss(text, &mut cur_items, &mut pos, text.len(), &mut res);
    res += &text[pos..];

    res
}
