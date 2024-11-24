mod decl;
mod flow;
mod doc;
mod member;
mod unresolve;

use crate::{db_index::DbIndex, Emmyrc, InFiled};
use emmylua_parser::LuaSyntaxTree;
use unresolve::UnResolve;

pub fn analyze(db: &mut DbIndex, context: AnalyzeContext) {
    let mut context = context;
    decl::analyze(db, &mut context);
    flow::analyze(db, &mut context);
    doc::analyze(db, &mut context);
    member::analyze(db, &mut context);
}

#[allow(unused)]
#[derive(Debug)]
pub struct AnalyzeContext<'a> {
    tree_list: Vec<InFiled<&'a LuaSyntaxTree>>,
    config: &'a Emmyrc,
    unresolves: Vec<UnResolve>,
}
#[allow(unused)]
impl<'a> AnalyzeContext<'a> {
    pub fn new(emmyrc: &'a Emmyrc) -> Self {
        Self {
            tree_list: Vec::new(),
            config: emmyrc,
            unresolves: Vec::new(),
        }
    }

    pub fn add_tree(&mut self, tree: InFiled<&'a LuaSyntaxTree>) {
        self.tree_list.push(tree);
    }

    pub fn add_unresolve(&mut self, un_resolve: UnResolve) {
        self.unresolves.push(un_resolve);
    }
}
