mod infer_expr_info;

use emmylua_parser::{
    LuaAstNode, LuaExpr, LuaSyntaxKind, LuaSyntaxNode, LuaSyntaxToken, LuaTokenKind,
};
use infer_expr_info::get_expr_semantic_info;

use crate::{DbIndex, LuaDeclId, LuaPropertyOwnerId, LuaType};

use super::{infer_expr, LuaInferConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticInfo {
    pub typ: LuaType,
    pub property_owner: Option<LuaPropertyOwnerId>,
}

pub fn infer_token_semantic_info(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    token: LuaSyntaxToken,
) -> Option<SemanticInfo> {
    if !matches!(token.kind().into(), LuaTokenKind::TkName) {
        return None;
    }

    let parent = token.parent()?;
    match parent.kind().into() {
        LuaSyntaxKind::ForStat
        | LuaSyntaxKind::ForRangeStat
        | LuaSyntaxKind::LocalName
        | LuaSyntaxKind::ParamName => {
            let file_id = infer_config.get_file_id();
            let decl_id = LuaDeclId::new(file_id, token.text_range().start());
            let decl = db.get_decl_index().get_decl(&decl_id)?;
            let typ = decl.get_type().cloned()?;
            Some(SemanticInfo {
                typ,
                property_owner: Some(LuaPropertyOwnerId::LuaDecl(decl_id)),
            })
        }
        _ => infer_node_semantic_info(db, infer_config, parent),
    }
}

pub fn infer_node_semantic_info(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    node: LuaSyntaxNode,
) -> Option<SemanticInfo> {
    match node {
        expr_node if LuaExpr::can_cast(expr_node.kind().into()) => {
            let expr = LuaExpr::cast(expr_node)?;
            get_expr_semantic_info(db, infer_config, expr)
        }
        _ => None,
    }
}
