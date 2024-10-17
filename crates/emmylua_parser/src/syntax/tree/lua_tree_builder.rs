use crate::{kind::{LuaSyntaxKind, LuaTokenKind}, parser::MarkEvent, LuaSyntaxNode};

#[derive(Debug)]
pub struct LuaTreeBuilder<'a> {
    text: &'a str,
    events: Vec<MarkEvent>,
    green_builder: rowan::GreenNodeBuilder<'static>,
}

impl<'a> LuaTreeBuilder<'a> {
    pub fn new(text: &'a str, events: Vec<MarkEvent>) -> Self {
        LuaTreeBuilder {
            text,
            events,
            green_builder: rowan::GreenNodeBuilder::new(),
        }
    }

    pub fn build(&mut self) {
        self.start_node(LuaSyntaxKind::Chunk);
        
        self.finish_node();
    }

    fn token(&mut self, kind: LuaTokenKind) {

    }

    fn start_node(&mut self, kind: LuaSyntaxKind) {

    }

    fn finish_node(&mut self) {

    }

    pub fn finish(self) -> LuaSyntaxNode {
        let root = self.green_builder.finish();
        LuaSyntaxNode::new_root(root)
    }
}
