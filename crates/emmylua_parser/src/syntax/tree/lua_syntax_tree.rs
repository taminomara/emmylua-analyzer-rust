
use crate::{parser_error::LuaParseError, syntax::{node::LuaChunk, traits::LuaAstNode}, LuaSyntaxNode};

#[derive(Debug, Clone)]
pub struct LuaSyntaxTree {
    root: LuaSyntaxNode,
    errors: Vec<LuaParseError>,
}

impl LuaSyntaxTree {
    pub fn new(root: LuaSyntaxNode, errors: Vec<LuaParseError>) -> Self {
        LuaSyntaxTree {
            root,
            errors,
        }
    }

    // get root node
    pub fn get_red_root(&self) -> &LuaSyntaxNode {
        &self.root
    }

    // get chunk node, only can cast to LuaChunk
    pub fn get_chunk_node(&self) -> LuaChunk {
        LuaChunk::cast(self.root.clone()).unwrap()
    }

    pub fn get_errors(&self) -> &[LuaParseError] {
        &self.errors
    }
}
