mod decl;
mod doc;
mod lua;
mod symbol;

use std::collections::HashMap;

use crate::{
    db_index::{DbIndex, LuaType},
    InFiled,
};
use emmylua_parser::{LuaIndexExpr, LuaSyntaxTree, LuaTableField};

pub fn analyze(db: &mut DbIndex, context: AnalyzeContext) {
    let mut context = context;
    decl::analyze(db, &mut context);
    doc::analyze(db, &mut context);
    symbol::analyze(db, &mut context);
}

#[derive(Debug)]
pub struct AnalyzeContext<'a> {
    tree_list: Vec<InFiled<&'a LuaSyntaxTree>>,
    unresolve_index_expr_type: HashMap<LuaIndexExpr, LuaType>,
    unresolve_table_field_type: HashMap<LuaTableField, LuaType>,
}

impl<'a> AnalyzeContext<'a> {
    pub fn new() -> Self {
        Self {
            tree_list: Vec::new(),
            unresolve_index_expr_type: HashMap::new(),
            unresolve_table_field_type: HashMap::new(),
        }
    }

    pub fn add_tree(&mut self, tree: InFiled<&'a LuaSyntaxTree>) {
        self.tree_list.push(tree);
    }
}


