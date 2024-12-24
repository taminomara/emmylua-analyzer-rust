mod infer_expr_info;

use emmylua_parser::{
    LuaAstNode, LuaDocNameType, LuaDocTag, LuaExpr, LuaSyntaxKind, LuaSyntaxNode, LuaSyntaxToken,
    LuaTableField,
};
use infer_expr_info::infer_expr_semantic_info;

use crate::{DbIndex, LuaDeclId, LuaMemberId, LuaPropertyOwnerId, LuaType};

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
    let parent = token.parent()?;
    match parent.kind().into() {
        LuaSyntaxKind::ForStat
        | LuaSyntaxKind::ForRangeStat
        | LuaSyntaxKind::LocalName
        | LuaSyntaxKind::ParamName => {
            let file_id = infer_config.get_file_id();
            let decl_id = LuaDeclId::new(file_id, token.text_range().start());
            let decl = db.get_decl_index().get_decl(&decl_id)?;
            let typ = decl.get_type().cloned().unwrap_or(LuaType::Unknown);
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
            infer_expr_semantic_info(db, infer_config, expr)
        }
        table_field_node if LuaTableField::can_cast(table_field_node.kind().into()) => {
            let table_field = LuaTableField::cast(table_field_node)?;
            let member_id =
                LuaMemberId::new(table_field.get_syntax_id(), infer_config.get_file_id());
            let member = db.get_member_index().get_member(&member_id)?;
            let typ = member.get_decl_type().clone();
            Some(SemanticInfo {
                typ,
                property_owner: Some(LuaPropertyOwnerId::Member(member_id)),
            })
        }
        name_type if LuaDocNameType::can_cast(name_type.kind().into()) => {
            let name_type = LuaDocNameType::cast(name_type)?;
            let name = name_type.get_name_text()?;
            let type_decl = db
                .get_type_index()
                .find_type_decl(infer_config.get_file_id(), &name)?;
            Some(SemanticInfo {
                typ: LuaType::Ref(type_decl.get_id()),
                property_owner: LuaPropertyOwnerId::TypeDecl(type_decl.get_id()).into(),
            })
        }
        tags if LuaDocTag::can_cast(tags.kind().into()) => {
            let tag = LuaDocTag::cast(tags)?;
            let name = match tag {
                LuaDocTag::Alias(alias) => alias.get_name_token()?.get_name_text().to_string(),
                LuaDocTag::Class(class) => class.get_name_token()?.get_name_text().to_string(),
                LuaDocTag::Enum(enum_) => enum_.get_name_token()?.get_name_text().to_string(),
                _ => return None,
            };

            let type_decl = db
                .get_type_index()
                .find_type_decl(infer_config.get_file_id(), &name)?;
            Some(SemanticInfo {
                typ: LuaType::Ref(type_decl.get_id()),
                property_owner: LuaPropertyOwnerId::TypeDecl(type_decl.get_id()).into(),
            })
        }
        _ => None,
    }
}
