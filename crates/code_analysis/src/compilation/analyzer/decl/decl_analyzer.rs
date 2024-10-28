use emmylua_parser::LuaSyntaxTree;

use crate::{db_index::{DbIndex, LuaDeclarationTree}, FileId};

use super::analyze_node::analyze_node;

#[derive(Debug)]
pub struct DeclAnalyzer<'a> {
    db: &'a mut DbIndex,
    tree: &'a LuaSyntaxTree,
    decl: LuaDeclarationTree
}

impl<'a> DeclAnalyzer<'a> {
    pub fn new(db: &'a mut DbIndex, file_id: FileId, tree:&'a LuaSyntaxTree) -> DeclAnalyzer<'a> {
        DeclAnalyzer {
            db,
            tree,
            decl: LuaDeclarationTree::new(file_id)
        }
    }

    pub fn analyze(&mut self) {
        analyze_node(self);
    }

    pub fn build_decl_tree(self) -> LuaDeclarationTree {
        self.decl
    }

    pub(crate) fn get_db(&mut self) -> &mut DbIndex {
        self.db
    }

    pub(crate) fn get_tree(&self) -> &LuaSyntaxTree {
        self.tree
    }

    pub(crate) fn get_decl_tree(&mut self) -> &mut LuaDeclarationTree {
        &mut self.decl
    }
}