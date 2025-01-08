use emmylua_parser::{LuaSyntaxToken, VisibilityKind};

use crate::{DbIndex, Emmyrc, FileId, LuaPropertyOwnerId};

pub fn check_visibility(
    db: &DbIndex,
    file_id: FileId,
    emmyrc: &Emmyrc,
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
            VisibilityKind::Protected => todo!(),
            VisibilityKind::Private => todo!(),
            VisibilityKind::Package => {
                return Some(file_id == property_owner.get_file_id()?);
            },
        }

    }
 
    Some(false)
}

