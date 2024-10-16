use crate::{grammar::ParseResult, kind::{LuaSyntaxKind, LuaTokenKind}, parser::{LuaDocParser, MarkerEventContainer}};


pub fn parse_type(p: &mut LuaDocParser) -> ParseResult {
    // match kind {
    //     LuaDocTokenKind::Enum => parse_tag_enum(p),
    //     LuaDocTokenKind::Interface => parse_tag_interface(p),
    //     LuaDocTokenKind::Alias => parse_tag_alias(p),
    //     LuaDocTokenKind::Module => parse_tag_module(p),
    //     LuaDocTokenKind::Field => parse_tag_field(p),
    //     LuaDocTokenKind::Type => parse_tag_type(p),
    //     LuaDocTokenKind::Param => parse_tag_param(p),
    //     LuaDocTokenKind::Return => parse_tag_return(p),
    //     LuaDocTokenKind::Generic => parse_tag_generic(p),
    //     LuaDocTokenKind::See => parse_tag_see(p),
    //     LuaDocTokenKind::As => parse_tag_as(p),
    //     LuaDocTokenKind::Overload => parse_tag_overload(p),
    //     LuaDocTokenKind::Cast => parse_tag_cast(p),
    //     LuaDocTokenKind::Source => parse_tag_source(p),
    //     LuaDocTokenKind::Diagnostic => parse_tag_diagnostic(p),
    //     _ => {
    //         p.error("Expecting a type tag");
    //         p.bump();
    //         Ok(start.complete(p, LuaDocKind::Type))
    //     }
    // }
    unimplemented!()
}

pub fn parse_type_list(p: &mut LuaDocParser) -> ParseResult {
    let m = p.mark(LuaSyntaxKind::TypeList);
    parse_type(p);
    while p.current_token() == LuaTokenKind::TkComma {
        p.bump();
        parse_type(p);
    }
    Ok(m.complete(p))
}

pub fn parse_fun_type(p: &mut LuaDocParser) -> ParseResult {
    unimplemented!()
}