use std::collections::HashSet;

use code_analysis::{DbIndex, LuaDeclId, LuaDocument, SemanticModel};
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaForRangeStat, LuaForStat, LuaLocalFuncStat, LuaLocalStat, LuaNameExpr, LuaParamList
};
use rowan::TextRange;

use super::{EmmyAnnotator, EmmyAnnotatorType};

pub fn build_annotators(semantic: &SemanticModel) -> Vec<EmmyAnnotator> {
    let mut result = vec![];
    let document = semantic.get_document();
    let root = semantic.get_root();
    let db = semantic.get_db();
    let mut use_range_set = HashSet::new();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaLocalStat(local_stat) => {
                build_local_stat_annotator(
                    &db,
                    &document,
                    &mut use_range_set,
                    &mut result,
                    local_stat,
                );
            }
            LuaAst::LuaForStat(for_stat) => {
                build_for_stat_annotator(&db, &document, &mut use_range_set, &mut result, for_stat);
            }
            LuaAst::LuaLocalFuncStat(local_func_stat) => {
                build_local_func_stat_annotator(
                    &db,
                    &document,
                    &mut use_range_set,
                    &mut result,
                    local_func_stat,
                );
            }
            LuaAst::LuaForRangeStat(for_range_stat) => {
                build_for_range_annotator(
                    &db,
                    &document,
                    &mut use_range_set,
                    &mut result,
                    for_range_stat,
                );
            }
            LuaAst::LuaParamList(params_list) => {
                build_params_annotator(
                    &db,
                    &document,
                    &mut use_range_set,
                    &mut result,
                    params_list,
                );
            }
            LuaAst::LuaNameExpr(name_expr) => {
                build_name_expr_annotator(&document, &mut use_range_set, &mut result, name_expr);
            }
            _ => {}
        }
    }

    result
}

fn build_local_stat_annotator(
    db: &DbIndex,
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    local_stat: LuaLocalStat,
) -> Option<()> {
    let file_id = document.get_file_id();
    let locals = local_stat.get_local_name_list();
    for local_name in locals {
        let mut annotator = EmmyAnnotator {
            typ: EmmyAnnotatorType::Local,
            ranges: vec![],
        };
        let name_token = local_name.get_name_token()?;
        let name_token_range = name_token.get_range();
        use_range_set.insert(name_token_range);
        annotator
            .ranges
            .push(document.to_lsp_range(name_token_range)?);

        let decl_id = LuaDeclId::new(file_id, local_name.get_position());
        let ref_ranges = db
            .get_reference_index()
            .get_local_references(&file_id, &decl_id);
        if let Some(ref_ranges) = ref_ranges {
            for range in ref_ranges {
                use_range_set.insert(*range);
                annotator.ranges.push(document.to_lsp_range(*range)?);
            }
        }

        result.push(annotator);
    }

    Some(())
}

fn build_params_annotator(
    db: &DbIndex,
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    param_list: LuaParamList,
) -> Option<()> {
    let file_id = document.get_file_id();
    for param_name in param_list.get_params() {
        let mut annotator = EmmyAnnotator {
            typ: EmmyAnnotatorType::Param,
            ranges: vec![],
        };
        let name_token = param_name.get_name_token()?;
        let name_token_range = name_token.get_range();
        use_range_set.insert(name_token_range);
        annotator
            .ranges
            .push(document.to_lsp_range(name_token_range)?);

        let decl_id = LuaDeclId::new(file_id, param_name.get_position());
        let ref_ranges = db
            .get_reference_index()
            .get_local_references(&file_id, &decl_id);
        if let Some(ref_ranges) = ref_ranges {
            for range in ref_ranges {
                use_range_set.insert(*range);
                annotator.ranges.push(document.to_lsp_range(*range)?);
            }
        }

        result.push(annotator);
    }

    Some(())
}

fn build_name_expr_annotator(
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    name_expr: LuaNameExpr,
) -> Option<()> {
    let name_range = name_expr.get_range();
    if use_range_set.contains(&name_range) {
        return Some(());
    }

    let name_text = name_expr.get_name_text()?;
    if name_text == "self" || name_text == "_" {
        return Some(());
    }

    let mut annotator = EmmyAnnotator {
        typ: EmmyAnnotatorType::Global,
        ranges: vec![],
    };

    let lsp_range = document.to_lsp_range(name_range)?;
    annotator.ranges.push(lsp_range);

    result.push(annotator);

    Some(())
}

fn build_for_stat_annotator(
    db: &DbIndex,
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    for_stat: LuaForStat,
) -> Option<()> {
    let file_id = document.get_file_id();
    let name_token = for_stat.get_var_name()?;
    let name_range = name_token.get_range();

    let mut annotator = EmmyAnnotator {
        typ: EmmyAnnotatorType::Param,
        ranges: vec![],
    };

    let lsp_range = document.to_lsp_range(name_range)?;
    annotator.ranges.push(lsp_range);

    let decl_id = LuaDeclId::new(file_id, name_token.get_position());
    let ref_ranges = db
        .get_reference_index()
        .get_local_references(&file_id, &decl_id);
    if let Some(ref_ranges) = ref_ranges {
        for range in ref_ranges {
            use_range_set.insert(*range);
            annotator.ranges.push(document.to_lsp_range(*range)?);
        }
    }

    result.push(annotator);

    Some(())
}

fn build_for_range_annotator(
    db: &DbIndex,
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    for_stat: LuaForRangeStat,
) -> Option<()> {
    let file_id = document.get_file_id();
    for name_token in for_stat.get_var_name_list() {
        let name_range = name_token.get_range();

        let mut annotator = EmmyAnnotator {
            typ: EmmyAnnotatorType::Param,
            ranges: vec![],
        };

        let lsp_range = document.to_lsp_range(name_range)?;
        annotator.ranges.push(lsp_range);

        let decl_id = LuaDeclId::new(file_id, name_token.get_position());
        let ref_ranges = db
            .get_reference_index()
            .get_local_references(&file_id, &decl_id);
        if let Some(ref_ranges) = ref_ranges {
            for range in ref_ranges {
                use_range_set.insert(*range);
                annotator.ranges.push(document.to_lsp_range(*range)?);
            }
        }

        result.push(annotator);
    }
    Some(())
}

fn build_local_func_stat_annotator(
    db: &DbIndex,
    document: &LuaDocument,
    use_range_set: &mut HashSet<TextRange>,
    result: &mut Vec<EmmyAnnotator>,
    local_func_stat: LuaLocalFuncStat,
) -> Option<()> {
    let file_id = document.get_file_id();
    let func_name = local_func_stat.get_local_name()?;
    let name_token = func_name.get_name_token()?;
    let name_range = name_token.get_range();

    let mut annotator = EmmyAnnotator {
        typ: EmmyAnnotatorType::Local,
        ranges: vec![],
    };

    let lsp_range = document.to_lsp_range(name_range)?;
    annotator.ranges.push(lsp_range);

    let decl_id = LuaDeclId::new(file_id, name_token.get_position());
    let ref_ranges = db
        .get_reference_index()
        .get_local_references(&file_id, &decl_id);
    if let Some(ref_ranges) = ref_ranges {
        for range in ref_ranges {
            use_range_set.insert(*range);
            annotator.ranges.push(document.to_lsp_range(*range)?);
        }
    }

    result.push(annotator);

    Some(())
}