use emmylua_parser::{LuaAst, LuaAstNode};
use rowan::NodeOrToken;

#[allow(unused)]
#[derive(Debug)]
pub struct LuaFormatter {
    root: LuaAst,
}

#[allow(unused)]
impl LuaFormatter {
    pub fn new(root: LuaAst) -> Self {
        Self { root }
    }

    pub fn get_formatted_text(&self) -> String {
        let mut formatted_text = String::new();
        for node_or_token in self.root.syntax().descendants_with_tokens() {
            if let NodeOrToken::Token(token) = node_or_token {
                formatted_text.push_str(&token.text());
            }
        }

        formatted_text
    }
}
