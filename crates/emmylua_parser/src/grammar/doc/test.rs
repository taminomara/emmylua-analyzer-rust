#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, LuaParser};

    macro_rules! assert_ast_eq {
        ($lua_code:expr, $expected:expr) => {
            let tree = LuaParser::parse($lua_code, ParserConfig::default());
            let result = format!("{:#?}", tree.get_red_root()).trim().to_string();
            let expected = $expected.trim().to_string();
            assert_eq!(result, expected);
        };
    }

    #[allow(unused)]
    fn print_ast(lua_code: &str) {
        let tree = LuaParser::parse(lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_red_root());
    }

    #[test]
    fn test_normal_doc() {
        let code = r#"
        -- comment

        -- hihihi
        --     hello
        --yyyy
        "#;
        print_ast(code);
        let ast = r#"
Syntax(Chunk)@0..83
  Token(TkEndOfLine)@0..1 "\n"
  Token(TkWhitespace)@1..9 "        "
  Syntax(Comment)@9..19
    Token(TkNormalStart)@9..11 "--"
    Syntax(DocDescription)@11..19
      Token(TkDocDetail)@11..19 " comment"
  Token(TkEndOfLine)@19..20 "\n"
  Token(TkEndOfLine)@20..21 "\n"
  Token(TkWhitespace)@21..29 "        "
  Syntax(Comment)@29..74
    Token(TkNormalStart)@29..31 "--"
    Syntax(DocDescription)@31..74
      Token(TkDocDetail)@31..38 " hihihi"
      Token(TkEndOfLine)@38..39 "\n"
      Token(TkWhitespace)@39..47 "        "
      Token(TkNormalStart)@47..49 "--"
      Token(TkWhitespace)@49..54 "     "
      Token(TkDocDetail)@54..59 "hello"
      Token(TkEndOfLine)@59..60 "\n"
      Token(TkWhitespace)@60..68 "        "
      Token(TkNormalStart)@68..70 "--"
      Token(TkDocDetail)@70..74 "yyyy"
  Token(TkEndOfLine)@74..75 "\n"
  Token(TkWhitespace)@75..83 "        "
        "#;
        assert_ast_eq!(code, ast);
    }

    #[test]
    fn test_class_doc() {
        let code = r#"
        ---@class A hello
        ---@class B<T>
        ---@class C<T>: B<T>
        ---@class D : A, C<T>
        ---@class E hello
        "#;

        let result = r#"
Syntax(Chunk)@0..143
  Token(TkEndOfLine)@0..1 "\n"
  Token(TkWhitespace)@1..9 "        "
  Syntax(Comment)@9..134
    Token(TkDocStart)@9..13 "---@"
    Syntax(DocTagClass)@13..35
      Token(TkTagClass)@13..18 "class"
      Token(TkWhitespace)@18..19 " "
      Token(TkName)@19..20 "A"
      Token(TkWhitespace)@20..21 " "
      Syntax(DocDescription)@21..35
        Token(TkDocDetail)@21..26 "hello"
        Token(TkEndOfLine)@26..27 "\n"
        Token(TkWhitespace)@27..35 "        "
    Token(TkDocStart)@35..39 "---@"
    Syntax(DocTagClass)@39..58
      Token(TkTagClass)@39..44 "class"
      Token(TkWhitespace)@44..45 " "
      Token(TkName)@45..46 "B"
      Syntax(DocGenericDeclareList)@46..58
        Token(TkLt)@46..47 "<"
        Syntax(DocGenericParameter)@47..48
          Token(TkName)@47..48 "T"
        Token(TkGt)@48..49 ">"
        Token(TkEndOfLine)@49..50 "\n"
        Token(TkWhitespace)@50..58 "        "
    Token(TkDocStart)@58..62 "---@"
    Syntax(DocTagClass)@62..87
      Token(TkTagClass)@62..67 "class"
      Token(TkWhitespace)@67..68 " "
      Token(TkName)@68..69 "C"
      Syntax(DocGenericDeclareList)@69..72
        Token(TkLt)@69..70 "<"
        Syntax(DocGenericParameter)@70..71
          Token(TkName)@70..71 "T"
        Token(TkGt)@71..72 ">"
      Token(TkColon)@72..73 ":"
      Token(TkWhitespace)@73..74 " "
      Syntax(DocTypeList)@74..87
        Syntax(TypeGeneric)@74..87
          Syntax(TypeName)@74..75
            Token(TkName)@74..75 "B"
          Token(TkLt)@75..76 "<"
          Syntax(DocTypeList)@76..77
            Syntax(TypeName)@76..77
              Token(TkName)@76..77 "T"
          Token(TkGt)@77..78 ">"
          Token(TkEndOfLine)@78..79 "\n"
          Token(TkWhitespace)@79..87 "        "
    Token(TkDocStart)@87..91 "---@"
    Syntax(DocTagClass)@91..117
      Token(TkTagClass)@91..96 "class"
      Token(TkWhitespace)@96..97 " "
      Token(TkName)@97..98 "D"
      Token(TkWhitespace)@98..99 " "
      Token(TkColon)@99..100 ":"
      Token(TkWhitespace)@100..101 " "
      Syntax(DocTypeList)@101..117
        Syntax(TypeName)@101..102
          Token(TkName)@101..102 "A"
        Token(TkComma)@102..103 ","
        Token(TkWhitespace)@103..104 " "
        Syntax(TypeGeneric)@104..117
          Syntax(TypeName)@104..105
            Token(TkName)@104..105 "C"
          Token(TkLt)@105..106 "<"
          Syntax(DocTypeList)@106..107
            Syntax(TypeName)@106..107
              Token(TkName)@106..107 "T"
          Token(TkGt)@107..108 ">"
          Token(TkEndOfLine)@108..109 "\n"
          Token(TkWhitespace)@109..117 "        "
    Token(TkDocStart)@117..121 "---@"
    Syntax(DocTagClass)@121..134
      Token(TkTagClass)@121..126 "class"
      Token(TkWhitespace)@126..127 " "
      Token(TkName)@127..128 "E"
      Token(TkWhitespace)@128..129 " "
      Syntax(DocDescription)@129..134
        Token(TkDocDetail)@129..134 "hello"
  Token(TkEndOfLine)@134..135 "\n"
  Token(TkWhitespace)@135..143 "        "
        "#;
        assert_ast_eq!(code, result);
    }

    #[test]
    fn test_basic_doc() {
        let lua_code = r#"
        ---@class A
        ---@field a number
        local a = {}

        ---@type A
        local b = {}

        ---@alias A B

        ---@param a number
        ---@param b string
        local function f(a, b)
        end

        ---@return number
        local function f()
        end

        ---@overload fun(a: number, b: string): number
        ---@overload fun(a: string, b: number): string
        local function f(a, b)
        end

        ---@generic T
        ---@param a T
        ---@return T
        local function f(a)
        end
        "#;

        let tree = LuaParser::parse(lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_red_root());
    }
}
