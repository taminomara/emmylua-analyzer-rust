use emmylua_parser::{LuaSyntaxId, LuaSyntaxKind};

use crate::compilation::analyzer::decl;

use super::FlowAnalyzer;

pub fn analyze(analyzer: &mut FlowAnalyzer) -> Option<()> {
    let references_index = analyzer.db.get_reference_index();
    let decl_index = analyzer.db.get_decl_index();
    let refs_map = references_index.get_local_references_map(&analyzer.file_id)?;

    for (decl_id, ranges) in refs_map {
        let decl = decl_index.get_decl(decl_id)?;
        let decl_syntax_id = decl.get_syntax_id();
        for range in ranges {
            let syntax_id = LuaSyntaxId::new(LuaSyntaxKind::NameExpr.into(), range.clone());
        }
    }

    Some(())
}
