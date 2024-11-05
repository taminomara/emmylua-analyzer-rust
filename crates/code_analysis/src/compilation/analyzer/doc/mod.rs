mod tags;
mod infer_type;

use emmylua_parser::{LuaAst, LuaAstNode, LuaComment, LuaSyntaxTree};
use crate::{db_index::DbIndex, FileId};
use super::AnalyzeContext;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in context.tree_list.iter() {
        let tree = in_filed_tree.value;
        let root = tree.get_chunk_node();
        for comment in root.descendants::<LuaComment>() {
            let mut analyzer = DocAnalyzer::new(db, in_filed_tree.file_id);
            analyze_comment(&mut analyzer, comment);
        }
    }
}

fn analyze_comment(analyzer: &mut DocAnalyzer, comment: LuaComment) {
    for tag in comment.get_doc_tags() {
        tags::analyze_tag(analyzer, tag);
    }
}

#[derive(Debug)]
pub struct DocAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
}

impl DocAnalyzer<'_> {
    pub fn new<'a>(db: &'a mut DbIndex, file_id: FileId) -> DocAnalyzer<'a> {
        DocAnalyzer { file_id, db }
    }
}