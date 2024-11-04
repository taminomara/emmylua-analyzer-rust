use emmylua_parser::LuaSyntaxTree;

use crate::{db_index::DbIndex, FileId};

use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in context.tree_list.iter() {
        let mut analyzer = SymbolAnalyzer::new(db, in_filed_tree.file_id, in_filed_tree.value);
        analyzer.analyze();
    }
}

#[derive(Debug)]
pub struct SymbolAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
}

impl SymbolAnalyzer<'_> {
    pub fn new<'a>(db: &'a mut DbIndex, file_id: FileId, tree: &'a LuaSyntaxTree) -> SymbolAnalyzer<'a> {
        SymbolAnalyzer { file_id, db, tree }
    }

    pub fn analyze(&mut self) {

    }
}