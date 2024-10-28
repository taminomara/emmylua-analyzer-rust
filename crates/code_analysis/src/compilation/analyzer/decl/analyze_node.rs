use emmylua_parser::{LuaAst, LuaAstNode};

use super::decl_analyzer::DeclAnalyzer;


pub fn analyze_node(analyzer: &mut DeclAnalyzer) {
    let tree = analyzer.get_tree();
    let root = tree.get_chunk_node();
    let root_scope_id = analyzer.get_decl_tree().create_scope(None);
    for node in root.descendants::<LuaAst>() {
        match node {
            _ => {}
        }
    }
}