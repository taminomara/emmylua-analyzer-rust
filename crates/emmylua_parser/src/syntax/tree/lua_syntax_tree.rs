use crate::{text::LineIndex, LuaSyntaxNode};

pub struct LuaSyntaxTree {
    root: LuaSyntaxNode,
    line_index: LineIndex,
}

impl LuaSyntaxTree {
    pub fn new(root: LuaSyntaxNode, line_index: LineIndex) -> Self {
        LuaSyntaxTree {
            root,
            line_index,
        }
    }

    pub fn get_line_index(&self) -> &LineIndex {
        &self.line_index
    }

    pub fn get_red_root(&self) -> &LuaSyntaxNode {
        &self.root
    }
}
