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
    fn test_full_lua_syntax() {
        let code = r#"
            -- This is a comment
            local a = 10
            local b = "string"
            local c = { key = "value", 1, 2, 3 }

            function foo(x, y)
                if x > y then
                    return x
                else
                    return y
                end
            end

            for i = 1, 10 do
                print(i)
            end

            while a > 0 do
                a = a - 1
            end

            repeat
                a = a + 1
            until a == 10

            local mt = {
                __index = function(table, key)
                    return "default"
                end
            }

            setmetatable(c, mt)

            local d = c.key
            local e = c[1]
        "#;

        let result = r#"
Syntax(Chunk)@0..770
  Syntax(Block)@0..770
    Token(TkEndOfLine)@0..1 "\n"
    Token(TkWhitespace)@1..13 "            "
    Syntax(Comment)@13..33
      Token(TkNormalStart)@13..15 "--"
      Syntax(DocDescription)@15..33
        Token(TkDocDetail)@15..33 " This is a comment"
    Token(TkEndOfLine)@33..34 "\n"
    Token(TkWhitespace)@34..46 "            "
    Syntax(LocalStat)@46..58
      Token(TkLocal)@46..51 "local"
      Token(TkWhitespace)@51..52 " "
      Syntax(LocalName)@52..53
        Token(TkName)@52..53 "a"
      Token(TkWhitespace)@53..54 " "
      Token(TkAssign)@54..55 "="
      Token(TkWhitespace)@55..56 " "
      Syntax(LiteralExpr)@56..58
        Token(TkFloat)@56..58 "10"
    Token(TkEndOfLine)@58..59 "\n"
    Token(TkWhitespace)@59..71 "            "
    Syntax(LocalStat)@71..89
      Token(TkLocal)@71..76 "local"
      Token(TkWhitespace)@76..77 " "
      Syntax(LocalName)@77..78
        Token(TkName)@77..78 "b"
      Token(TkWhitespace)@78..79 " "
      Token(TkAssign)@79..80 "="
      Token(TkWhitespace)@80..81 " "
      Syntax(LiteralExpr)@81..89
        Token(TkString)@81..89 "\"string\""
    Token(TkEndOfLine)@89..90 "\n"
    Token(TkWhitespace)@90..102 "            "
    Syntax(LocalStat)@102..138
      Token(TkLocal)@102..107 "local"
      Token(TkWhitespace)@107..108 " "
      Syntax(LocalName)@108..109
        Token(TkName)@108..109 "c"
      Token(TkWhitespace)@109..110 " "
      Token(TkAssign)@110..111 "="
      Token(TkWhitespace)@111..112 " "
      Syntax(TableExpr)@112..138
        Token(TkLeftBrace)@112..113 "{"
        Token(TkWhitespace)@113..114 " "
        Syntax(TableFieldAssign)@114..127
          Token(TkName)@114..117 "key"
          Token(TkWhitespace)@117..118 " "
          Token(TkAssign)@118..119 "="
          Token(TkWhitespace)@119..120 " "
          Syntax(LiteralExpr)@120..127
            Token(TkString)@120..127 "\"value\""
        Token(TkComma)@127..128 ","
        Token(TkWhitespace)@128..129 " "
        Syntax(TableFieldValue)@129..130
          Syntax(LiteralExpr)@129..130
            Token(TkFloat)@129..130 "1"
        Token(TkComma)@130..131 ","
        Token(TkWhitespace)@131..132 " "
        Syntax(TableFieldValue)@132..133
          Syntax(LiteralExpr)@132..133
            Token(TkFloat)@132..133 "2"
        Token(TkComma)@133..134 ","
        Token(TkWhitespace)@134..135 " "
        Syntax(TableFieldValue)@135..136
          Syntax(LiteralExpr)@135..136
            Token(TkFloat)@135..136 "3"
        Token(TkWhitespace)@136..137 " "
        Token(TkRightBrace)@137..138 "}"
    Token(TkEndOfLine)@138..139 "\n"
    Token(TkEndOfLine)@139..140 "\n"
    Token(TkWhitespace)@140..152 "            "
    Syntax(FuncStat)@152..315
      Token(TkFunction)@152..160 "function"
      Token(TkWhitespace)@160..161 " "
      Syntax(NameExpr)@161..164
        Token(TkName)@161..164 "foo"
      Syntax(ClosureExpr)@164..315
        Syntax(ParamList)@164..170
          Token(TkLeftParen)@164..165 "("
          Syntax(ParamName)@165..166
            Token(TkName)@165..166 "x"
          Token(TkComma)@166..167 ","
          Token(TkWhitespace)@167..168 " "
          Syntax(ParamName)@168..169
            Token(TkName)@168..169 "y"
          Token(TkRightParen)@169..170 ")"
        Token(TkEndOfLine)@170..171 "\n"
        Token(TkWhitespace)@171..187 "                "
        Syntax(Block)@187..312
          Syntax(IfStat)@187..299
            Token(TkIf)@187..189 "if"
            Token(TkWhitespace)@189..190 " "
            Syntax(BinaryExpr)@190..195
              Syntax(NameExpr)@190..191
                Token(TkName)@190..191 "x"
              Token(TkWhitespace)@191..192 " "
              Token(TkGt)@192..193 ">"
              Token(TkWhitespace)@193..194 " "
              Syntax(NameExpr)@194..195
                Token(TkName)@194..195 "y"
            Token(TkWhitespace)@195..196 " "
            Token(TkThen)@196..200 "then"
            Token(TkEndOfLine)@200..201 "\n"
            Token(TkWhitespace)@201..221 "                    "
            Syntax(Block)@221..246
              Syntax(ReturnStat)@221..229
                Token(TkReturn)@221..227 "return"
                Token(TkWhitespace)@227..228 " "
                Syntax(NameExpr)@228..229
                  Token(TkName)@228..229 "x"
              Token(TkEndOfLine)@229..230 "\n"
              Token(TkWhitespace)@230..246 "                "
            Syntax(ElseClauseStat)@246..296
              Token(TkElse)@246..250 "else"
              Token(TkEndOfLine)@250..251 "\n"
              Token(TkWhitespace)@251..271 "                    "
              Syntax(Block)@271..296
                Syntax(ReturnStat)@271..279
                  Token(TkReturn)@271..277 "return"
                  Token(TkWhitespace)@277..278 " "
                  Syntax(NameExpr)@278..279
                    Token(TkName)@278..279 "y"
                Token(TkEndOfLine)@279..280 "\n"
                Token(TkWhitespace)@280..296 "                "
            Token(TkEnd)@296..299 "end"
          Token(TkEndOfLine)@299..300 "\n"
          Token(TkWhitespace)@300..312 "            "
        Token(TkEnd)@312..315 "end"
    Token(TkEndOfLine)@315..316 "\n"
    Token(TkEndOfLine)@316..317 "\n"
    Token(TkWhitespace)@317..329 "            "
    Syntax(ForStat)@329..386
      Token(TkFor)@329..332 "for"
      Token(TkWhitespace)@332..333 " "
      Token(TkName)@333..334 "i"
      Token(TkWhitespace)@334..335 " "
      Token(TkAssign)@335..336 "="
      Token(TkWhitespace)@336..337 " "
      Syntax(LiteralExpr)@337..338
        Token(TkFloat)@337..338 "1"
      Token(TkComma)@338..339 ","
      Token(TkWhitespace)@339..340 " "
      Syntax(LiteralExpr)@340..342
        Token(TkFloat)@340..342 "10"
      Token(TkWhitespace)@342..343 " "
      Token(TkDo)@343..345 "do"
      Token(TkEndOfLine)@345..346 "\n"
      Token(TkWhitespace)@346..362 "                "
      Syntax(Block)@362..383
        Syntax(ExprStat)@362..370
          Syntax(CallExpr)@362..370
            Syntax(NameExpr)@362..367
              Token(TkName)@362..367 "print"
            Syntax(CallArgList)@367..370
              Token(TkLeftParen)@367..368 "("
              Syntax(NameExpr)@368..369
                Token(TkName)@368..369 "i"
              Token(TkRightParen)@369..370 ")"
        Token(TkEndOfLine)@370..371 "\n"
        Token(TkWhitespace)@371..383 "            "
      Token(TkEnd)@383..386 "end"
    Token(TkEndOfLine)@386..387 "\n"
    Token(TkEndOfLine)@387..388 "\n"
    Token(TkWhitespace)@388..400 "            "
    Syntax(WhileStat)@400..456
      Token(TkWhile)@400..405 "while"
      Token(TkWhitespace)@405..406 " "
      Syntax(BinaryExpr)@406..411
        Syntax(NameExpr)@406..407
          Token(TkName)@406..407 "a"
        Token(TkWhitespace)@407..408 " "
        Token(TkGt)@408..409 ">"
        Token(TkWhitespace)@409..410 " "
        Syntax(LiteralExpr)@410..411
          Token(TkFloat)@410..411 "0"
      Token(TkWhitespace)@411..412 " "
      Token(TkDo)@412..414 "do"
      Token(TkEndOfLine)@414..415 "\n"
      Token(TkWhitespace)@415..431 "                "
      Syntax(Block)@431..453
        Syntax(AssignStat)@431..440
          Syntax(NameExpr)@431..432
            Token(TkName)@431..432 "a"
          Token(TkWhitespace)@432..433 " "
          Token(TkAssign)@433..434 "="
          Token(TkWhitespace)@434..435 " "
          Syntax(BinaryExpr)@435..440
            Syntax(NameExpr)@435..436
              Token(TkName)@435..436 "a"
            Token(TkWhitespace)@436..437 " "
            Token(TkMinus)@437..438 "-"
            Token(TkWhitespace)@438..439 " "
            Syntax(LiteralExpr)@439..440
              Token(TkFloat)@439..440 "1"
        Token(TkEndOfLine)@440..441 "\n"
        Token(TkWhitespace)@441..453 "            "
      Token(TkEnd)@453..456 "end"
    Token(TkEndOfLine)@456..457 "\n"
    Token(TkEndOfLine)@457..458 "\n"
    Token(TkWhitespace)@458..470 "            "
    Syntax(RepeatStat)@470..528
      Token(TkRepeat)@470..476 "repeat"
      Token(TkEndOfLine)@476..477 "\n"
      Token(TkWhitespace)@477..493 "                "
      Syntax(Block)@493..515
        Syntax(AssignStat)@493..502
          Syntax(NameExpr)@493..494
            Token(TkName)@493..494 "a"
          Token(TkWhitespace)@494..495 " "
          Token(TkAssign)@495..496 "="
          Token(TkWhitespace)@496..497 " "
          Syntax(BinaryExpr)@497..502
            Syntax(NameExpr)@497..498
              Token(TkName)@497..498 "a"
            Token(TkWhitespace)@498..499 " "
            Token(TkPlus)@499..500 "+"
            Token(TkWhitespace)@500..501 " "
            Syntax(LiteralExpr)@501..502
              Token(TkFloat)@501..502 "1"
        Token(TkEndOfLine)@502..503 "\n"
        Token(TkWhitespace)@503..515 "            "
      Token(TkUntil)@515..520 "until"
      Token(TkWhitespace)@520..521 " "
      Syntax(BinaryExpr)@521..528
        Syntax(NameExpr)@521..522
          Token(TkName)@521..522 "a"
        Token(TkWhitespace)@522..523 " "
        Token(TkEq)@523..525 "=="
        Token(TkWhitespace)@525..526 " "
        Syntax(LiteralExpr)@526..528
          Token(TkFloat)@526..528 "10"
    Token(TkEndOfLine)@528..529 "\n"
    Token(TkEndOfLine)@529..530 "\n"
    Token(TkWhitespace)@530..542 "            "
    Syntax(LocalStat)@542..672
      Token(TkLocal)@542..547 "local"
      Token(TkWhitespace)@547..548 " "
      Syntax(LocalName)@548..550
        Token(TkName)@548..550 "mt"
      Token(TkWhitespace)@550..551 " "
      Token(TkAssign)@551..552 "="
      Token(TkWhitespace)@552..553 " "
      Syntax(TableExpr)@553..672
        Token(TkLeftBrace)@553..554 "{"
        Token(TkEndOfLine)@554..555 "\n"
        Token(TkWhitespace)@555..571 "                "
        Syntax(TableFieldAssign)@571..658
          Token(TkName)@571..578 "__index"
          Token(TkWhitespace)@578..579 " "
          Token(TkAssign)@579..580 "="
          Token(TkWhitespace)@580..581 " "
          Syntax(ClosureExpr)@581..658
            Token(TkFunction)@581..589 "function"
            Syntax(ParamList)@589..601
              Token(TkLeftParen)@589..590 "("
              Syntax(ParamName)@590..595
                Token(TkName)@590..595 "table"
              Token(TkComma)@595..596 ","
              Token(TkWhitespace)@596..597 " "
              Syntax(ParamName)@597..600
                Token(TkName)@597..600 "key"
              Token(TkRightParen)@600..601 ")"
            Token(TkEndOfLine)@601..602 "\n"
            Token(TkWhitespace)@602..622 "                    "
            Syntax(Block)@622..655
              Syntax(ReturnStat)@622..638
                Token(TkReturn)@622..628 "return"
                Token(TkWhitespace)@628..629 " "
                Syntax(LiteralExpr)@629..638
                  Token(TkString)@629..638 "\"default\""
              Token(TkEndOfLine)@638..639 "\n"
              Token(TkWhitespace)@639..655 "                "
            Token(TkEnd)@655..658 "end"
        Token(TkEndOfLine)@658..659 "\n"
        Token(TkWhitespace)@659..671 "            "
        Token(TkRightBrace)@671..672 "}"
    Token(TkEndOfLine)@672..673 "\n"
    Token(TkEndOfLine)@673..674 "\n"
    Token(TkWhitespace)@674..686 "            "
    Syntax(ExprStat)@686..705
      Syntax(CallExpr)@686..705
        Syntax(NameExpr)@686..698
          Token(TkName)@686..698 "setmetatable"
        Syntax(CallArgList)@698..705
          Token(TkLeftParen)@698..699 "("
          Syntax(NameExpr)@699..700
            Token(TkName)@699..700 "c"
          Token(TkComma)@700..701 ","
          Token(TkWhitespace)@701..702 " "
          Syntax(NameExpr)@702..704
            Token(TkName)@702..704 "mt"
          Token(TkRightParen)@704..705 ")"
    Token(TkEndOfLine)@705..706 "\n"
    Token(TkEndOfLine)@706..707 "\n"
    Token(TkWhitespace)@707..719 "            "
    Syntax(LocalStat)@719..734
      Token(TkLocal)@719..724 "local"
      Token(TkWhitespace)@724..725 " "
      Syntax(LocalName)@725..726
        Token(TkName)@725..726 "d"
      Token(TkWhitespace)@726..727 " "
      Token(TkAssign)@727..728 "="
      Token(TkWhitespace)@728..729 " "
      Syntax(IndexExpr)@729..734
        Syntax(NameExpr)@729..730
          Token(TkName)@729..730 "c"
        Token(TkDot)@730..731 "."
        Token(TkName)@731..734 "key"
    Token(TkEndOfLine)@734..735 "\n"
    Token(TkWhitespace)@735..747 "            "
    Syntax(LocalStat)@747..761
      Token(TkLocal)@747..752 "local"
      Token(TkWhitespace)@752..753 " "
      Syntax(LocalName)@753..754
        Token(TkName)@753..754 "e"
      Token(TkWhitespace)@754..755 " "
      Token(TkAssign)@755..756 "="
      Token(TkWhitespace)@756..757 " "
      Syntax(IndexExpr)@757..761
        Syntax(NameExpr)@757..758
          Token(TkName)@757..758 "c"
        Token(TkLeftBracket)@758..759 "["
        Syntax(LiteralExpr)@759..760
          Token(TkFloat)@759..760 "1"
        Token(TkRightBracket)@760..761 "]"
    Token(TkEndOfLine)@761..762 "\n"
    Token(TkWhitespace)@762..770 "        "
"#;
        assert_ast_eq!(code, result);
    }

    #[test]
    fn test_expr() {
        let code = r#"
        local a = 1 + 2 + 3 + 4
        "#;

        let result = r#"
Syntax(Chunk)@0..41
  Syntax(Block)@0..41
    Token(TkEndOfLine)@0..1 "\n"
    Token(TkWhitespace)@1..9 "        "
    Syntax(LocalStat)@9..32
      Token(TkLocal)@9..14 "local"
      Token(TkWhitespace)@14..15 " "
      Syntax(LocalName)@15..16
        Token(TkName)@15..16 "a"
      Token(TkWhitespace)@16..17 " "
      Token(TkAssign)@17..18 "="
      Token(TkWhitespace)@18..19 " "
      Syntax(BinaryExpr)@19..32
        Syntax(BinaryExpr)@19..28
          Syntax(BinaryExpr)@19..24
            Syntax(LiteralExpr)@19..20
              Token(TkInt)@19..20 "1"
            Token(TkWhitespace)@20..21 " "
            Token(TkPlus)@21..22 "+"
            Token(TkWhitespace)@22..23 " "
            Syntax(LiteralExpr)@23..24
              Token(TkInt)@23..24 "2"
          Token(TkWhitespace)@24..25 " "
          Token(TkPlus)@25..26 "+"
          Token(TkWhitespace)@26..27 " "
          Syntax(LiteralExpr)@27..28
            Token(TkInt)@27..28 "3"
        Token(TkWhitespace)@28..29 " "
        Token(TkPlus)@29..30 "+"
        Token(TkWhitespace)@30..31 " "
        Syntax(LiteralExpr)@31..32
          Token(TkInt)@31..32 "4"
    Token(TkEndOfLine)@32..33 "\n"
    Token(TkWhitespace)@33..41 "        "
        "#;

        assert_ast_eq!(code, result);
    }
}
