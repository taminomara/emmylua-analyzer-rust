use rowan::Language;

use crate::{
    kind::{LuaSyntaxKind, LuaTokenKind}, parser::MarkEvent, text::SourceRange, LuaKind, LuaLanguage, LuaSyntaxNode
};

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
        let mut parents: Vec<LuaSyntaxKind> = Vec::new();
        for i in 0..self.events.len() {
            match std::mem::replace(&mut self.events[i], MarkEvent::none()) {
                MarkEvent::NodeStart {
                    kind: LuaSyntaxKind::None,
                    ..
                } => {}
                MarkEvent::NodeStart { kind, parent } => {
                    parents.push(kind);
                    let mut parent_position = parent;
                    while parent_position > 0 {
                        match std::mem::replace(
                            &mut self.events[parent_position],
                            MarkEvent::none(),
                        ) {
                            MarkEvent::NodeStart { kind, parent } => {
                                parents.push(kind);
                                parent_position = parent;
                            }
                            _ => unreachable!(),
                        }
                    }

                    for kind in parents.drain(..).rev() {
                        self.start_node(kind);
                    }
                }
                MarkEvent::NodeEnd => {
                    self.finish_node();
                }
                MarkEvent::EatToken { kind, range } => {
                    self.token(kind, range);
                }
            }
        }

        self.finish_node();
    }

    fn token(&mut self, kind: LuaTokenKind, range: SourceRange) {
        let lua_kind = LuaKind::from(kind);
        let token_text = &self.text[range.start_offset..range.end_offset()];
        self.green_builder.token(LuaLanguage::kind_to_raw(lua_kind), token_text);
    }

    fn start_node(&mut self, kind: LuaSyntaxKind) {
        let lua_kind = LuaKind::from(kind);
        self.green_builder.start_node(LuaLanguage::kind_to_raw(lua_kind));
    }

    fn finish_node(&mut self) {
        self.green_builder.finish_node();
    }

    pub fn finish(self) -> LuaSyntaxNode {
        let root = self.green_builder.finish();
        LuaSyntaxNode::new_root(root)
    }
}
