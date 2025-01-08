use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaBlock, LuaClosureExpr, LuaFuncStat, LuaGeneralToken,
    LuaSyntaxToken, LuaVarExpr, VisibilityKind,
};

use crate::{DbIndex, Emmyrc, FileId, LuaMemberOwner, LuaPropertyOwnerId, LuaType};

use super::{infer_expr, LuaInferConfig};

pub fn check_visibility(
    db: &DbIndex,
    file_id: FileId,
    emmyrc: &Emmyrc,
    infer_config: &mut LuaInferConfig,
    token: LuaSyntaxToken,
    property_owner: LuaPropertyOwnerId,
) -> Option<bool> {
    let property = db
        .get_property_index()
        .get_property(property_owner.clone())?;
    if let Some(version_conds) = &property.version_conds {
        let version_number = emmyrc.runtime.version.to_lua_version_number();
        let visiable = version_conds.iter().any(|cond| cond.check(&version_number));
        if !visiable {
            return Some(false);
        }
    }

    if let Some(visibility) = property.visibility {
        match visibility {
            VisibilityKind::None |
            // this donot use
            VisibilityKind::Internal |
            VisibilityKind::Public => return Some(true),
            VisibilityKind::Protected |
            VisibilityKind::Private => {
                return Some(check_visibility_by_visibility(db, infer_config, property_owner, token, visibility).unwrap_or(false));
            },
            VisibilityKind::Package => {
                return Some(file_id == property_owner.get_file_id()?);
            },
        }
    }

    Some(true)
}

fn check_visibility_by_visibility(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    property_owner: LuaPropertyOwnerId,
    token: LuaSyntaxToken,
    visibility: VisibilityKind,
) -> Option<bool> {
    let member_owner = match property_owner {
        LuaPropertyOwnerId::Member(member_id) => {
            db.get_member_index().get_member(&member_id)?.get_owner()
        }
        _ => return Some(true),
    };

    let token = LuaGeneralToken::cast(token)?;
    let blocks = token.ancestors::<LuaBlock>();
    for block in blocks {
        if check_block_visibility(db, infer_config, &member_owner, block, visibility)
            .unwrap_or(false)
        {
            return Some(true);
        }
    }

    Some(false)
}

fn check_block_visibility(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    member_owner: &LuaMemberOwner,
    block: LuaBlock,
    visibility: VisibilityKind,
) -> Option<bool> {
    let func_stat = block
        .get_parent::<LuaClosureExpr>()?
        .get_parent::<LuaFuncStat>()?;

    let func_name = func_stat.get_func_name()?;
    if let LuaVarExpr::IndexExpr(index_expr) = func_name {
        let prefix_expr = index_expr.get_prefix_expr()?;
        let typ = infer_expr(db, infer_config, prefix_expr.into())?;
        if visibility == VisibilityKind::Protected {
            match (typ, member_owner) {
                (LuaType::Def(left), LuaMemberOwner::Type(right)) => {
                    if left == *right {
                        return Some(true);
                    }

                    // todo is subclass
                }
                _ => {}
            }
        } else if visibility == VisibilityKind::Private {
            match (typ, member_owner) {
                (LuaType::Def(left), LuaMemberOwner::Type(right)) => {
                    return Some(left == *right);
                }
                (LuaType::TableConst(left), LuaMemberOwner::Element(right)) => {
                    return Some(left == *right);
                }
                _ => {}
            }
        }
    }

    Some(false)
}
