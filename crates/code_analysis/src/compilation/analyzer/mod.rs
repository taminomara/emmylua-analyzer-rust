mod decl;
mod symbol;
mod doc;

use std::collections::HashMap;

use emmylua_parser::{LuaComment, LuaSyntaxTree};
use crate::{db_index::{DbIndex, LuaType}, InFiled};

pub fn analyze(db: &mut DbIndex, context: AnalyzeContext) {
    let mut context = context;
    decl::analyze(db, &mut context);
    doc::analyze(db, &mut context);
    symbol::analyze(db, &mut context);
}

pub struct AnalyzeContext<'a> {
    tree_list: Vec<InFiled<&'a LuaSyntaxTree>>,
    bind_type_def: HashMap<LuaComment, LuaType>
}

impl<'a> AnalyzeContext<'a> {
    pub fn new() -> Self {
        Self {
            tree_list: Vec::new(),
            bind_type_def: HashMap::new()
        }
    }

    pub fn add_tree(&mut self, tree: InFiled<&'a LuaSyntaxTree>) {
        self.tree_list.push(tree);
    }
}
