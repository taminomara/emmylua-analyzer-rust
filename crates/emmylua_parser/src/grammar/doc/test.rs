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
        let ast = r#"
Syntax(Chunk)@0..83
  Syntax(Block)@0..83
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
  Syntax(Block)@0..143
    Token(TkEndOfLine)@0..1 "\n"
    Token(TkWhitespace)@1..9 "        "
    Syntax(Comment)@9..134
      Token(TkDocStart)@9..13 "---@"
      Syntax(DocTagClass)@13..26
        Token(TkTagClass)@13..18 "class"
        Token(TkWhitespace)@18..19 " "
        Token(TkName)@19..20 "A"
        Token(TkWhitespace)@20..21 " "
        Syntax(DocDescription)@21..26
          Token(TkDocDetail)@21..26 "hello"
      Token(TkEndOfLine)@26..27 "\n"
      Token(TkWhitespace)@27..35 "        "
      Token(TkDocStart)@35..39 "---@"
      Syntax(DocTagClass)@39..49
        Token(TkTagClass)@39..44 "class"
        Token(TkWhitespace)@44..45 " "
        Token(TkName)@45..46 "B"
        Syntax(DocGenericDeclareList)@46..49
          Token(TkLt)@46..47 "<"
          Syntax(DocGenericParameter)@47..48
            Token(TkName)@47..48 "T"
          Token(TkGt)@48..49 ">"
      Token(TkEndOfLine)@49..50 "\n"
      Token(TkWhitespace)@50..58 "        "
      Token(TkDocStart)@58..62 "---@"
      Syntax(DocTagClass)@62..78
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
        Syntax(DocTypeList)@74..78
          Syntax(TypeGeneric)@74..78
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
      Syntax(DocTagClass)@91..108
        Token(TkTagClass)@91..96 "class"
        Token(TkWhitespace)@96..97 " "
        Token(TkName)@97..98 "D"
        Token(TkWhitespace)@98..99 " "
        Token(TkColon)@99..100 ":"
        Token(TkWhitespace)@100..101 " "
        Syntax(DocTypeList)@101..108
          Syntax(TypeName)@101..102
            Token(TkName)@101..102 "A"
          Token(TkComma)@102..103 ","
          Token(TkWhitespace)@103..104 " "
          Syntax(TypeGeneric)@104..108
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
        let code = r#"
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
        print_ast(&code);
        let result = r#"
