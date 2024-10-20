#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, LuaParser};

    #[test]
    fn test_line_col() {
        let code = r#"
--hihii
--你好啊
--aiaiai
--1231313 好了好了
        "#;
        let tree = LuaParser::parse(code, ParserConfig::default());
        let offset_1 = tree.get_offset(1, 3).unwrap();
        assert_eq!(offset_1, 4.into());
        let offset_2 = tree.get_offset(2, 4).unwrap();
        assert_eq!(offset_2, 17.into());
        let offset_3 = tree.get_offset(3, 0).unwrap();
        assert_eq!(offset_3, 21.into());

        let line_col_1 = tree.get_line_col(offset_1).unwrap();
        assert_eq!(line_col_1, (1, 3));
        let line_col_2 = tree.get_line_col(offset_2).unwrap();
        assert_eq!(line_col_2, (2, 4));
        let line_col_3 = tree.get_line_col(offset_3).unwrap();
        assert_eq!(line_col_3, (3, 0));
    }
}