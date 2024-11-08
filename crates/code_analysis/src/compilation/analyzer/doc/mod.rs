mod file_generic_index;
mod infer_type;
mod tags;
mod type_def_tags;

use super::AnalyzeContext;
use crate::{
    db_index::{DbIndex, LuaTypeDeclId},
    FileId,
};
use emmylua_parser::{LuaAstNode, LuaComment, LuaSyntaxNode};
use file_generic_index::FileGenericIndex;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    for in_filed_tree in context.tree_list.iter() {
        let tree = in_filed_tree.value;
        let root = tree.get_chunk_node();
        let mut generic_index = FileGenericIndex::new();
        for comment in root.descendants::<LuaComment>() {
            let mut analyzer = DocAnalyzer::new(
                db,
                in_filed_tree.file_id,
                &mut generic_index,
                comment,
                root.syntax().clone(),
            );
            analyze_comment(&mut analyzer);
        }
    }
}

fn analyze_comment(analyzer: &mut DocAnalyzer) {
    let comment = &analyzer.comment;
    for tag in comment.get_doc_tags() {
        tags::analyze_tag(analyzer, tag);
    }
}

#[derive(Debug)]
pub struct DocAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    generic_index: &'a mut FileGenericIndex,
    current_type_id: Option<LuaTypeDeclId>,
    comment: LuaComment,
    root: LuaSyntaxNode,
    bind_type: Option<LuaTypeDeclId>,
}

impl DocAnalyzer<'_> {
    pub fn new<'a>(
        db: &'a mut DbIndex,
        file_id: FileId,
        generic_index: &'a mut FileGenericIndex,
        comment: LuaComment,
        root: LuaSyntaxNode,
    ) -> DocAnalyzer<'a> {
        DocAnalyzer {
            file_id,
            db,
            generic_index,
            current_type_id: None,
            comment,
            root,
            bind_type: None,
        }
    }
}
