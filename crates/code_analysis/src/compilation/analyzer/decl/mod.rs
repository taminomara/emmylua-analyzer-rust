mod decl_analyzer;
mod analyze_node;

use decl_analyzer::DeclAnalyzer;

use crate::db_index::DbIndex;

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in context.tree_list.iter() {
        let mut analyzer = DeclAnalyzer::new(db, in_filed_tree.file_id, in_filed_tree.value);
        analyzer.analyze();
        let decl_tree = analyzer.build_decl_tree();
        db.add_decl_tree(decl_tree);
    }
}
