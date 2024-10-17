#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, LuaParser};
    #[test]
    fn test_parse_and_print_ast() {
        let lua_code = r#"
            function foo(a, b)
                return a + b
            end
        "#;

        let tree = LuaParser::parse(&lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_root());
    }
}