Syntax(Chunk)@0..565
  Syntax(Block)@0..565
    Token(TkEndOfLine)@0..1 "\n"
    Token(TkWhitespace)@1..9 "        "
    Syntax(Comment)@9..47
      Token(TkDocStart)@9..13 "---@"
      Syntax(DocTagClass)@13..20
        Token(TkTagClass)@13..18 "class"
        Token(TkWhitespace)@18..19 " "
        Token(TkName)@19..20 "A"
      Token(TkEndOfLine)@20..21 "\n"
      Token(TkWhitespace)@21..29 "        "
      Token(TkDocStart)@29..33 "---@"
      Syntax(DocTagField)@33..47
        Token(TkTagField)@33..38 "field"
        Token(TkWhitespace)@38..39 " "
        Token(TkName)@39..40 "a"
        Token(TkWhitespace)@40..41 " "
        Syntax(TypeName)@41..47
          Token(TkName)@41..47 "number"
    Token(TkEndOfLine)@47..48 "\n"
    Token(TkWhitespace)@48..56 "        "
    Syntax(LocalStat)@56..68
      Token(TkLocal)@56..61 "local"
      Token(TkWhitespace)@61..62 " "
      Syntax(LocalName)@62..63
        Token(TkName)@62..63 "a"
      Token(TkWhitespace)@63..64 " "
      Token(TkAssign)@64..65 "="
      Token(TkWhitespace)@65..66 " "
      Syntax(TableExpr)@66..68
        Token(TkLeftBrace)@66..67 "{"
        Token(TkRightBrace)@67..68 "}"
    Token(TkEndOfLine)@68..69 "\n"
    Token(TkEndOfLine)@69..70 "\n"
    Token(TkWhitespace)@70..78 "        "
    Syntax(Comment)@78..88
      Token(TkDocStart)@78..82 "---@"
      Syntax(DocTagType)@82..88
        Token(TkTagType)@82..86 "type"
        Token(TkWhitespace)@86..87 " "
        Syntax(TypeName)@87..88
          Token(TkName)@87..88 "A"
    Token(TkEndOfLine)@88..89 "\n"
    Token(TkWhitespace)@89..97 "        "
    Syntax(LocalStat)@97..109
      Token(TkLocal)@97..102 "local"
      Token(TkWhitespace)@102..103 " "
      Syntax(LocalName)@103..104
        Token(TkName)@103..104 "b"
      Token(TkWhitespace)@104..105 " "
      Token(TkAssign)@105..106 "="
      Token(TkWhitespace)@106..107 " "
      Syntax(TableExpr)@107..109
        Token(TkLeftBrace)@107..108 "{"
        Token(TkRightBrace)@108..109 "}"
    Token(TkEndOfLine)@109..110 "\n"
    Token(TkEndOfLine)@110..111 "\n"
    Token(TkWhitespace)@111..119 "        "
    Syntax(Comment)@119..132
      Token(TkDocStart)@119..123 "---@"
      Syntax(DocTagAlias)@123..132
        Token(TkTagAlias)@123..128 "alias"
        Token(TkWhitespace)@128..129 " "
        Token(TkName)@129..130 "A"
        Token(TkWhitespace)@130..131 " "
        Syntax(TypeName)@131..132
          Token(TkName)@131..132 "B"
    Token(TkEndOfLine)@132..133 "\n"
    Token(TkEndOfLine)@133..134 "\n"
    Token(TkWhitespace)@134..142 "        "
    Syntax(Comment)@142..187
      Token(TkDocStart)@142..146 "---@"
      Syntax(DocTagParam)@146..160
        Token(TkTagParam)@146..151 "param"
        Token(TkWhitespace)@151..152 " "
        Token(TkName)@152..153 "a"
        Token(TkWhitespace)@153..154 " "
        Syntax(TypeName)@154..160
          Token(TkName)@154..160 "number"
      Token(TkEndOfLine)@160..161 "\n"
      Token(TkWhitespace)@161..169 "        "
      Token(TkDocStart)@169..173 "---@"
      Syntax(DocTagParam)@173..187
        Token(TkTagParam)@173..178 "param"
        Token(TkWhitespace)@178..179 " "
        Token(TkName)@179..180 "b"
        Token(TkWhitespace)@180..181 " "
        Syntax(TypeName)@181..187
          Token(TkName)@181..187 "string"
    Token(TkEndOfLine)@187..188 "\n"
    Token(TkWhitespace)@188..196 "        "
    Syntax(LocalFuncStat)@196..230
      Token(TkLocal)@196..201 "local"
      Token(TkWhitespace)@201..202 " "
      Token(TkFunction)@202..210 "function"
      Token(TkWhitespace)@210..211 " "
      Syntax(LocalName)@211..212
        Token(TkName)@211..212 "f"
      Syntax(ClosureExpr)@212..230
        Syntax(ParamList)@212..218
          Token(TkLeftParen)@212..213 "("
          Syntax(ParamName)@213..214
            Token(TkName)@213..214 "a"
          Token(TkComma)@214..215 ","
          Token(TkWhitespace)@215..216 " "
          Syntax(ParamName)@216..217
            Token(TkName)@216..217 "b"
          Token(TkRightParen)@217..218 ")"
        Token(TkEndOfLine)@218..219 "\n"
        Token(TkWhitespace)@219..227 "        "
        Token(TkEnd)@227..230 "end"
    Token(TkEndOfLine)@230..231 "\n"
    Token(TkEndOfLine)@231..232 "\n"
    Token(TkWhitespace)@232..240 "        "
    Syntax(Comment)@240..257
      Token(TkDocStart)@240..244 "---@"
      Syntax(DocTagReturn)@244..257
        Token(TkTagReturn)@244..250 "return"
        Token(TkWhitespace)@250..251 " "
        Syntax(TypeName)@251..257
          Token(TkName)@251..257 "number"
    Token(TkEndOfLine)@257..258 "\n"
    Token(TkWhitespace)@258..266 "        "
    Syntax(LocalFuncStat)@266..296
      Token(TkLocal)@266..271 "local"
      Token(TkWhitespace)@271..272 " "
      Token(TkFunction)@272..280 "function"
      Token(TkWhitespace)@280..281 " "
      Syntax(LocalName)@281..282
        Token(TkName)@281..282 "f"
      Syntax(ClosureExpr)@282..296
        Syntax(ParamList)@282..284
          Token(TkLeftParen)@282..283 "("
          Token(TkRightParen)@283..284 ")"
        Token(TkEndOfLine)@284..285 "\n"
        Token(TkWhitespace)@285..293 "        "
        Token(TkEnd)@293..296 "end"
    Token(TkEndOfLine)@296..297 "\n"
    Token(TkEndOfLine)@297..298 "\n"
    Token(TkWhitespace)@298..306 "        "
    Syntax(Comment)@306..407
      Token(TkDocStart)@306..310 "---@"
      Syntax(DocTagOverload)@310..352
        Token(TkTagOverload)@310..318 "overload"
        Token(TkWhitespace)@318..319 " "
        Syntax(TypeFun)@319..352
          Token(TkName)@319..322 "fun"
          Token(TkLeftParen)@322..323 "("
          Syntax(DocTypedParameter)@323..332
            Token(TkName)@323..324 "a"
            Token(TkColon)@324..325 ":"
            Token(TkWhitespace)@325..326 " "
            Syntax(TypeName)@326..332
              Token(TkName)@326..332 "number"
          Token(TkComma)@332..333 ","
          Token(TkWhitespace)@333..334 " "
          Syntax(DocTypedParameter)@334..343
            Token(TkName)@334..335 "b"
            Token(TkColon)@335..336 ":"
            Token(TkWhitespace)@336..337 " "
            Syntax(TypeName)@337..343
              Token(TkName)@337..343 "string"
          Token(TkRightParen)@343..344 ")"
          Token(TkColon)@344..345 ":"
          Token(TkWhitespace)@345..346 " "
          Syntax(TypeName)@346..352
            Token(TkName)@346..352 "number"
      Token(TkEndOfLine)@352..353 "\n"
      Token(TkWhitespace)@353..361 "        "
      Token(TkDocStart)@361..365 "---@"
      Syntax(DocTagOverload)@365..407
        Token(TkTagOverload)@365..373 "overload"
        Token(TkWhitespace)@373..374 " "
        Syntax(TypeFun)@374..407
          Token(TkName)@374..377 "fun"
          Token(TkLeftParen)@377..378 "("
          Syntax(DocTypedParameter)@378..387
            Token(TkName)@378..379 "a"
            Token(TkColon)@379..380 ":"
            Token(TkWhitespace)@380..381 " "
            Syntax(TypeName)@381..387
              Token(TkName)@381..387 "string"
          Token(TkComma)@387..388 ","
          Token(TkWhitespace)@388..389 " "
          Syntax(DocTypedParameter)@389..398
            Token(TkName)@389..390 "b"
            Token(TkColon)@390..391 ":"
            Token(TkWhitespace)@391..392 " "
            Syntax(TypeName)@392..398
              Token(TkName)@392..398 "number"
          Token(TkRightParen)@398..399 ")"
          Token(TkColon)@399..400 ":"
          Token(TkWhitespace)@400..401 " "
          Syntax(TypeName)@401..407
            Token(TkName)@401..407 "string"
    Token(TkEndOfLine)@407..408 "\n"
    Token(TkWhitespace)@408..416 "        "
    Syntax(LocalFuncStat)@416..450
      Token(TkLocal)@416..421 "local"
      Token(TkWhitespace)@421..422 " "
      Token(TkFunction)@422..430 "function"
      Token(TkWhitespace)@430..431 " "
      Syntax(LocalName)@431..432
        Token(TkName)@431..432 "f"
      Syntax(ClosureExpr)@432..450
        Syntax(ParamList)@432..438
          Token(TkLeftParen)@432..433 "("
          Syntax(ParamName)@433..434
            Token(TkName)@433..434 "a"
          Token(TkComma)@434..435 ","
          Token(TkWhitespace)@435..436 " "
          Syntax(ParamName)@436..437
            Token(TkName)@436..437 "b"
          Token(TkRightParen)@437..438 ")"
        Token(TkEndOfLine)@438..439 "\n"
        Token(TkWhitespace)@439..447 "        "
        Token(TkEnd)@447..450 "end"
    Token(TkEndOfLine)@450..451 "\n"
    Token(TkEndOfLine)@451..452 "\n"
    Token(TkWhitespace)@452..460 "        "
    Syntax(Comment)@460..516
      Token(TkDocStart)@460..464 "---@"
      Syntax(DocTagGeneric)@464..473
        Token(TkTagGeneric)@464..471 "generic"
        Token(TkWhitespace)@471..472 " "
        Token(TkName)@472..473 "T"
      Token(TkEndOfLine)@473..474 "\n"
      Token(TkWhitespace)@474..482 "        "
      Token(TkDocStart)@482..486 "---@"
      Syntax(DocTagParam)@486..495
        Token(TkTagParam)@486..491 "param"
        Token(TkWhitespace)@491..492 " "
        Token(TkName)@492..493 "a"
        Token(TkWhitespace)@493..494 " "
        Syntax(TypeName)@494..495
          Token(TkName)@494..495 "T"
      Token(TkEndOfLine)@495..496 "\n"
      Token(TkWhitespace)@496..504 "        "
      Token(TkDocStart)@504..508 "---@"
      Syntax(DocTagReturn)@508..516
        Token(TkTagReturn)@508..514 "return"
        Token(TkWhitespace)@514..515 " "
        Syntax(TypeName)@515..516
          Token(TkName)@515..516 "T"
    Token(TkEndOfLine)@516..517 "\n"
    Token(TkWhitespace)@517..525 "        "
    Syntax(LocalFuncStat)@525..556
      Token(TkLocal)@525..530 "local"
      Token(TkWhitespace)@530..531 " "
      Token(TkFunction)@531..539 "function"
      Token(TkWhitespace)@539..540 " "
      Syntax(LocalName)@540..541
        Token(TkName)@540..541 "f"
      Syntax(ClosureExpr)@541..556
        Syntax(ParamList)@541..544
          Token(TkLeftParen)@541..542 "("
          Syntax(ParamName)@542..543
            Token(TkName)@542..543 "a"
          Token(TkRightParen)@543..544 ")"
        Token(TkEndOfLine)@544..545 "\n"
        Token(TkWhitespace)@545..553 "        "
        Token(TkEnd)@553..556 "end"
    Token(TkEndOfLine)@556..557 "\n"
    Token(TkWhitespace)@557..565 "        "
        "#;
        assert_ast_eq!(code, result);
    }
}
