mod diagnostic_tags;
mod field_or_operator_def_tags;
mod file_generic_index;
mod infer_type;
mod property_tags;
mod tags;
mod type_def_tags;
mod type_ref_tags;

use super::AnalyzeContext;
use crate::{
    db_index::{DbIndex, LuaTypeDeclId},
    FileId,
};
use emmylua_parser::{LuaAstNode, LuaComment, LuaDocDescriptionOwner, LuaSyntaxNode};
use file_generic_index::FileGenericIndex;
use tags::get_owner_id;

pub(crate) fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let tree_list = context.tree_list.clone();
    for in_filed_tree in tree_list.iter() {
        let root = &in_filed_tree.value;
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

fn analyze_comment(analyzer: &mut DocAnalyzer) -> Option<()> {
    let comment = analyzer.comment.clone();
    for tag in comment.get_doc_tags() {
        tags::analyze_tag(analyzer, tag);
    }

    let owenr = get_owner_id(analyzer)?;
    let comment_description = comment.get_description()?.get_description_text();
    analyzer.db.get_property_index_mut().add_description(
        analyzer.file_id,
        owenr,
        comment_description,
    );

    Some(())
}

#[derive(Debug)]
pub struct DocAnalyzer<'a> {
    file_id: FileId,
    db: &'a mut DbIndex,
    generic_index: &'a mut FileGenericIndex,
    current_type_id: Option<LuaTypeDeclId>,
    comment: LuaComment,
    root: LuaSyntaxNode,
}

impl<'a> DocAnalyzer<'a> {
    pub fn new(
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
        }
    }
}
