use crate::LuaType;

use super::{LuaMemberId, LuaMemberIndex, LuaMemberKey, LuaMemberOwner};

pub fn migrate_members(
    member_index: &mut LuaMemberIndex,
    owner: LuaMemberOwner,
    key: &LuaMemberKey,
    new_add_member_id: LuaMemberId,
) -> Option<()> {
    let member_owner = get_owner_id(member_index, new_add_member_id)?;
    let member_item = member_index.get_member_item(&owner, key)?;
    let resolve_member_id = member_item.resolve_member_id(member_index)?;
    if new_add_member_id != resolve_member_id {
        return None;
    }

    let member_ids = member_item.get_member_ids();
    for need_migrate_id in member_ids {
        if need_migrate_id == new_add_member_id {
            continue;
        }

        if let Some(need_migrate_owner) = get_owner_id(member_index, need_migrate_id) {
            if member_owner == need_migrate_owner {
                continue;
            }
            migrate_member_to(member_index, &member_owner, &need_migrate_owner);
        }
    }

    Some(())
}

fn get_owner_id(
    member_index: &mut LuaMemberIndex,
    member_id: LuaMemberId,
) -> Option<LuaMemberOwner> {
    let member = member_index.get_member(&member_id)?;
    let typ = member.get_decl_type();
    match typ {
        LuaType::Ref(type_id) => Some(LuaMemberOwner::Type(type_id)),
        LuaType::TableConst(id) => Some(LuaMemberOwner::Element(id)),
        _ => None,
    }
}

fn migrate_member_to(
    member_index: &mut LuaMemberIndex,
    owner: &LuaMemberOwner,
    need_migrate_owner: &LuaMemberOwner,
) -> Option<()> {
    let member_ids = member_index
        .get_members(need_migrate_owner)?
        .iter()
        .map(|member| member.get_id())
        .collect::<Vec<_>>();

    for member_id in member_ids {
        member_index.set_member_owner(owner.clone(), member_id.file_id, member_id);
        member_index.add_member_to_owner(owner.clone(), member_id);
    }

    Some(())
}
