#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, syntax::traits::LuaAstNode, LuaAst, LuaParser};

    #[test]
    fn test_iter_ast() {
        let code = r#"
            local a = 1
            local b = 2
            print(a + b)
        "#;
        let tree = LuaParser::parse(code, ParserConfig::default());

        let chunk = tree.get_chunk_node();
        for node in chunk.descendants::<LuaAst>() {
            println!("{:?}", node);
        }
    }
}