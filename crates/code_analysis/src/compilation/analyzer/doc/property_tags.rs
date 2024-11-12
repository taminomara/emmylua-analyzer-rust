use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaDocDescriptionOwner, LuaDocTagDeprecated, LuaDocTagSource, LuaDocTagVisibility, LuaLocalName, LuaVarExpr, VisibilityKind
};

use crate::
    db_index::{LuaPropertyOwnerId, LuaSignatureId}
;

use super::{tags::find_owner_closure, DocAnalyzer};

pub fn analyze_visibility(
    analyzer: &mut DocAnalyzer,
    visibility: LuaDocTagVisibility,
) -> Option<()> {
    let visibility_kind = visibility.get_visibility_token()?.get_visibility();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_visibility(analyzer.file_id, owner_id, visibility_kind);

    Some(())
}

pub fn analyze_source(analyzer: &mut DocAnalyzer, source: LuaDocTagSource) -> Option<()> {
    let source = source.get_path_token()?.get_path().to_string();
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_source(analyzer.file_id, owner_id, source);

    Some(())
}

pub fn analyze_nodiscard(analyzer: &mut DocAnalyzer) -> Option<()> {
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_nodiscard(analyzer.file_id, owner_id);

    Some(())
}

pub fn analyze_deprecated(analyzer: &mut DocAnalyzer, tag: LuaDocTagDeprecated) -> Option<()> {
    let message = if let Some(desc) = tag.get_description() {
        Some(desc.get_description_text().to_string())
    } else {
        None
    };
    let owner_id = get_owner_id(analyzer)?;

    analyzer
        .db
        .get_property_index()
        .add_deprecated(analyzer.file_id, owner_id, message);

    Some(())
}

fn get_owner_id(analyzer: &mut DocAnalyzer) -> Option<LuaPropertyOwnerId> {
    let owner = analyzer.comment.get_owner()?;
    match owner {
        LuaAst::LuaLocalFuncStat(_) | LuaAst::LuaFuncStat(_) => {
            let closure = find_owner_closure(analyzer)?;
            Some(LuaPropertyOwnerId::Signature(LuaSignatureId::new(
                analyzer.file_id,
                &closure,
            )))
        },
        LuaAst::LuaAssignStat(assign) => {
            let first_var = assign.child::<LuaVarExpr>()?;
            match first_var {
                LuaVarExpr::NameExpr(name_expr) => {
                    let name = name_expr.get_name_text()?;
                    let decl = analyzer
                        .db
                        .get_decl_index()
                        .get_decl_tree(&analyzer.file_id)?
                        .find_local_decl(&name, name_expr.get_position())?;
                    return Some(LuaPropertyOwnerId::LuaDecl(decl.get_id()));
                }
                _ => None,
            }
        },
        LuaAst::LuaLocalStat(local_stat) => {
            let local_name = local_stat.child::<LuaLocalName>()?;
            let name_token = local_name.get_name_token()?;
            let name = name_token.get_name_text();
            let decl = analyzer
                .db
                .get_decl_index()
                .get_decl_tree(&analyzer.file_id)?
                .find_local_decl(&name, name_token.get_position())?;
            return Some(LuaPropertyOwnerId::LuaDecl(decl.get_id()));
        }
        _ => None,
    }
}
