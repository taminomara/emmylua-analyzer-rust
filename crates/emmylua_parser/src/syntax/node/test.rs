#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, syntax::traits::LuaAstNode, LuaAst, LuaLocalStat, LuaParser};

    #[allow(unused)]
    fn get_ast_node<N: LuaAstNode>(code: &str) -> N {
        let tree = LuaParser::parse(code, ParserConfig::default());
        let chunk = tree.get_chunk_node();
        let node = chunk.descendants::<N>().next().unwrap();
        node
    }

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

    #[test]
    fn test_local_stat1() {
        let code = "local a = 123";
        let local_stat = get_ast_node::<LuaLocalStat>(code);
        let mut name_list = local_stat.get_local_name_list();
        let local_name = name_list.next().unwrap();
        assert_eq!(format!("{:?}", local_name), r#"LuaLocalName { syntax: Syntax(LocalName)@6..7 }"#);
        let mut expr_list = local_stat.get_value_exprs();
        let expr = expr_list.next().unwrap();
        println!("{:?}", expr);
        assert_eq!(format!("{:?}", expr), r#"LiteralExpr(LuaLiteralExpr { syntax: Syntax(LiteralExpr)@10..13 })"#);
    }

    #[test]
    fn test_name_token() {
        let code = "local a<const> = 123";
        let local_stat = get_ast_node::<LuaLocalStat>(code);
        let mut name_list = local_stat.get_local_name_list();
        let local_name1 = name_list.next().unwrap();
        let name = local_name1.get_name_token().unwrap();
        assert_eq!(name.get_name_text(), "a");
        let attrib = local_name1.get_attrib().unwrap();
        assert!(attrib.is_const());
    }
}